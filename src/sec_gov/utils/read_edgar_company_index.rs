use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::model::company_report_index::CompanyReportIndex;
use crate::prelude::*;
use async_std::io::prelude::*;
use async_std::io::BufReader;
use async_std::prelude::*;
mod index_table_record;
mod index_table_schema;
use self::index_table_schema::IndexTableSchema;

pub async fn read_edgar_company_index(file: EdgarFile) -> Result<CompanyReportIndex, Failure> {
    let file = file.into_file();
    let file = BufReader::new(file);
    let mut lines = file.split(b'\n');
    let mut previous_line = String::new();
    let mut table_schema: Option<IndexTableSchema> = None;
    let mut result = CompanyReportIndex::new();
    while let Some(line) = lines.next().await {
        let line = line?;
        let line = String::from_utf8_lossy(&line).to_string();
        if let Some(ref table_schema) = table_schema {
            let record = table_schema.create_record(&line)?;
            let report_ref = record.create_company_report_ref()?;
            result.push_report(report_ref);
        }
        if IndexTableSchema::is_separator(&line) {
            if table_schema.is_some() {
                return Err(Failure::msg("Edgar index file contains two separators"));
            }
            table_schema = Some(IndexTableSchema::from_header(&previous_line)?);
        }
        previous_line = line;
    }
    return Ok(result);
}