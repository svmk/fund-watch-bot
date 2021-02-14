use crate::app::model::datetime::DateTime;

#[derive(new, Debug)]
pub struct SplitRule {
    started_at: DateTime,
    ended_at: DateTime,
    nominator: u32,
    denominator: u32,
}