mod models;
mod retriable;
mod schema;

#[macro_use]
extern crate diesel;

use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::Database;
use diesel::PgConnection;
use dotenv::dotenv;
use futures::StreamExt;
use near_lake_framework::{
    near_indexer_primitives::views::{
        ActionView, ExecutionOutcomeView, ExecutionStatusView, ReceiptEnumView, ReceiptView,
    },
    LakeConfig, LakeConfigBuilder,
};
use std::collections::HashSet;
use std::env;
use std::str::FromStr;
use tracing_subscriber::EnvFilter;

use crate::models::enums::ExecutionOutcomeStatus;
use crate::models::events::Event;
use crate::models::social::Receipt;

fn get_database_credentials() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}

fn establish_connection() -> actix_diesel::Database<PgConnection> {
    let database_url = get_database_credentials();
    actix_diesel::Database::builder()
        .pool_max_size(30)
        .open(&database_url)
}

fn lake_config() -> anyhow::Result<LakeConfig> {
    let chain_id = env::var("CHAIN_ID").expect("CHAIN_ID must be set in .env file");
    let start_block_height: u64 = u64::from_str(
        env::var("START_BLOCK_HEIGHT")
            .expect("START_BLOCK_HEIGHT must be set in .env file")
            .as_str(),
    )
    .expect("Failed to parse START_BLOCK_HEIGHT as u64");
    let mut config = LakeConfigBuilder::default().start_block_height(start_block_height);

    match chain_id.as_str() {
        "testnet" => {
            config = config.testnet();
        }
        "mainnet" => {
            config = config.mainnet();
        }
        _ => {
            panic!(
                "Unsupported CHAIN_ID provided: {}. Only `testnet` and `mainnet` are supported",
                &chain_id
            )
        }
    };

    Ok(config.build()?)
}

const SCAM_PROJECT: &str = "scam_project";
const INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);
const MAX_DELAY_TIME: std::time::Duration = std::time::Duration::from_secs(120);

#[actix::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();

    // let whitelisted_accounts = HashSet::from(["social.near".to_string()]);
    let whitelisted_accounts = env::var("WHITELIST_ACCOUNTS")
        .unwrap_or_default()
        .split(',')
        .into_iter()
        .map(|account| account.trim().to_string())
        .collect::<HashSet<String>>();

    let mut env_filter = EnvFilter::new(
        "tokio_reactor=info,near=info,stats=info,telemetry=info,indexer=info,aggregated=info",
    );

    env_filter = env_filter.add_directive(
        "scam_project=info"
            .parse()
            .expect("Failed to parse directive"),
    );

    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        if !rust_log.is_empty() {
            for directive in rust_log.split(',').filter_map(|s| match s.parse() {
                Ok(directive) => Some(directive),
                Err(err) => {
                    eprintln!("Ignoring directive `{}`: {}", s, err);
                    None
                }
            }) {
                env_filter = env_filter.add_directive(directive);
            }
        }
    }

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();

    let pool = establish_connection();

    // create a NEAR Lake Framework config
    let config = lake_config()?;

    // instantiate the NEAR Lake Framework Stream
    let (sender, stream) = near_lake_framework::streamer(config);

    tracing::info!(
        "Starting the indexer with whitelisted accounts:\n{:?}",
        &whitelisted_accounts
    );

    // read the stream events and pass them to a handler function with
    // concurrency 1
    let mut handlers = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| extract_info(&pool, streamer_message, &whitelisted_accounts))
        .buffer_unordered(1usize);

    while let Some(_handle_message) = handlers.next().await {}
    drop(handlers); // close the channel so the sender will stop

    // propagate errors from the sender
    match sender.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(anyhow::Error::from(e)), // JoinError
    }
}

const EVENT_LOG_PREFIX: &str = "EVENT_JSON:";

