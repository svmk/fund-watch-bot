use crate::market::common::model::historical_candlestick::HistoricalCandleStick;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::repository::model::entity::Entity;
#[derive(new, Debug, Serialize, Deserialize)]
pub struct DayPrice {
    #[serde(rename="id")]
    id: DayPriceId,
    #[serde(rename="candlestick")]
    candlestick: HistoricalCandleStick,
}

impl DayPrice {
    pub fn is_candlestick_equals(&self, candlestick: &HistoricalCandleStick) -> bool {
        return &self.candlestick == candlestick;
    }

    pub fn get_candlestick(&self) -> &HistoricalCandleStick {
        return &self.candlestick;
    }
}

impl Entity<DayPriceId> for DayPrice {
    fn get_entity_id(&self) -> &DayPriceId {
        return &self.id;
    }
}