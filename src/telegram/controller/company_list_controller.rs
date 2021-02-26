use crate::telegram::controller::prelude::*;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::query::all_query::AllQuery;
use futures::stream::{StreamExt};
use typed_di::service::Service;

#[derive(new)]
pub struct CompanyListController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
}


#[async_trait]
impl CommandHandler for CompanyListController {
    async fn handle_message(&self, message: IncomingMessage) -> Result<View, Failure> {
        // let mut funds = self.fund_repository.query(AllQuery::new()).await?;
        // let id: String = funds.next().await;
        // let funds_records: Vec<_> = Vec::new();
        // while let Some(fund) = funds.next().await? {
        //     funds_records.push(fund);
        // }

        unimplemented!()
    }
}