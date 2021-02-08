use crate::fetching::model::url::Url;
use crate::sec_gov::model::year_quartal::YearQuartal;

#[derive(new, Debug)]
pub struct CompanyIndexRequest(YearQuartal);

impl CompanyIndexRequest {
    pub fn create_url(&self) -> Url {
        unimplemented!()
        // let url = format!(
        //     "/{}/{}/company.idx", 
        //     self.0.get_year(), 
        //     self.0.get_quartal(),
        // );
        // return Url::parse(url).unwrap();
    }
}