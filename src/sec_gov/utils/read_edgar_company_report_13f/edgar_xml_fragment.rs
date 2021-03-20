use crate::prelude::*;
use sxd_document::dom::Root;
use sxd_xpath::Value as XPathValue;
use sxd_xpath::Context as XPathContext;
use sxd_xpath::Factory as XPathFactory;
use std::collections::HashMap;
use sxd_xpath::nodeset::Node;

#[derive(Debug)]
pub struct EdgarXmlFragment<'a> {
    node: Node<'a>,
    namespaces: HashMap<String, String>,
}

impl <'a>EdgarXmlFragment<'a> {
    pub fn new(root: Root<'a>) -> EdgarXmlFragment<'a> {
        return EdgarXmlFragment {
            node: Node::Root(root),
            namespaces: HashMap::new(),
        }
    }

    fn new_like(&self, node: Node<'a>) -> EdgarXmlFragment<'a> {
        return EdgarXmlFragment {
            node,
            namespaces: self.namespaces.clone(),
        }
    }

    pub fn with_namespace(mut self, prefix: String, ns_uri: String) -> EdgarXmlFragment<'a> {
        let _ = self.namespaces.insert(prefix, ns_uri);
        return self;
    }

    pub fn read_xpath_string(&self, selector: &str) -> Result<String, Failure> {
        let value = self.evaluate_xpath( selector)?;
        let value = value.string();
        return Ok(value);
    }

    pub fn list(&self, selector: &str) -> Result<Vec<EdgarXmlFragment>, Failure> {
        let nodes = self.evaluate_xpath( selector)?;
        let nodes = match nodes {
            XPathValue::Nodeset(nodes) => nodes,
            _ => {
                return Err(Failure::msg(format!("Expected nodeset for selector `{}`", selector)));
            }
        };
        let nodes: Vec<_> = nodes
            .iter()
            .map(|node| {
                return self.new_like(node);
            })
            .collect();
        return Ok(nodes);
    }

    pub fn exists(&self, selector: &str) -> Result<bool, Failure> {
        let nodes = self.evaluate_xpath( selector)?;
        let nodes = match nodes {
            XPathValue::Nodeset(nodes) => nodes,
            _ => {
                return Err(Failure::msg(format!("Expected nodeset for selector `{}`", selector)));
            }
        };
        return Ok(nodes.size() >= 1);
    }

    fn evaluate_xpath(&self, selector: &str) -> Result<XPathValue, Failure> {
        let mut context = XPathContext::new();
        for (prefix, ns_uri) in self.namespaces.iter() {
            context.set_namespace(prefix, ns_uri);
        }
        let factory = XPathFactory::new();
        let xpath = factory.build(selector)?;
        let xpath = match xpath {
            Some(xpath) => xpath,
            None => {
                return crate::fail!("Unable to compile xpath `{}`", selector);
            },
        };
        let value = xpath.evaluate(&context, self.node)?;
        return Ok(value);
    }
}