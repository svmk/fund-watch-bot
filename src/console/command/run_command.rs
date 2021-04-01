use crate::app::model::date::Date;

#[derive(Debug, StructOpt)]
pub struct RunCommand {
    started_at: Option<Date>,
}

impl RunCommand {
    pub fn get_started_at(&self) -> Option<Date> {
        return self.started_at.clone();
    }
}