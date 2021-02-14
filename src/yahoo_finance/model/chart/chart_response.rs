use crate::yahoo_finance::model::chart::chart_split::ChartSplit;
use crate::yahoo_finance::model::chart::chart_dividiend::ChartDividend;
use crate::app::model::timestamp::TimeStamp;
use crate::market::model::actual_price::ActualPrice;
use crate::market::model::volume::Volume;
use std::collections::HashMap;

#[derive(Debug, serde_query::Deserialize)]
pub struct ChartResponse {
    #[query(".splits")]
    splits: HashMap<String, ChartSplit>,
    #[query(".dividends")]
    dividends: HashMap<String, ChartDividend>,
    #[query(".timestamps")]
    timestamps: Vec<TimeStamp>,
    #[query(".open")]
    open: Vec<ActualPrice>,
    #[query(".close")]
    close: Vec<ActualPrice>,
    #[query(".high")]
    high: Vec<ActualPrice>,
    #[query(".low")]
    low: Vec<ActualPrice>,
    #[query(".volume")]
    volume: Vec<Volume>,
}