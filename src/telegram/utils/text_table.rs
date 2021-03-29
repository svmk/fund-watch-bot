use std::fmt;
#[derive(Debug)]
struct Column {
    code: String,
    printable_text: String,
}

#[derive(Debug)]
pub struct Row {
    columns: Vec<Column>,
}

impl Row {
    pub fn new() -> Row {
        return Row {
            columns: Vec::new(),
        }
    }

    pub fn with_cell(mut self, printable_text: impl fmt::Display, code: impl fmt::Display) -> Self {
        let code = format!("{}", code);
        let printable_text = format!("{}", printable_text);
        let column = Column {code, printable_text};
        self.columns.push(column);
        return self;
    }

    pub fn with_code(self, code: impl fmt::Display) -> Self {
        return self.with_cell("", code);
    }

    pub fn with_text(self, text: impl fmt::Display) -> Self {
        let text = format!("{}", text);
        return self.with_cell(text.clone(), text);
    }

    fn get_column_count(&self) -> usize {
        return self.columns.len();
    }
}


#[derive(Clone)]
struct ColumnRenderer {
    max_width: usize,
}

impl ColumnRenderer {
    pub fn new() -> ColumnRenderer {
        return ColumnRenderer {
            max_width: 0,
        };
    }

    pub fn update_width(&mut self, width: usize) {
        self.max_width = self.max_width.max(width);
    }

    pub fn render(&self, column: &Column) -> String {
        let leading_spaces = self.max_width - column.printable_text.len();
        let result = " ".repeat(leading_spaces);
        let result = format!("{}{}", column.code, result);
        return result;
    }
}

#[derive(Debug)]
pub struct TextTable {
    rows: Vec<Row>,
}

impl TextTable {
    pub fn new_empty() -> TextTable {
        return TextTable {
            rows: Vec::new(),
        }
    }

    pub fn with_row(mut self, row: Row) -> Self {
        self.rows.push(row);
        return self;
    }

    fn get_column_count(&self) -> usize {
        let mut result = 0;
        for row in self.rows.iter() {
            result = result.max(row.get_column_count());
        }
        return result;
    }

    fn create_column_renderers(&self) -> Vec<ColumnRenderer> {
        let column_count = self.get_column_count();
        let mut result = Vec::with_capacity(column_count);
        for _ in 0..column_count {
            result.push(ColumnRenderer::new());
        }
        for row in self.rows.iter() {
            for (column_index, column) in row.columns.iter().enumerate() {
                result[column_index].update_width(column.printable_text.len());
            }
        }
        return result;
    }
}

impl fmt::Display for TextTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column_renderers = self.create_column_renderers();
        for row in self.rows.iter() {
            let mut row_text = Vec::new();
            for (column_index, column) in row.columns.iter().enumerate() {
                let column_renderer = &column_renderers[column_index];
                row_text.push(column_renderer.render(&column));
            }
            let row_text = row_text.join(" ");
            write!(f, "{}\n", row_text)?;
        }
        return Ok(());
    }
}