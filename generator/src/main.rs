#[macro_use]
extern crate lazy_static;

use tera::Tera;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
enum Row {
    Link {
        ico: String,
        lbl: String,
        url: String,
    },
    Separator
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    name: String,
    rows: Vec<Row>
}

#[derive(Debug, Deserialize, Serialize)]
struct Column {
    categories: Vec<Category>
}

#[derive(Debug, Deserialize, Serialize)]
struct Page {
    name: String,
    columns: Vec<Column>
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

fn get_pages() -> Vec<Page> {
    let mut file = File::open("../startpages.yaml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to to read file");

    let pages: Vec<Page> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    pages
}

fn generate_page(pages: Vec<Page>) {
    println!("{:?}", pages);
    let mut context = tera::Context::new();
    context.insert("pages", &pages);

    let rendered = TEMPLATES.render("foo.html", &context).unwrap();
    println!("{}", rendered);
}

fn main() {
    let pages = get_pages();
    // for link in links {
    //     println!("Icon: {}, label: {}, url: {}", link.icon, link.label, link.url);
    // }
    generate_page(pages);

}

