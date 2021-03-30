use crate::app::model::date::Date;

#[derive(Debug)]
pub struct DailyFundReportImportRequest {
    started_at: Date, 
    ended_at: Option<Date>,
    process_only_new: bool,
}

impl DailyFundReportImportRequest {
    pub fn new(started_at: Date) -> DailyFundReportImportRequest {
        return DailyFundReportImportRequest {
            started_at,
            ended_at: None,
            process_only_new: false,
        }
    }

    pub fn with_end_at(mut self, ended_at: Date) -> Self {
        self.ended_at = Some(ended_at);
        return self;
    }

    pub fn with_process_only_new(mut self, only_new: bool) -> Self {
        self.process_only_new = only_new;
        return self;
    }

    pub fn get_started_at(&self) -> &Date {
        return &self.started_at;
    }

    pub fn get_ended_at(&self) -> &Option<Date> {
        return &self.ended_at;
    }

    pub fn get_process_only_new(&self) -> bool {
        return self.process_only_new;
    }
}