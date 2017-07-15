use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use handlebars::{Handlebars, to_json};

extern crate handlebars;

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
    fn out(&self, template: &Template) -> String {
        let mut out: String = String::new();

        out.push_str(template.col_start.as_str());
        for ref buf in self.buffers.iter() {
            out.push_str(format!("{}{}{}",
                    template.segment_start.as_str(),
                    buf,
                    template.segment_end.as_str()
                ).as_str());
        }
        out.push_str(template.col_end.as_str());

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
    fn out(&self, template: &Template) -> String {
        let mut out: String = String::new();

        out.push_str(template.row_start.as_str());
        for ref col in self.cols.iter() {
            let formatted = format!("{}", col.out(template)); 
            out.push_str(formatted.as_str());
        }
        out.push_str(template.row_end.as_str());

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
    button: String,
    select: String 
}

#[derive(PartialEq)]
enum InputType {
    Text,
    Radio,
    Checkbox,
    Textarea
}

#[derive(PartialEq)]
enum Type {
    Unknown,
    Input(InputType),
    Button,
    Label,
    Hr,
    Select
}

struct Working {
    str: String,
    until: char,
    work_type: Type
}
impl Working {
    fn new(until: char, work_type: Type) -> Working {
        Working {
            str: String::new(),
            until: until,
            work_type: work_type
        }
    }
    fn is_working(&self) -> bool {
        self.until != '\0'
    }
    fn append(&mut self, chr: char) {
        self.str.push(chr);
    }
    fn compile(&mut self, template: &Template) -> String {
        // Convert type to template
        let ref template = match self.work_type {
            Type::Button => template.button.as_str(),
            Type::Label => template.label.as_str(),
            Type::Hr => template.hr.as_str(),
            Type::Select => template.select.as_str(),
            Type::Input(ref t) => match t {
                &InputType::Text => template.text.as_str(),
                &InputType::Radio => template.radio.as_str(),
                &InputType::Checkbox => template.checkbox.as_str(),
                &InputType::Textarea => template.textarea.as_str()
            },
            _ => ""
        };

        // No actual template available
        if template.is_empty() {
            return String::new();
        }

        // Compile template
        let mut handlebars = Handlebars::new();
        assert!(handlebars.register_template_string("tmpl", template)
            .is_ok());

        // Bind data to template
        let mut data = BTreeMap::new();
        let val = match self.work_type {
            Type::Select => {
                let options: Vec<&str> = self.str.split(",").collect();
                to_json(&options)
            },
            _ => to_json(&self.str)
        };
        data.insert("value".to_string(), val);
        handlebars.render("tmpl", &data).unwrap()
    }
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
        Some(file) => {
            let dir = env::current_dir().unwrap();
            let template_file = format!("{}/{}", dir.display(), file);

            let mut in_file = match File::open(&template_file) {
                Err(_) => panic!("Unable to open template file"),
                Ok(file) => file,
            };

            let mut content = String::new();
            in_file.read_to_string(&mut content)
                .expect("Unable to process template file");

            let sp: Vec<&str> = content.split("\n").collect();

            Template {
                container_start: sp[0].to_string().trim().to_string(),
                row_start: sp[1].to_string().trim().to_string(),
                col_start: sp[2].to_string().trim().to_string(),
                segment_start: sp[3].to_string().trim().to_string(),
                segment_end: sp[4].to_string().trim().to_string(),
                col_end: sp[5].to_string().trim().to_string(),
                row_end: sp[6].to_string().trim().to_string(),
                container_end: sp[7].to_string().trim().to_string(),
                label: sp[8].to_string().trim().to_string(),
                text: sp[9].to_string().trim().to_string(),
                checkbox: sp[10].to_string().trim().to_string(),
                radio: sp[11].to_string().trim().to_string(),
                textarea: sp[12].to_string().trim().to_string(),
                button: sp[13].to_string().trim().to_string(),
                select: sp[14].to_string().trim().to_string(),
                hr: sp[15].to_string().trim().to_string()
            }

        },
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
            button: "<button>{{value}}</button>".to_string(),
            select: "<select>{{#each value}}<option>{{this}}</option>{{/each}}</select>".to_string()
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
        let mut working = Working::new('\0', Type::Unknown);

        // Iterate over characters in string
        for (i, c) in line.chars().enumerate() {
            // Match c against known tokens
            match c {
                '-' => {
                    dashes += 1;
                    if dashes == 3 {
                        row.get_col(col_index-1).append_str(
                            Working::new('\0', Type::Hr)
                                .compile(&template).as_str()
                            );
                        break;
                    }
                },

                // Controlling form modes
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

                // Handle items that have a counterpart
                c if c == '[' && form_mode
                    => working = Working::new(']', Type::Unknown),
                c if c == '{' && form_mode
                    => working = Working::new('}', Type::Label),
                c if c == '(' && form_mode
                    => working = Working::new(')', Type::Button),
                c if c == '<' && form_mode
                    => working = Working::new('>', Type::Select),

                // Handle input type and we are waiting for mode qualifier
                c if c != working.until
                    && working.until != '\0'
                    && working.work_type == Type::Unknown =>
                        match c {
                            'o' => working.work_type = Type::Input(
                                InputType::Radio),
                            '/' => working.work_type = Type::Input(
                                InputType::Checkbox),
                            '+' => working.work_type = Type::Input(
                                InputType::Textarea),
                            c => {
                                working.work_type = Type::Input(
                                    InputType::Text);
                                working.append(c);
                            },
                        },

                // Handle the case we are working towards the end goal of an
                // incoming char
                c if c == working.until => {
                    row.get_col(col_index-1)
                        .append_str(working.compile(&template).as_str());
                    working = Working::new('\0', Type::Unknown);
                },
                
                // In any other scenario we are building strings
                _ => {
                    if working.is_working() {
                        working.append(c);
                    } else {
                        row.get_col(col_index-1).append(c)
                    }
                }
            }
        }
    }
    
    println!("{}", template.container_start);
    for row in rows {
        println!("{}", row.out(&template));
    }
    println!("{}", template.container_end);
}
