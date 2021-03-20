use crate::market::common::model::cusip::Cusip;
use crate::market::common::model::ticker::Ticker;
use crate::openfigi::model::figi_record::FigiRecord;
use crate::repository::model::entity::Entity;
use crate::prelude::*;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct CusipCacheRecord {
    #[serde(rename="cusip")]
    cusip: Cusip,
    #[serde(rename="records")]
    records: Vec<FigiRecord>,
}

impl Entity<Cusip> for CusipCacheRecord {
    fn get_entity_id(&self) -> &Cusip {
        return &self.cusip;
    }
}

impl CusipCacheRecord {
    pub fn get_first_record(&self) -> Result<&FigiRecord, Failure> {
        if let Some(record) = self.records.first() {
            return Ok(record);
        }
        return crate::fail!("Unable to get figi record for cusip `{}`", self.cusip);
    }

    pub fn find_ticker(&self) -> Option<Ticker> {
        for record in self.records.iter() {
            if let Some(ticker) = record.get_opt_ticker() {
                return Some(ticker);
            }
        }
        return None;
    }
}