use crate::prelude::*;
use crate::sec_gov::utils::read_edgar_company_report_13f::edgar_xml_fragment::EdgarXmlFragment;
use sxd_document::Package;
use sxd_document::parser::parse;

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

    pub fn root(&self) -> EdgarXmlFragment {
        return EdgarXmlFragment::new(self.file.as_document().root());
    }
}