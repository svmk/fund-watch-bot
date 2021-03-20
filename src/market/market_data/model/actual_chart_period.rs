use crate::{app::model::datetime::DateTime, market::market_data::model::chart_period::ChartPeriod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualChartPeriod {
    #[serde(rename = "period")]
    period: Option<ChartPeriod>,
}

impl ActualChartPeriod {
    pub fn new_uncached() -> ActualChartPeriod {
        return ActualChartPeriod {
            period: None,
        }
    }

    pub fn new(chart_period: ChartPeriod) -> ActualChartPeriod {
        return ActualChartPeriod {
            period: Some(chart_period),
        }
    }

    pub fn get_period(&self) -> Option<&ChartPeriod> {
        return self.period.as_ref();
    }

    pub fn is_actual(&self, expected_period: &ChartPeriod) -> bool {
        match self.period {
            Some(ref period) => {
                return period.contains(expected_period);
            },
            None => {
                return false;
            },
        }
    }

    pub fn is_actual_datetime(&self, datetime: &DateTime) -> bool {
        match self.period {
            Some(ref period) => {
                return period.contains_datetime(datetime);
            },
            None => {
                return false;
            },
        }
    }

    pub fn update_chart_period(&mut self, chart_period: ChartPeriod) {
        self.period = Some(chart_period);
    }
}