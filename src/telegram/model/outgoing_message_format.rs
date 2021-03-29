#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutgoingMessageFormat {
    #[serde(rename="markdown_v2")]
    MarkdownV2,
    #[serde(rename="html")]
    Html,
}