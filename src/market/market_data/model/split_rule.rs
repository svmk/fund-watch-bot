use crate::app::model::datetime::DateTime;
use std::num::NonZeroU32;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct SplitRule {
    #[serde(rename = "started_at")]
    started_at: Option<DateTime>,
    #[serde(rename = "ended_at")]
    ended_at: Option<DateTime>,
    #[serde(rename = "nominator")]
    nominator: NonZeroU32,
    #[serde(rename = "denominator")]
    denominator: NonZeroU32,
}

impl SplitRule {
    pub fn is_match_datetime(&self, datetime: &DateTime) -> bool {
        // started_at <= date < ended_at
        if let Some(ref started_at) = self.started_at {
            if datetime < started_at {
                return false;
            }   
        }
        if let Some(ref ended_at) = self.ended_at {
            if datetime >= ended_at {
                return false;
            }
        }
        return true;
    }

    pub fn calculate_from_actual_to_original(&self, value: f64) -> f64 {
        let value = value / self.nominator.get() as f64;
        let value = value * self.denominator.get() as f64;
        return value;
    }

    pub fn calculate_from_original_to_actual(&self, value: f64) -> f64 {
        let value = value / self.denominator.get() as f64;
        let value = value * self.nominator.get() as f64;
        return value;
    }
}