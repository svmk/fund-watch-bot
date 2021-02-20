use crate::market::common::model::cik::Cik;
use crate::prelude::*;
use crate::repository::model::identity::Identity;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "FundId::from_cik")]
pub struct FundId(Cik);

impl FundId {
    pub fn from_cik(value: Cik) -> Result<FundId, Failure> {
        return Ok(FundId(value));
    }
}

impl Identity for FundId {
    fn to_string(&self) -> String {
        return format!("{}", self.0);
    }
}