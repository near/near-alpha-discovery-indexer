use crate::ExecutionOutcomeStatus;
use bigdecimal::BigDecimal;

use crate::schema;
use schema::events;

#[derive(Insertable, Clone, Debug)]
pub struct Event {
    pub block_height: BigDecimal,
    pub block_hash: String,
    pub block_timestamp: BigDecimal,
    pub block_epoch_id: String,
    pub receipt_id: String,
    pub log_index: i32,
    pub predecessor_id: String,
    pub account_id: String,
    pub status: ExecutionOutcomeStatus,
    pub event: String,
}
