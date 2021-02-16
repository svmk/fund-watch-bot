use crate::sec_gov::model::company_name::CompanyName;
use crate::sec_gov::model::cusip::Cusip;
use crate::sec_gov::model::investment_discretion::InvestmentDiscretion;
use crate::market::common::model::share::Share;

#[derive(new, Debug)]
pub struct Form13FComponent {
    company_name: CompanyName,
    cusip: Cusip,
    investment_discretion: InvestmentDiscretion,
    share: Share,
}

// 77ea5ae2-43ba-4fec-a181-b452cd3d148f