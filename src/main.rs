use std::io::{self, Read};
use std::env;
use std::fmt;

struct Col {
    pub buffers: Vec<String>
}
impl Col {
    fn new() -> Col {
        Col { buffers: Vec::new() }
    }
    fn add_buf(&mut self) {
        self.buffers.push(String::new());
    }
    fn append(&mut self, chr: char) {
        let mut len = self.buffers.len();

        if len == 0 {
            self.add_buf();
            len = 1;
        }

        let ref mut buffer = self.buffers[len-1];
        buffer.push(chr);
    }
    fn append_str(&mut self, str: &str) {
        for chr in str.chars() {
            self.append(chr);
        }
    }
    fn out(&self) -> String {
        let mut out: String = String::new();

        out.push_str("<div class=\"col\">");
        for ref buf in self.buffers.iter() {
            out.push_str(format!("<div>{}</div>", buf).as_str());
        }
        out.push_str("</div>");

        out
    }
}

struct Row {
    pub cols: Vec<Col>
}
impl Row {
    fn add_col(&mut self, col: Col) {
        self.cols.push(col);
    }
    fn get_col(&mut self, index: u8) -> &mut Col {
        self.cols.get_mut(index as usize).unwrap()
    }
    fn col_count(&self) -> usize {
        self.cols.len()
    }
    fn new() -> Row {
        let mut row: Row = Row { cols: Vec::new() };
        row.add_col(Col::new());
        row
    }
    fn out(&self) -> String {
        let mut out: String = String::new();

        out.push_str("<div class=\"row\">");
        for ref col in self.cols.iter() {
            let formatted = format!("{}", col.out()); 
            out.push_str(formatted.as_str());
        }
        out.push_str("</div>");

        out
    }
}

struct Template {
    container_start: String,
    container_end: String,
    row_start: String,
    row_end: String,
    col_start: String,
    col_end: String,
    segment_start: String,
    segment_end: String,
    label: String,
    text: String,
    checkbox: String,
    radio: String,
    textarea: String,
    hr: String,
    button: String
}

/** 
 * An empty line implies a new "row"
 * A line starting with a | implies a columnar form section
 * [value] is an input text field
 * {value} is an input textarea field
 * @ is a radio
 * * is a checkbox
 * --- is an hr
 * 
 */
fn main() {
    // Read in standard input
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
        .expect("Error reading from stdin");

    // Split input by line
    let v: Vec<&str> = buffer.split("\n").collect();

    // Keep track of rows in output
    let mut rows: Vec<Row> = Vec::new();
    let mut row: Row = Row::new();

    // Check arg for template
    let template = match env::args().nth(1) {
        /*
        Some(file) => {
            Template {
                container_start: "",
                container_end: "",
                row_start: "<div>",
                row_end: "</div>",
                col_start: "<span>",
                col_end: "</span>",
                segment_start: "",
                segment_end: "<br/>",
                label: "<label>{[value}}</label>",
                text: "<input type=\"text\" value=\"{{value}}\"/>",
                checkbox: "<input type=\"checkbox\"/>",
                radio: "<input type=\"radio\"/>",
                textarea: "<textarea>{{value}}</textarea>",
                hr: "<hr/>",
                button: "<button>{{value}}</button>"
            }

        },
        */
        _ => Template {
            container_start: "".to_string(),
            container_end: "".to_string(),
            row_start: "<div>".to_string(),
            row_end: "</div>".to_string(),
            col_start: "<span>".to_string(),
            col_end: "</span>".to_string(),
            segment_start: "".to_string(),
            segment_end: "<br/>".to_string(),
            label: "<label>{[value}}</label>".to_string(),
            text: "<input type=\"text\" value=\"{{value}}\"/>".to_string(),
            checkbox: "<input type=\"checkbox\"/>".to_string(),
            radio: "<input type=\"radio\"/>".to_string(),
            textarea: "<textarea>{{value}}</textarea>".to_string(),
            hr: "<hr/>".to_string(),
            button: "<button>{{value}}</button>".to_string()
        }
    };

    // Iterate over lines in input
    for line in &v {
        // New row by way of empty line
        if line.chars().count() == 0 {
            rows.push(row);
            row = Row::new();
            continue;
        }

        let mut form_mode = false; // If user toggled form mode
        let mut col_index: u8 = 1; // Keeping tracking of column index
        let mut dashes: u8 = 0; // Keep track of dash count

        // Iterate over characters in string
        for (i, c) in line.chars().enumerate() {
            // Match c against known tokens
            match c {
                '-' => {
                    dashes += 1;
                    if dashes == 3 {
                        row.get_col(col_index-1)
                            .append_str("<hr/>");
                        break;
                    }
                },
                '|' => {
                    if i == 0 {
                        form_mode = true;
                        row.get_col(col_index-1).add_buf()
                    } else if form_mode {
                        // Check if we have enough columns in row
                        // if not add a new column
                        if (row.col_count() as u8) < col_index + 1 {
                            row.add_col(Col::new());
                        } 

                        // Increment index
                        col_index += 1;

                        row.get_col(col_index-1).add_buf();
                    } else {
                        row.get_col(col_index-1).append(c);
                    }
                },
                '<' => row.get_col(col_index-1)
                        .append_str("<label>"),
                '>' => row.get_col(col_index-1)
                        .append_str("</label>"),
                '[' => row.get_col(col_index-1)
                        .append_str("<input type=\"text\" value=\""),
                ']' => row.get_col(col_index-1)
                        .append_str("\">"),
                '{' => row.get_col(col_index-1)
                        .append_str("<textarea>"),
                '}' => row.get_col(col_index-1)
                        .append_str("</textarea>"),
                '@' => row.get_col(col_index-1)
                        .append_str("<input type=\"radio\">"),
                '(' => row.get_col(col_index-1)
                        .append_str("<button class=\"btn\">"),
                ')' => row.get_col(col_index-1)
                        .append_str("</button>"),
                _ => row.get_col(col_index-1).append(c)
            }
        }
    }
    
    println!("<div class=\"container\">");
    for row in rows {
        println!("{}", row.out());
    }
    println!("</div>");
}
