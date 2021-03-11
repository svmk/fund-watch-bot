use crate::app::model::date::Date;

#[derive(Debug, StructOpt)]
pub struct Import13FFormCommand {
    started_at: Option<Date>,
    ended_at: Option<Date>,
}

impl Import13FFormCommand {
    pub fn get_started_at(&self) -> Date {
        return self.started_at.clone().unwrap_or_else(|| {
            return Date::from_ymd(1993, 1, 1);
        });
    }
    
    pub fn get_ended_at(&self) -> Option<Date> {
        return self.ended_at.clone();
    }
}