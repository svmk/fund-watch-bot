use crate::prelude::*;
use sxd_document::dom::Document;
use sxd_xpath::evaluate_xpath;
use sxd_xpath::Value as XPathValue;
#[derive(new)]
pub struct EdgarXmlFragment<'a> {
    document: Document<'a>,
}

impl <'a>EdgarXmlFragment<'a> {
    pub fn read_xpath_string(&self, selector: &str) -> Result<String, Failure> {
        let value = evaluate_xpath(&self.document, selector)?;
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

    pub fn list(&self, selector: &str) -> Result<Vec<EdgarXmlFragment>, Failure> {
        let nodes = evaluate_xpath(&self.document, selector)?;
        let nodes = match nodes {
            XPathValue::Nodeset(nodes) => nodes,
            _ => {
                return Err(Failure::msg(format!("Expected nodeset for selecor `{}`", selector)));
            }
        };
        let nodes: Vec<_> = nodes
            .iter()
            .map(|node| {
                return EdgarXmlFragment::new(node.document());
            })
            .collect();
        return Ok(nodes);
    }
}