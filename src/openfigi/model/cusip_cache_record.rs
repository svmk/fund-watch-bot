use crate::market::common::model::cusip::Cusip;
use crate::market::common::model::company_id::CompanyId;
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

impl Entity for CusipCacheRecord {
    type Id = Cusip;
    fn get_entity_id(&self) -> &Cusip {
        return &self.cusip;
    }
}

impl CusipCacheRecord {
    fn find_ticker(&self) -> Option<Ticker> {
        for record in self.records.iter() {
            if let Some(ticker) = record.get_opt_ticker() {
                return Some(ticker);
            }
        }
        return None;
    }

    pub fn create_company_id(&self) -> CompanyId {
        let mut company_id = CompanyId::new(self.cusip.clone());
        if let Some(ticker) = self.find_ticker() {
            company_id = company_id.with_ticker(ticker);
        }
        return company_id;
    }
}