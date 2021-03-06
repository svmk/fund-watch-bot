use crate::prelude::*;
use crate::sec_gov::utils::read_edgar_company_report_13f::edgar_document::EdgarDocument;
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
    const IMS_HEADER_END_TAG: &'static str = "</IMS-HEADER>";
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
            if line == Self::IMS_HEADER_END_TAG {
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