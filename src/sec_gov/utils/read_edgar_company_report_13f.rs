use crate::prelude::*;
use crate::sec_gov::model::company_report_13f::CompanyReport13F;
use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::model::form_13f::Form13F;
use crate::sec_gov::model::cik::Cik;
use crate::sec_gov::model::company_name::CompanyName;
use crate::sec_gov::model::cusip::Cusip;
use crate::sec_gov::model::investment_discretion::InvestmentDiscretion;
use crate::sec_gov::model::form_13f_component::Form13FComponent;
use crate::sec_gov::model::form_13f_componenttable::Form13FComponentTable;
use crate::market::common::model::share::Share;
use crate::app::model::date::Date;
use std::str::FromStr;

mod document_reports;
mod edgar_xml_fragment;
mod edgar_xml_document;
mod edgar_document;
mod reader;
use self::edgar_document::EdgarDocument;
use self::reader::Reader;
use self::document_reports::DocumentReports;


fn parse_document_13f(document: &EdgarDocument) -> Result<Option<Form13F>, Failure> {
    const DOCUMENT_TYPE_13F_HR: &'static str = "13F";
    let document_type = document.get_document_type()?;
    if !document_type.starts_with(DOCUMENT_TYPE_13F_HR) {
        return Ok(None);
    }
    if !document.is_xml_document() {
        return Ok(None);
    }
    let document = document.as_xml_document()?;
    let document = document.root();
    let cik = document.read_xpath_string("//edgarSubmission//headerData//filerInfo//filer//cik")?;
    let cik = Cik::from_str(&cik)?;
    let company_name = document.read_xpath_string("//edgarSubmission//formData//coverPage//filingManager//name")?;
    let company_name = CompanyName::from_string(company_name)?;
    let period_of_report = document.read_xpath_string("//edgarSubmission//headerData//filerInfo//periodOfReport")?;
    let period_of_report = Date::parse_mdy(&period_of_report)?;
    let report_calendar_or_quarter = document.read_xpath_string("//edgarSubmission//formData//coverPage//reportCalendarOrQuarter")?;
    let report_calendar_or_quarter = Date::parse_mdy(&report_calendar_or_quarter)?;
    let report = Form13F::new(
        cik,
        company_name,
        period_of_report,
        report_calendar_or_quarter,
    );
    return Ok(Some(report));
}

fn parse_document_information_table(document: &EdgarDocument) -> Result<Option<Form13FComponentTable>, Failure> {
    const DOCUMENT_TYPE_INFORMATION_TABLE: &'static str = "INFORMATION TABLE";
    let document_type = document.get_document_type()?;
    if document_type != DOCUMENT_TYPE_INFORMATION_TABLE {
        return Ok(None);
    }
    if !document.is_xml_document() {
        return Ok(None);
    }
    let document = document.as_xml_document()?;
    let document = document.root();
    let info_tables = document.list("//ns1:informationTable//ns1:infoTable")?;
    let mut result = Form13FComponentTable::new();
    for info_table in info_tables.iter() {
        let company_name = info_table.read_xpath_string("//ns1:nameOfIssuer")?;
        let company_name = CompanyName::from_string(company_name)?;
        let cusip = info_table.read_xpath_string("//ns1:cusip")?;
        let cusip = Cusip::from_string(cusip)?;
        let investment_discretion = info_table.read_xpath_string("//ns1:investmentDiscretion")?;
        let investment_discretion = InvestmentDiscretion::from_str(&investment_discretion)?;
        let share = info_table.read_xpath_string("//ns1:shrsOrPrnAmt/ns1:sshPrnamt")?;
        let share = Share::from_str(&share)?;
        let share_type = info_table.read_xpath_string("//ns1:shrsOrPrnAmt/ns1:sshPrnamtType")?;
        if share_type != "SH" {
            return Err(Failure::msg(format!("Unknown share type {}", share_type)));
        }
        let component = Form13FComponent::new(
            company_name,
            cusip,
            investment_discretion,
            share,
        );
        result.push_component(component);
    } 
    return Ok(Some(result));
}

pub async fn read_edgar_company_report_13f(file: EdgarFile) -> Result<Option<CompanyReport13F>, Failure> {
    let path = file.get_path().clone();
    let file = file.into_file();
    
    let mut reader = Reader::new(path, file);
    reader.skip_header().await?;
    let mut document_reports = DocumentReports::new();
    while let Some(document) = reader.read_document().await? {
        if let Some(report) = parse_document_13f(&document)? {
            document_reports.set_form_13f(report)?;
        }
        if let Some(report) = parse_document_information_table(&document)? {
            document_reports.set_information_table(report)?;
        }
    }
    let report = document_reports.create_company_report_13f();
    return Ok(report);
}