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
struct InputTextConfig {
    name: String,
    placeholder: String,
    value: String
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
    a: String,
    select: String ,
    bind: String
}
impl Template {
    fn format(s: &str) -> String {
        s.trim().to_string()
    }
    fn from_vec(&mut self, vec: &Vec<&str>) {
        if !vec[0].is_empty() { self.head = Template::format(vec[0]) }
        if !vec[1].is_empty() { self.foot = Template::format(vec[1]) }
        if !vec[2].is_empty() { self.container_start = Template::format(vec[2]) }
        if !vec[3].is_empty() { self.container_end = Template::format(vec[3]) }
        if !vec[4].is_empty() { self.row_start = Template::format(vec[4]) }
        if !vec[5].is_empty() { self.row_end = Template::format(vec[5]) }
        if !vec[6].is_empty() { self.col_start = Template::format(vec[6]) }
        if !vec[7].is_empty() { self.col_end = Template::format(vec[7]) }
        if !vec[8].is_empty() { self.segment_start = Template::format(vec[8]) }
        if !vec[9].is_empty() { self.segment_end = Template::format(vec[9]) }
        if !vec[10].is_empty() { self.label = Template::format(vec[10]) }
        if !vec[11].is_empty() { self.text = Template::format(vec[11]) }
        if !vec[12].is_empty() { self.checkbox = Template::format(vec[12]) }
        if !vec[13].is_empty() { self.radio = Template::format(vec[13]) }
        if !vec[14].is_empty() { self.textarea = Template::format(vec[14]) }
        if !vec[15].is_empty() { self.hr = Template::format(vec[15]) }
        if !vec[16].is_empty() { self.button = Template::format(vec[16]) }
        if !vec[17].is_empty() { self.a = Template::format(vec[17]) }
        if !vec[18].is_empty() { self.select = Template::format(vec[18]) }
        if !vec[19].is_empty() { self.bind = Template::format(vec[19]) }
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
enum ButtonType {
    Unknown,
    Button,
    A
}

#[derive(PartialEq)]
enum Type {
    Unknown,
    Input(InputType),
    Button(ButtonType),
    Label,
    Hr,
    Select,
    Bind
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
            Type::Button(ref t) => match t {
                &ButtonType::Button => template.button.as_str(),
                &ButtonType::A => template.a.as_str(),
                _ => ""
            },
            Type::Label => template.label.as_str(),
            Type::Bind => template.bind.as_str(),
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
            Type::Input(InputType::Text) => {
                let split: Vec<String> = self.str.split("?").map(|s| s.to_string()).collect();
                let value = split[0].trim().to_owned();
                let placeholder = if split.len() > 1 { split[1].trim().to_owned() } else { "".to_string() };

                let split: Vec<String> = self.str.split("->").map(|s| s.to_string()).collect();

                to_json(&InputTextConfig {
                    name: if split.len() > 1 { split[1].trim().to_owned() } else { "".to_string()} ,
                    value: value,
                    placeholder: placeholder
                })
            },
            Type::Button(_) => {
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
    let mut template = Template {
        head: "<script src=\"https://code.jquery.com/jquery-3.2.1.min.js\" integrity=\"sha256-hwg4gsxgFZhOsEEamdOYGBf13FyQuiTwlAQgxVSNgt4=\" crossorigin=\"anonymous\"></script> \
            <style>[txtof-container] {display: none} :target {display: block}</style>
        ".to_string(),
        foot: "<script>$(function(){ \
                  if (!window.location.hash) { \
                    localStorage.removeItem('txtof'); \
                    window.location.hash = 'default'; \
                  } \
                  $(window).on('hashchange', function(e) { \
                    var data = JSON.parse(localStorage.getItem('txtof')) || {}; \
                    for (var key in data) { \
                      $('[name=\"'+key+'\"]').val(data[key]); \
                      $('[data-bind=\"'+key+'\"]').html(data[key]); \
                    } \
                  }); \
                  $(document).on('click', '[data-target]', function(){ \
                    var data = JSON.parse(localStorage.getItem('txtof')) || {}; \
                    data = $(this).closest('[txtof-container]').find(':input').serializeArray().reduce(function(m,o){m[o.name] = o.value; return m;}, data); \
                    localStorage.setItem('txtof', JSON.stringify(data)); \
                    window.location.hash = $(this).data('target');  \
                  }); \
        });</script>".to_string(),
        container_start: "<div txtof-container id=\"{{value}}\">".to_string(),
        container_end: "</div>".to_string(),
        row_start: "<div>".to_string(),
        row_end: "</div>".to_string(),
        col_start: "<span>".to_string(),
        col_end: "</span>".to_string(),
        segment_start: "".to_string(),
        segment_end: "<br/>".to_string(),
        label: "<label>{{value}}</label>".to_string(),
        text: "<input name=\"{{name}}\" type=\"text\" placeholder=\"{{placeholder}}\" value=\"{{value}}\"/>".to_string(),
        checkbox: "<input type=\"checkbox\"/>".to_string(),
        radio: "<input type=\"radio\"/>".to_string(),
        textarea: "<textarea>{{value}}</textarea>".to_string(),
        hr: "<hr/>".to_string(),
        button: "<button data-target=\"{{trigger}}\">{{value}}</button>".to_string(),
        a: "<a href=\"#{{trigger}}\">{{value}}</a>".to_string(),
        select: "<select>{{#each value}}<option>{{this}}</option>{{/each}}</select>".to_string(),
        bind: "<span data-bind=\"{{value}}\"></span>".to_string()
    };
    match env::args().nth(1) {
        Some(file) => {
            let mut in_file = match File::open(&file) {
                Err(_) => panic!("Unable to open template file"),
                Ok(file) => file,
            };

            let mut content = String::new();
            in_file.read_to_string(&mut content)
                .expect("Unable to process template file");

            let sp: Vec<&str> = content.split("\n").collect();

            template.from_vec(&sp);
        },
        _ => match env::var("template") {
            Ok(val) => {
                let tv: Vec<&str> = val.split(",").collect();
                template.from_vec(&tv);
            },
            _ => {}
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
                // Separator, ignore
                '=' if i == 0 => break,

                // Horizontal divider
                '-' if i == 0 => {
                    page.get_current_row().get_col(col_index-1).append_str(
                        Working::new('\0', Type::Hr)
                            .compile(&template).as_str());
                    break;
                },

                // New page
                '#' if i == 0 => {
                    pages.push(page);
                    page = Page::new(line[1..].to_string());
                    break;
                },

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
                c if c == '(' && form_mode => working = Working::new(')', Type::Button(ButtonType::Unknown)),
                c if c == '<' && form_mode => working = Working::new('>', Type::Select),
                c if c == '%' && form_mode => working = Working::new(' ', Type::Bind),

                // Handle button type and we are waiting for mode qualifier
                c if c != working.until
                    && working.until != '\0'
                    && working.work_type == Type::Button(ButtonType::Unknown) =>
                        match c {
                            '#' => working.work_type = Type::Button(ButtonType::A),
                            c => {
                                working.work_type = Type::Button(ButtonType::Button);
                                working.append(c);
                            },
                        },

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
