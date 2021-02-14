use crate::yahoo_finance::model::chart::chart_response::ChartResponse;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename="chart")]
    chart: Option<ChartResponse>,
}