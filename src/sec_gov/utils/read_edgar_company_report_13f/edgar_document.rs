use crate::prelude::*;
use crate::sec_gov::utils::read_edgar_company_report_13f::edgar_xml_document::EdgarXmlDocument;

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

    pub fn is_xml_document(&self) -> bool {
        let is_start_contains = self.text.find(Self::XML_START_TAG).is_some();
        let is_end_contains = self.text.find(Self::XML_END_TAG).is_some();
        return is_start_contains && is_end_contains;
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