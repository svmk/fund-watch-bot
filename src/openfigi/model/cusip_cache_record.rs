use crate::market::model::cusip::Cusip;
use crate::openfigi::model::figi_record::FigiRecord;
use crate::repository::model::entity::Entity;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct CusipCacheRecord {
    #[serde(rename="cusip")]
    cusip: Cusip,
    #[serde(rename="record")]
    record: FigiRecord,
}

impl Entity<Cusip> for CusipCacheRecord {
    fn get_entity_id(&self) -> &Cusip {
        return &self.cusip;
    }
}

impl CusipCacheRecord {
    pub fn get_record(&self) -> &FigiRecord {
        return &self.record;
    }
}