use crate::app::model::date::Date;

pub fn date_view(date: &Date) -> String {
    return format!(
        "{:04}-{:02}-{:02}", 
        date.get_year().to_i32(),
        date.get_month().to_u32(),
        date.get_day().to_u32(),
    );
}