use crate::yahoo_finance::model::chart::chart_response::ChartResponse;
use crate::yahoo_finance::model::common_api::result::ResponseResult;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename="chart")]
    chart: Option<ResponseResult<ChartResponse>>,
}

impl Response {
    pub fn get_charts(&self) -> Result<&ChartResponse, Failure> {
        let chart = match &self.chart {
            Some(ResponseResult::Ok {result}) => result,
            Some(ResponseResult::Error {error}) => {
                return crate::fail!("Unable to get chart from response: {}", error);
            },
            None => {
                return crate::fail!("Unable to get chart from response");
            },
        };
        return Ok(chart);
    }
}