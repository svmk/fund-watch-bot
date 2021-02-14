use crate::app::model::datetime::DateTime;
use std::num::NonZeroU32;

#[derive(Debug)]
pub struct Split {
    nominator: NonZeroU32,
    denominator: NonZeroU32,
    date: DateTime,
}

impl Split {
    pub fn get_datetime(&self) -> &DateTime {
        return &self.date;
    }

    pub fn get_nominator(&self) -> NonZeroU32 {
        return self.nominator;
    }
    
    pub fn get_denominator(&self) -> NonZeroU32 {
        return self.denominator;
    }
}