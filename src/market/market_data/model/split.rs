use crate::app::model::datetime::DateTime;
use std::num::NonZeroU32;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct Split {
    #[serde(rename = "date")]
    date: DateTime,
    #[serde(rename = "nominator")]
    nominator: NonZeroU32,
    #[serde(rename = "denominator")]
    denominator: NonZeroU32,
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