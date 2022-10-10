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

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    receipts (receipt_id, index_in_receipt) {
        block_height -> Numeric,
        block_hash -> Text,
        block_timestamp -> Numeric,
        block_epoch_id -> Text,
        outcome_index -> Int4,
        receipt_id -> Text,
        index_in_receipt -> Int4,
        signer_public_key -> Text,
        signer_id -> Text,
        predecessor_id -> Text,
        account_id -> Text,
        status -> Execution_outcome_status,
        deposit -> Numeric,
        gas -> Numeric,
        method_name -> Text,
        args -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    receipts,
);
