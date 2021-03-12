use crate::prelude::*;
use crate::sec_gov::utils::read_edgar_company_report_13f::edgar_fragment::EdgarFragment;
use crate::sec_gov::utils::read_edgar_company_report_13f::edgar_xml_document::EdgarXmlDocument;
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

#[derive(Debug)]
enum LineContent<'a> {
    Start(&'a str),
    Content(&'a str),
    End(&'a str),
    None,
}

struct FragmentWriter {
    path: PathBuf,
    fragment: Option<EdgarFragment>,
}

impl FragmentWriter {
    pub fn empty(path: PathBuf) -> FragmentWriter {
        return FragmentWriter {
            path,
            fragment: None,
        }
    }

    pub fn new(path: PathBuf) -> FragmentWriter {
        return FragmentWriter {
            path,
            fragment: Some(EdgarFragment::new()),
        };
    }

    pub fn is_some(&self) -> bool {
        return self.fragment.is_some();
    }

    pub fn is_none(&self) -> bool {
        return self.fragment.is_none();
    }

    pub fn write_line(&mut self, line: &str, skip_empty: bool) -> Result<(), Failure> {
        if let Some(working_document) = self.fragment.as_mut() {
            if skip_empty && line.is_empty() {
                return Ok(());
            }
            working_document.push_line(line);
            return Ok(());
        }
        return crate::fail!("Unable to write to edgar document: `{}`", self.path.display());
    }

    pub fn take(&mut self) -> Option<EdgarFragment> {
        return self.fragment.take();
    }
}

impl Reader {
    // const SEC_HEADER_END_TAG: &'static str = "</SEC-HEADER>";
    // const IMS_HEADER_END_TAG: &'static str = "</IMS-HEADER>";
    const DOCUMENT_TAG: &'static str = "DOCUMENT";
    const XML_TAG: &'static str = "XML";

    pub fn new(path: PathBuf, file: File) -> Reader {
        let file = BufReader::new(file);
        let file = file.lines();
        return Reader {
            path,
            file,
        };
    }

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

    pub async fn read_xml(&mut self) -> Result<Option<EdgarXmlDocument>, Failure> {
        let fragment = self.read_fragment(&Self::XML_TAG).await?;
        if let Some(fragment) = fragment {
            let document = EdgarXmlDocument::parse(fragment.get_text())?;
            return Ok(Some(document));
        }
        return Ok(None);
    }

    async fn read_fragment(&mut self, tag: &str) -> Result<Option<EdgarFragment>, Failure> {
        let mut result = FragmentWriter::empty(self.path.clone());
        while let Some(line) = self.file.next().await {
            let line = line?;
            let line_content = Self::get_content_tag(&line, tag);
            match line_content {
                LineContent::Start(start_content) => {
                    if result.is_some() {
                        return crate::fail!("Edgar fragment double start tag detected `{}`", self.path.display());
                    }
                    result = FragmentWriter::new(self.path.clone());
                    result.write_line(start_content, true)?;
                },
                LineContent::Content(content) => {
                    if result.is_some() {
                        return crate::fail!("Edgar fragment double start tag detected within content `{}`", self.path.display());
                    }
                    result = FragmentWriter::new(self.path.clone());
                    result.write_line(content, true)?;
                    return Ok(result.take());
                },
                LineContent::End(end_content) => {
                    result.write_line(end_content, true)?;
                    if result.is_none() {
                        return crate::fail!("Edgar fragment double end tag detected `{}`", self.path.display());
                    }
                    return Ok(result.take());
                },
                LineContent::None => {
                    if result.is_some() {
                        result.write_line(&line, false)?;
                    }
                },
            }
        }
        if result.is_some() {
            return crate::fail!("Edgar document parsing incomplete `{}`", self.path.display());
        }
        return Ok(None);
    }
}