#[derive(Error, Debug)]
pub enum FundChangesError {
    #[error("Same daily reports")]
    SameDailyReports,
    #[error("Fund id differ")]
    FundIdDiffer,
}