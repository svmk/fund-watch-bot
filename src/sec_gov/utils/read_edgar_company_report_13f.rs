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
mod edgar_fragment;
mod reader;
use self::edgar_xml_document::EdgarXmlDocument;
use self::reader::Reader;
use self::document_reports::DocumentReports;

const EDGAR_NS_PREIFX: &'static str = "edgar";
const EDGAR_NS_13F_URI: &'static str = "http://www.sec.gov/edgar/thirteenffiler";
const EDGAR_NS_INFORMATION_TABLE_URI: &'static str = "http://www.sec.gov/edgar/document/thirteenf/informationtable";


fn parse_document_13f(document: &EdgarXmlDocument) -> Result<Option<Form13F>, Failure> {
    let document = document.root();
    let document = document.with_namespace(EDGAR_NS_PREIFX.to_string(), EDGAR_NS_13F_URI.to_string());
    if !document.exists("//edgar:edgarSubmission")? {
        return Ok(None);
    }
    let cik = document.read_xpath_string("//edgar:edgarSubmission/edgar:headerData/edgar:filerInfo/edgar:filer/edgar:credentials/edgar:cik")?;
    let cik = Cik::from_str(&cik)?;
    let company_name = document.read_xpath_string("//edgar:edgarSubmission/edgar:formData/edgar:coverPage/edgar:filingManager/edgar:name")?;
    let company_name = CompanyName::from_string(company_name)?;
    let period_of_report = document.read_xpath_string("//edgar:edgarSubmission/edgar:headerData/edgar:filerInfo/edgar:periodOfReport")?;
    let period_of_report = Date::parse_mdy(&period_of_report)?;
    let report_calendar_or_quarter = document.read_xpath_string("//edgar:edgarSubmission/edgar:formData/edgar:coverPage/edgar:reportCalendarOrQuarter")?;
    let report_calendar_or_quarter = Date::parse_mdy(&report_calendar_or_quarter)?;
    let report = Form13F::new(
        cik,
        company_name,
        period_of_report,
        report_calendar_or_quarter,
    );
    return Ok(Some(report));
}

fn parse_document_information_table(document: &EdgarXmlDocument) -> Result<Option<Form13FComponentTable>, Failure> {
    let document = document.root();
    let document = document.with_namespace(EDGAR_NS_PREIFX.to_string(), EDGAR_NS_INFORMATION_TABLE_URI.to_string());
    if !document.exists("//edgar:informationTable")? {
        return Ok(None);
    }
    let info_tables = document.list("//edgar:informationTable//edgar:infoTable")?;
    let mut result = Form13FComponentTable::new();
    for info_table in info_tables.iter() {
        let company_name = info_table.read_xpath_string("edgar:nameOfIssuer")?;
        let company_name = company_name.trim();
        let company_name = CompanyName::from_str(company_name)?;
        let cusip = info_table.read_xpath_string("edgar:cusip")?;
        let cusip = Cusip::from_string(cusip)?;
        let investment_discretion = info_table.read_xpath_string("edgar:investmentDiscretion")?;
        let investment_discretion = InvestmentDiscretion::from_str(&investment_discretion)?;
        let share = info_table.read_xpath_string("edgar:shrsOrPrnAmt/edgar:sshPrnamt")?;
        let share = Share::from_str(&share)?;
        let share_type = info_table.read_xpath_string("edgar:shrsOrPrnAmt/edgar:sshPrnamtType")?;
        let share_type = share_type.trim();
        // sh - shares
        // prn - principal amount
        // https://www.sec.gov/rules/extra/form13f.txt
        if share_type != "SH" && share_type != "PRN" {
            return crate::fail!("Unknown share type `{}`", share_type);
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
    
    let mut reader = Reader::new(path.clone(), file);
    let mut document_reports = DocumentReports::new();
    while let Some(document) = reader.read_xml().await? {
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