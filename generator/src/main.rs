#[macro_use]
extern crate lazy_static;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tera::Tera;

#[derive(Debug, Deserialize, Serialize)]
enum Row {
    Link {
        ico: String,
        lbl: String,
        url: String,
    },
    Separator,
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    name: String,
    rows: Vec<Row>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Column {
    categories: Vec<Category>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Startpage {
    name: String,
    columns: Vec<Column>,
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

fn get_startpages() -> Vec<Startpage> {
    // Parse the config file to Rust structure
    let mut file = File::open("../startpages.yaml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to to read file");

    let startpages: Vec<Startpage> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    startpages
}

fn generate_startpages(startpages: Vec<Startpage>) {
    let out_dir = "../docs";

    // Delete docs directory if it exists
    if fs::metadata(out_dir).is_ok() {
        fs::remove_dir_all(out_dir).expect("Failed to delete docs directory");
    }

    // Create docs directory
    fs::create_dir(out_dir).expect("Failed to create docs directory");

    for startpage in startpages {
        let startpage_safe_name = startpage.name.to_lowercase().replace(" ", "_");
        let startpage_file_name = format!("{}.html", startpage_safe_name);

        // Use tera to generate the startpage html code
        let mut context = tera::Context::new();
        context.insert("startpage", &startpage);
        let html_code = TEMPLATES.render("startpage.html", &context).unwrap();

        // Write generatesd html code to the file in the docs directory
        let startpage_filepath = format!("{}/{}", out_dir, startpage_file_name);
        let mut file = File::create(startpage_filepath).expect("Failed to create output file");
        file.write_all(html_code.as_bytes())
            .expect("Failed to write to output file");
    }
}

fn main() {
    let startpages = get_startpages();
    generate_startpages(startpages);
}
