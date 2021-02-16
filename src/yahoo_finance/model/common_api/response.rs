use crate::yahoo_finance::model::chart::chart_response::ChartResponse;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename="chart")]
    chart: Option<ChartResponse>,
}

impl Response {
    pub fn get_charts(&self) -> Result<&ChartResponse, Failure> {
        if let Some(ref chart) = self.chart {
            return Ok(chart);
        }
        return Err(Failure::msg("Unable to get chart from response"));
    }
}