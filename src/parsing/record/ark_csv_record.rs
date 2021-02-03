#[derive(Serialize, Deserialize)]
pub struct ArkCsvRecord {
    #[serde(rename="date")]
    pub date: String,
    #[serde(rename="fund")]
    pub fund: String,
    #[serde(rename="company")]
    pub company: String,
    #[serde(rename="ticker")]
    pub ticker: String,
    #[serde(rename="cusip")]
    pub cusip: String,
    #[serde(rename="shares")]
    pub shares: String,
    #[serde(rename="market_value")]
    pub market_value: String,
    #[serde(rename="weight")]
    pub weight: String,
}

impl ArkCsvRecord {
    pub fn is_empty(&self) -> bool {
        if self.date.is_empty() {
            return true;
        }
        if self.fund.is_empty() {
            return true;
        }
        if self.company.is_empty() {
            return true;
        }
        if self.ticker.is_empty() {
            return true;
        }
        if self.cusip.is_empty() {
            return true;
        }
        if self.shares.is_empty() {
            return true;
        }
        if self.market_value.is_empty() {
            return true;
        }
        if self.weight.is_empty() {
            return true;
        }
        return false;
    }
}