use crate::yahoo_finance::model::chart::chart_response::ChartResponse;
use crate::yahoo_finance::model::common_api::result::ResponseResult;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename="chart")]
    chart: Option<ResponseResult<Vec<ChartResponse>>>,
}

impl Response {
    pub fn get_charts(&self) -> Result<&ChartResponse, Failure> {
        let chart = match &self.chart {
            Some(chart) => {
                chart.get_result()?
            },
            None => {
                return crate::fail!("Unable to get chart from response");
            },
        };
        let chart = match chart.first() {
            Some(chart) => {chart},
            None => {
                return crate::fail!("Unable to get chart from response: chart field is empty");
            },
        };
        return Ok(chart);
    }
}