use crate::app::model::datetime::DateTime;

#[derive(Debug)]
pub struct Split {
    nominator: u32,
    denominator: u32,
    date: DateTime,
}

impl Split {
    pub fn get_datetime(&self) -> &DateTime {
        return &self.date;
    }
}