async fn extract_info(
    pool: &Database<PgConnection>,
    msg: near_lake_framework::near_indexer_primitives::StreamerMessage,
    whitelisted_accounts: &HashSet<String>,
) -> anyhow::Result<()> {
    let block_height = msg.block.header.height;
    let block_hash = msg.block.header.hash.to_string();
    let block_timestamp = msg.block.header.timestamp_nanosec;
    let block_epoch_id = msg.block.header.epoch_id.to_string();

    let mut events = vec![];
    let mut receipts = vec![];
    for shard in msg.shards {
        for (outcome_index, outcome) in shard.receipt_execution_outcomes.into_iter().enumerate() {
            let ReceiptView {
                predecessor_id,
                receiver_id: account_id,
                receipt_id,
                receipt,
            } = outcome.receipt;
            let predecessor_id = predecessor_id.to_string();
            let account_id = account_id.to_string();
            let receipt_id = receipt_id.to_string();
            let ExecutionOutcomeView { logs, status, .. } = outcome.execution_outcome.outcome;
            let status = match status {
                ExecutionStatusView::Unknown => ExecutionOutcomeStatus::Failure,
                ExecutionStatusView::Failure(_) => ExecutionOutcomeStatus::Failure,
                ExecutionStatusView::SuccessValue(_) => ExecutionOutcomeStatus::Success,
                ExecutionStatusView::SuccessReceiptId(_) => ExecutionOutcomeStatus::Success,
            };
            for (log_index, log) in logs.into_iter().enumerate() {
                if log.starts_with(EVENT_LOG_PREFIX) {
                    events.push(Event {
                        block_height: block_height.into(),
                        block_hash: block_hash.clone(),
                        block_timestamp: block_timestamp.into(),
                        block_epoch_id: block_epoch_id.clone(),
                        receipt_id: receipt_id.clone(),
                        log_index: log_index as i32,
                        predecessor_id: predecessor_id.clone(),
                        account_id: account_id.clone(),
                        status,
                        event: log.as_str()[EVENT_LOG_PREFIX.len()..].to_string(),
                    })
                }
            }
            if whitelisted_accounts.contains(&account_id) {
                match receipt {
                    ReceiptEnumView::Action {
                        signer_id,
                        signer_public_key,
                        actions,
                        ..
                    } => {
                        for (index_in_receipt, action) in actions.into_iter().enumerate() {
                            match action {
                                ActionView::FunctionCall {
                                    method_name,
                                    args,
                                    gas,
                                    deposit,
                                } => {
                                    if let Ok(args) = String::from_utf8(args) {
                                        receipts.push(Receipt {
                                            block_height: block_height.into(),
                                            block_hash: block_hash.clone(),
                                            block_timestamp: block_timestamp.into(),
                                            block_epoch_id: block_epoch_id.clone(),
                                            outcome_index: outcome_index as i32,
                                            receipt_id: receipt_id.clone(),
                                            index_in_receipt: index_in_receipt as i32,
                                            signer_public_key: signer_public_key.to_string(),
                                            signer_id: signer_id.to_string(),
                                            predecessor_id: predecessor_id.clone(),
                                            account_id: account_id.clone(),
                                            status,
                                            deposit: bigdecimal::BigDecimal::from_str(
                                                deposit.to_string().as_str(),
                                            )
                                            .unwrap(),
                                            gas: gas.into(),
                                            method_name,
                                            args,
                                        });
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    ReceiptEnumView::Data { .. } => {}
                }
            }
        }
    }

    tracing::info!(
        "Block #{}: EVENTS: {} | RECEIPTS: {}",
        msg.block.header.height,
        &events.len(),
        &receipts.len(),
    );

    crate::await_retry_or_panic!(
        diesel::insert_into(schema::events::table)
            .values(events.clone())
            .on_conflict_do_nothing()
            .execute_async(pool),
        10,
        "Events insert foilureee".to_string(),
        &events
    );

    crate::await_retry_or_panic!(
        diesel::insert_into(schema::receipts::table)
            .values(receipts.clone())
            .on_conflict_do_nothing()
            .execute_async(pool),
        10,
        "Receipts insert foilureee".to_string(),
        &receipts
    );

    Ok(())
}
