use crate::prelude::*;
use crate::sec_gov::utils::read_edgar_company_index::index_table_record::IndexTableRecord;

#[derive(Debug)]
pub struct IndexTableSchema {
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    name: String,
    start: usize,
    end: usize,
    is_last: bool,
}

impl Field {
    fn read_value(&self, text: &str) -> Result<String, Failure> {
        let mut end = self.end.min(text.len());
        if self.is_last {
            end = text.len();
        }
        if let Some(result) = text.get(self.start..end) {
            return Ok(result.trim().to_string());
        }
        let error = format!(
            "Unable to read field `{}` from edgar index at position `{}..{}`",
            self.name,
            self.start,
            self.end,
        );
        return Err(Failure::msg(error));
    }
}

enum HeaderScanState {
    Waiting,
    HeaderConsuming(usize, String),
}

impl IndexTableSchema {
    pub fn from_header(text: &str) -> Result<IndexTableSchema, Failure> {
        if text.is_empty() {
            return Err(Failure::msg("Edgar index header text is empty"));
        }
        let mut field_positions: Vec<(usize, String)> = Vec::new();
        let mut state = HeaderScanState::Waiting;
        let mut space_counter = 0;
        for (index, c) in text.char_indices() {
            if c.is_whitespace() {
                space_counter += 1;
            } else {
                space_counter = 0;
            }
            match state {
                HeaderScanState::Waiting => {
                    if !c.is_whitespace() {
                        let field = String::from(c);
                        state = HeaderScanState::HeaderConsuming(index, field);
                    }
                },
                HeaderScanState::HeaderConsuming(start_index, ref mut field) => {
                    field.push(c);
                    if space_counter >= 2 {
                        let field = field.trim().to_string();
                        field_positions.push((start_index, field));
                        state = HeaderScanState::Waiting;
                    }
                },
            }
        }
        if let HeaderScanState::HeaderConsuming(start_index, field) = state {
            let field = field.trim().to_string();
            field_positions.push((start_index, field));
        }
        let mut fields = Vec::new();
        let mut end_index = text.len();
        let mut is_last = true;
        for (start_index, field_name) in field_positions.drain(..).rev() {
            assert!(start_index <= end_index);
            assert!(!field_name.is_empty());
            let field = Field {
                name: field_name,
                start: start_index,
                end: end_index,
                is_last,
            };
            is_last = false;
            fields.push(field);
            end_index = start_index;
        }
        fields.reverse();
        if fields.is_empty() {
            return Err(Failure::msg("Edgar index header is empty"));
        }
        let schema = IndexTableSchema {
            fields,
        };
        return Ok(schema);
    }

    pub fn is_separator(text: &str) -> bool {
        for c in text.chars() {
            if c != '-' {
                return false;
            }
        }
        return !text.is_empty();
    }

    pub fn create_record(&self, text: &str) -> Result<IndexTableRecord, Failure> {
        let mut result = IndexTableRecord::new();
        for field in self.fields.iter() {
            let key = field.name.clone();
            let value = field.read_value(text)?;
            result.set(key, value);
        }
        return Ok(result);
    }
}
