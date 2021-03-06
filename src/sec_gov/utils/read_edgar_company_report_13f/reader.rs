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
    working_document: Option<EdgarDocument>,
}

#[derive(Debug)]
enum LineContent<'a> {
    Start(&'a str),
    Content(&'a str),
    End(&'a str),
    None,
}

impl Reader {
    // const SEC_HEADER_END_TAG: &'static str = "</SEC-HEADER>";
    // const IMS_HEADER_END_TAG: &'static str = "</IMS-HEADER>";
    const DOCUMENT_TAG: &'static str = "DOCUMENT";

    pub fn new(path: PathBuf, file: File) -> Reader {
        let file = BufReader::new(file);
        let file = file.lines();
        return Reader {
            path,
            file,
            working_document: None,
        };
    }

    // pub async fn skip_header(&mut self) -> Result<(), Failure> {
    //     let mut header_was_skipped = false;
    //     while let Some(line) = self.file.next().await {
    //         let line = line?;
    //         let line = line.trim();
    //         if line == Self::SEC_HEADER_END_TAG {
    //             header_was_skipped = true;
    //             break;
    //         }
    //         if line == Self::IMS_HEADER_END_TAG {
    //             header_was_skipped = true;
    //             break;
    //         }
    //     }
    //     if !header_was_skipped {
    //         return crate::fail!("Edgar header not found at file `{}`", self.path.display());
    //     }
    //     return Ok(());
    // }

    fn get_content_tag<'a>(line: &'a str, tag: &str) -> LineContent<'a> {
        let start_tag = format!("<{}>", tag);
        let start_index = line
            .find(&start_tag)
            .map(|start_index| {
                return start_index + start_tag.len();
            });
        let end_tag = format!("</{}>", tag);
        let end_index = line.find(&end_tag);
        match (start_index, end_index) {
            (Some(start_index), Some(end_index)) => {
                let line = &line[start_index..end_index];
                return LineContent::Content(line);
            },
            (Some(start_index), None) => {
                let line = &line[start_index..];
                return LineContent::Start(line);
            },
            (None, Some(end_index)) => {
                let line = &line[..end_index];
                return LineContent::End(line);
            },
            (None, None) => {
                return LineContent::None;
            },
        }
    }

    pub async fn read_document(&mut self) -> Result<Option<EdgarDocument>, Failure> {
        while let Some(line) = self.file.next().await {
            let line = line?;
            let line_content = Self::get_content_tag(&line, Self::DOCUMENT_TAG);
            match line_content {
                LineContent::Start(start_content) => {
                    if self.working_document.is_some() {
                        return crate::fail!("Edgar document double start tag detected `{}`", self.path.display());
                    }
                    self.working_document = Some(EdgarDocument::new());
                    self.write_line_to_document(start_content, true)?;
                },
                LineContent::Content(content) => {
                    if self.working_document.is_some() {
                        return crate::fail!("Edgar document double start tag detected within content `{}`", self.path.display());
                    }
                    self.working_document = Some(EdgarDocument::new());
                    self.write_line_to_document(content, true)?;
                    return Ok(self.working_document.take());
                },
                LineContent::End(end_content) => {
                    self.write_line_to_document(end_content, true)?;
                    if self.working_document.is_none() {
                        return crate::fail!("Edgar document double end tag detected `{}`", self.path.display());
                    }
                    return Ok(self.working_document.take());
                },
                LineContent::None => {
                    if self.working_document.is_some() {
                        self.write_line_to_document(&line, false)?;
                    }
                },
            }
        }
        if self.working_document.is_some() {
            return crate::fail!("Edgar document parsing incomplete `{}`", self.path.display());
        }
        return Ok(None);
    }

    fn write_line_to_document(&mut self, line: &str, skip_empty: bool) -> Result<(), Failure> {
        if let Some(working_document) = self.working_document.as_mut() {
            if skip_empty && line.is_empty() {
                return Ok(());
            }
            working_document.push_line(line);
            return Ok(());
        }
        return crate::fail!("Unable to write to edgar document: `{}`", self.path.display());
    }
}