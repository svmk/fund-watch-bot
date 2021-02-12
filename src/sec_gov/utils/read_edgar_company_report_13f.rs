use crate::prelude::*;
use crate::sec_gov::model::company_report_13f::CompanyReport13F;
use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::model::form_13f::Form13F;
use crate::sec_gov::model::cik::Cik;
use crate::sec_gov::model::company_name::CompanyName;
use crate::app::model::date::Date;
use std::str::FromStr;
use std::path::PathBuf;
use async_std::io::prelude::*;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::fs::File;
use async_std::io::Lines;

pub struct Reader {
    path: PathBuf,
    file: Lines<BufReader<File>>,
}

impl Reader {
    const SEC_HEADER_END_TAG: &'static str = "</SEC-HEADER>";
    const DOCUMENT_START_TAG: &'static str = "<DOCUMENT>";
    const DOCUMENT_END_TAG: &'static str = "</DOCUMENT>";

    pub fn new(path: PathBuf, file: File) -> Reader {
        let file = BufReader::new(file);
        let file = file.lines();
        return Reader {
            path,
            file,
        };
    }

    pub async fn skip_header(&mut self) -> Result<(), Failure> {
        let mut header_was_skipped = false;
        while let Some(line) = self.file.next().await {
            let line = line?;
            let line = line.trim();
            if line == Self::SEC_HEADER_END_TAG {
                header_was_skipped = true;
                break;
            }
        }
        if !header_was_skipped {
            return Err(Failure::msg(format!("Edgar header not found at file `{}`", self.path.display())));
        }
        return Ok(());
    }

    pub async fn read_document(&mut self) -> Result<Option<EdgarDocument>, Failure> {
        let mut working_document: Option<EdgarDocument> = None;
        while let Some(line) = self.file.next().await {
            let line = line?;
            let line = line.trim();
            if line == Self::DOCUMENT_START_TAG {
                working_document = Some(EdgarDocument::new());
            } else if line == Self::DOCUMENT_END_TAG {
                if working_document.is_none() {
                    return Err(Failure::msg(format!("Edgar document double end tag detected`{}`", self.path.display())));
                }
                return Ok(working_document);
            } else {
                if let Some(ref mut working_document) = working_document {
                    working_document.push_line(line);
                }
            }
        }
        if working_document.is_some() {
            return Err(Failure::msg(format!("Edgar document parsing incomplete `{}`", self.path.display())));
        }
        return Ok(None);
    }
}

pub struct EdgarDocument {
    text: String,
}

impl EdgarDocument {
    const TYPE_TAG: &'static str = "<TYPE>";
    const XML_START_TAG: &'static str = "<XML>";
    const XML_END_TAG: &'static str = "</XML>";
    pub fn new() -> EdgarDocument {
        return EdgarDocument {
            text: String::new(),
        };
    }

    pub fn push_line(&mut self, line: &str) {
        self.text.push_str(line);
        self.text.push_str("\n");
    }

    pub fn get_document_type(&self) -> Result<&str, Failure> {
        for line in self.text.lines() {
            if line.starts_with(Self::TYPE_TAG) {
                let document_type = &line[Self::TYPE_TAG.len()..];
                return Ok(document_type);
            }
        }
        return Err(Failure::msg("Unable to get edgar document type"));
    }

    pub fn as_xml_document(&self) -> Result<EdgarXmlDocument, Failure> {
        let start = self.text.find(Self::XML_START_TAG).ok_or_else(|| {
            return Failure::msg("Unable to find xml start tag in edgar document");
        })?;
        let end = self.text.find(Self::XML_END_TAG).ok_or_else(|| {
            return Failure::msg("Unable to find xml end tag in edgar document");
        })?;
        let content = self.text.get(start..end).ok_or_else(|| {
            return Failure::msg("Unable to get xml content in edgar document");
        })?;
        let content = content.trim();
        if content.is_empty() {
            return Err(Failure::msg("Xml content is empty in edgar document"));
        }
        let xml_document = EdgarXmlDocument::parse(content)?;
        return Ok(xml_document);
    }
}

use sxd_document::Package;
use sxd_document::parser::parse;
use sxd_xpath::evaluate_xpath;
use sxd_xpath::Value as XPathValue;
pub struct EdgarXmlDocument {
    file: Package,
}

impl EdgarXmlDocument {
    pub fn parse(text: &str) -> Result<EdgarXmlDocument, Failure> {
        let file = parse(text)?;
        let document = EdgarXmlDocument {
            file,
        };
        return Ok(document);
    }

    pub fn read_xpath_content(&self, selector: &str) -> Result<String, Failure> {
        let document = self.file.as_document();
        let value = evaluate_xpath(&document, selector)?;
        let value = match value {
            XPathValue::String(value) => {
                value
            },
            XPathValue::Boolean(..) => {
                return Err(Failure::msg(format!("Invalid type boolean for selecor `{}`", selector)));
            },
            XPathValue::Number(..) => {
                return Err(Failure::msg(format!("Invalid type number for selecor `{}`", selector)));
            },
            XPathValue::Nodeset(..) => {
                return Err(Failure::msg(format!("Invalid type nodeset for selecor `{}`", selector)));
            },
        };
        return Ok(value);
    }
}

fn parse_document_13f(document: &EdgarDocument) -> Result<Option<Form13F>, Failure> {
    const DOCUMENT_TYPE_13F_HR: &'static str = "13F";
    let document_type = document.get_document_type()?;
    if !document_type.starts_with(DOCUMENT_TYPE_13F_HR) {
        return Ok(None);
    }
    let document = document.as_xml_document()?;
    let cik = document.read_xpath_content("//edgarSubmission//headerData//filerInfo//filer//cik")?;
    let cik = Cik::from_str(&cik)?;
    let company_name = document.read_xpath_content("//edgarSubmission//formData//coverPage//filingManager//name")?;
    let company_name = CompanyName::from_string(company_name)?;
    let period_of_report = document.read_xpath_content("//edgarSubmission//headerData//filerInfo//periodOfReport")?;
    let period_of_report = Date::parse_mdy(&period_of_report)?;
    let report_calendar_or_quarter = document.read_xpath_content("//edgarSubmission//formData//coverPage//reportCalendarOrQuarter")?;
    let report_calendar_or_quarter = Date::parse_mdy(&report_calendar_or_quarter)?;
    let report = Form13F::new(
        cik,
        company_name,
        period_of_report,
        report_calendar_or_quarter,
    );
    return Ok(Some(report));
}

fn parse_document_information_table(document: &EdgarDocument) -> Result<Option<(Form13F)>, Failure> {
    const DOCUMENT_TYPE_INFORMATION_TABLE: &'static str = "INFORMATION TABLE";
    let document_type = document.get_document_type()?;
    if document_type != DOCUMENT_TYPE_INFORMATION_TABLE {
        return Ok(None);
    }
    let xml_document = document.as_xml_document()?;
    unimplemented!()
}

pub async fn read_edgar_company_report_13f(file: EdgarFile) -> Result<CompanyReport13F, Failure> {
    let path = file.get_path().clone();
    let file = file.into_file();
    
    let mut reader = Reader::new(path, file);
    reader.skip_header().await?;
    while let Some(document) = reader.read_document().await? {
        if let Some(_) = parse_document_13f(&document)? {

        }
        if let Some(_) = parse_document_information_table(&document)? {

        }
    }
    unimplemented!()
}