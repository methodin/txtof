use std::env;
use std::fs::File;
use std::io::{self, Read};
use handlebars::{Handlebars, to_json};

extern crate handlebars;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ButtonConfig {
    value: String,
    trigger: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ManyConfig {
    value: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct GeneralConfig {
    value: String
}

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
    cols: Vec<Col>
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

        out.push_str("\n");
        out.push_str(template.row_start.as_str());
        for ref col in self.cols.iter() {
            let formatted = format!("{}", col.out(template)); 
            out.push_str(formatted.as_str());
        }
        out.push_str(template.row_end.as_str());

        out
    }
}

struct Page {
    name: String,
    rows: Vec<Row>
}
impl Page {
    fn new(name: String) -> Page {
        let mut page: Page = Page { name: name, rows: Vec::new() };
        page.add_row(Row::new());
        page
    }
    fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }
    fn get_current_row(&mut self) -> &mut Row {
        let len = self.rows.len();
        self.rows.get_mut(len-1).unwrap()
    }
    fn out(&self, template: &Template) -> String {
        let mut out: String = String::new();

        let config = to_json(&GeneralConfig {value: self.name.to_string()});
        out.push_str(render_template(template.container_start.as_str(), &config).as_str());

        for ref row in self.rows.iter() {
            let formatted = format!("{}", row.out(template)); 
            out.push_str(formatted.as_str());
        }

        out.push_str(render_template(template.container_end.as_str(), &config).as_str());

        out
    }
}

struct Template {
    head: String,
    foot: String,
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
impl Template {
    fn from_vec(vec: &Vec<&str>) -> Template {
        Template {
            head: vec[0].to_string().trim().to_string(),
            foot: vec[1].to_string().trim().to_string(),
            container_start: vec[2].to_string().trim().to_string(),
            container_end: vec[3].to_string().trim().to_string(),
            row_start: vec[4].to_string().trim().to_string(),
            row_end: vec[5].to_string().trim().to_string(),
            col_start: vec[6].to_string().trim().to_string(),
            col_end: vec[7].to_string().trim().to_string(),
            segment_start: vec[8].to_string().trim().to_string(),
            segment_end: vec[9].to_string().trim().to_string(),
            label: vec[10].to_string().trim().to_string(),
            text: vec[11].to_string().trim().to_string(),
            checkbox: vec[12].to_string().trim().to_string(),
            radio: vec[13].to_string().trim().to_string(),
            textarea: vec[14].to_string().trim().to_string(),
            button: vec[15].to_string().trim().to_string(),
            select: vec[16].to_string().trim().to_string(),
            hr: vec[17].to_string().trim().to_string()
        }
    }
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

        let config = match self.work_type {
            Type::Button => {
                let split: Vec<String> = self.str.split("->").map(|s| s.to_string()).collect();
                to_json(&ButtonConfig {
                    value: split[0].trim().to_owned(),
                    trigger: if split.len() > 1 { split[1].trim().to_owned() } else { "".to_string() }
                })
            },
            Type::Select => to_json(&ManyConfig {value: self.str.split(",").map(|s| s.to_owned()).collect()}),
            _ => to_json(&GeneralConfig {value: self.str.to_string()})
        };

        render_template(template, &config)
    }
}

fn render_template(template: &str, val: &serde_json::value::Value) -> String {
    // Compile template
    let mut handlebars = Handlebars::new();
    assert!(handlebars.register_template_string("tmpl", template)
        .is_ok());

    handlebars.render("tmpl", &val).unwrap()
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
    
    let mut pages: Vec<Page> = Vec::new();

    // Check arg for template
    let template = match env::args().nth(1) {
        Some(file) => {
            let mut in_file = match File::open(&file) {
                Err(_) => panic!("Unable to open template file"),
                Ok(file) => file,
            };

            let mut content = String::new();
            in_file.read_to_string(&mut content)
                .expect("Unable to process template file");

            let sp: Vec<&str> = content.split("\n").collect();

            Template::from_vec(&sp)
        },
        _ => match env::var("template") {
            Ok(val) => {
                let tv: Vec<&str> = val.split(",").collect();
                Template::from_vec(&tv)
            },
            _ => Template {
                head: "<script src=\"https://code.jquery.com/jquery-3.2.1.min.js\" integrity=\"sha256-hwg4gsxgFZhOsEEamdOYGBf13FyQuiTwlAQgxVSNgt4=\" crossorigin=\"anonymous\"></script>".to_string(),
                foot: "<script>$(function(){ $('#default').show(); \
                    $('[data-trigger]').click(function(){ \
                        $('[txtof-container]').hide(); \
                        var trigger = $(this).data('trigger'); \
                        $('#'+trigger).show(); \
                    }); \
                });</script>".to_string(),
                container_start: "<div txtof-container id=\"{{value}}\" style=\"display:none\">".to_string(),
                container_end: "</div>".to_string(),
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
                button: "<button data-trigger=\"{{trigger}}\">{{value}}</button>".to_string(),
                select: "<select>{{#each value}}<option>{{this}}</option>{{/each}}</select>".to_string()
            }
        }
    };

    let mut page: Page = Page::new("default".to_string());

    // Iterate over lines in input
    for line in &v {
        // New row by way of empty line
        if line.chars().count() == 0 {
            page.add_row(Row::new());
            continue;
        }

        let mut form_mode = false; // If user toggled form mode
        let mut col_index: u8 = 1; // Keeping tracking of column index
        let mut working = Working::new('\0', Type::Unknown);

        // Iterate over characters in string
        for (i, c) in line.chars().enumerate() {
            // Match c against known tokens
            match c {
                '-' if i == 0 => {
                    page.get_current_row().get_col(col_index-1).append_str(
                        Working::new('\0', Type::Hr)
                            .compile(&template).as_str()
                        );
                    break;
                },

                // New page
                '#' if i == 0 => {
                    pages.push(page);
                    page = Page::new(line[1..].to_string());
                    break;
                },
                //
                // Controlling form modes
                '|' => {
                    let mut row = page.get_current_row();
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
                c if c == '[' && form_mode => working = Working::new(']', Type::Unknown),
                c if c == '{' && form_mode => working = Working::new('}', Type::Label),
                c if c == '(' && form_mode => working = Working::new(')', Type::Button),
                c if c == '<' && form_mode => working = Working::new('>', Type::Select),

                // Handle input type and we are waiting for mode qualifier
                c if c != working.until
                    && working.until != '\0'
                    && working.work_type == Type::Unknown =>
                        match c {
                            'o' => working.work_type = Type::Input(InputType::Radio),
                            '/' => working.work_type = Type::Input(InputType::Checkbox),
                            '+' => working.work_type = Type::Input(InputType::Textarea),
                            c => {
                                working.work_type = Type::Input(
                                    InputType::Text);
                                working.append(c);
                            },
                        },

                // Handle the case we are working towards the end goal of an
                // incoming char
                c if c == working.until => {
                    page.get_current_row().get_col(col_index-1)
                        .append_str(working.compile(&template).as_str());
                    working = Working::new('\0', Type::Unknown);
                },
                
                
                // In any other scenario we are building strings
                _ => {
                    if working.is_working() {
                        working.append(c);
                    } else {
                        page.get_current_row().get_col(col_index-1).append(c)
                    }
                }
            }
        }
    }

    pages.push(page);

    print!("{}", &template.head);

    for page in pages {
        print!("{}", page.out(&template));
    }

    println!("{}", &template.foot);
}
