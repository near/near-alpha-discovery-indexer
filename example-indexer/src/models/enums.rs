use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, Clone, Copy)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[DieselType = "Execution_outcome_status"]
#[PgType = "execution_outcome_status"]
pub enum ExecutionOutcomeStatus {
    Failure,
    Success,
}
