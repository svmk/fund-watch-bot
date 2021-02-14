use crate::app::model::datetime::DateTime;
use std::num::NonZeroU32;

#[derive(new, Debug)]
pub struct SplitRule {
    started_at: Option<DateTime>,
    ended_at: Option<DateTime>,
    nominator: NonZeroU32,
    denominator: NonZeroU32,
}