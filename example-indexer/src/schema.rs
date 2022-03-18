table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    events (receipt_id, log_index) {
        block_height -> Numeric,
        block_hash -> Text,
        block_timestamp -> Numeric,
        block_epoch_id -> Text,
        receipt_id -> Text,
        log_index -> Int4,
        predecessor_id -> Text,
        account_id -> Text,
        status -> Execution_outcome_status,
        event -> Text,
    }
}
