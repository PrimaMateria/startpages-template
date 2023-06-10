#[macro_use]
extern crate lazy_static;

use rsass::{compile_scss_path, output};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

//  <name, filename>
type Navigation = HashMap<String, String>;

static OUT_DIR: &str = "docs";
static CONFIGURATION: &str = "content/startpages.yaml";

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

/// It parses YAML configuration into internal Rust structure.
fn get_startpages() -> Vec<Startpage> {
    let mut file = File::open(CONFIGURATION).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to to read file");

    let startpages: Vec<Startpage> = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    startpages
}

/// It collects names of startpages and returns a map to their file paths.
fn get_navigation(startpages: &Vec<Startpage>) -> Navigation {
    let mut navigation: Navigation = HashMap::new();

    for startpage in startpages {
        let startpage_name = startpage.name.to_owned();
        let startpage_safe_name = startpage_name.to_lowercase().replace(" ", "_");
        let startpage_file_name = format!("{}.html", startpage_safe_name);

        navigation.insert(startpage_name, startpage_file_name);
    }
    navigation
}

/// Recreates output directory.
fn prepare_out_dir() {
    if fs::metadata(OUT_DIR).is_ok() {
        fs::remove_dir_all(OUT_DIR).expect("Failed to delete docs directory");
    }
    fs::create_dir(OUT_DIR).expect("Failed to create docs directory");
}

/// For each startpage it generates a HTML file from Tera template.
/// Each startpage contains navigation to other startpages
fn generate_startpages(startpages: &Vec<Startpage>, navigation: &Navigation) {
    for startpage in startpages {
        // Use tera to generate the startpage html code
        let mut context = tera::Context::new();
        context.insert("startpage", &startpage);
        context.insert("navigation", &navigation);
        let html_code = TEMPLATES
            .render("startpage.html", &context)
            .expect("Failed to generate startpage from the template");

        // Write generatesd html code to the file in the docs directory
        let startpage_file_name = navigation.get(&startpage.name).unwrap();
        let startpage_file_path = format!("{}/{}", OUT_DIR, startpage_file_name);
        let mut file = File::create(startpage_file_path).expect("Failed to create output file");
        file.write_all(html_code.as_bytes())
            .expect("Failed to write to output file");
    }
}

fn compile_sass() {
    let css_dir = format!("{}/css", OUT_DIR);
    fs::create_dir(&css_dir).expect("Failed to create css directory");

    let path = "sass/styles.scss".as_ref();
    let format = output::Format {
        ..Default::default()
    };
    let css = compile_scss_path(path, format).unwrap();

    let css_file_path = format!("{}/styles.css", css_dir);
    let mut file = File::create(css_file_path).expect("Failed to create css file");
    file.write_all(&css.as_slice())
        .expect("Failed to write css file");
}

fn main() {
    let startpages = get_startpages();
    let navigation = get_navigation(&startpages);

    prepare_out_dir();
    generate_startpages(&startpages, &navigation);
    compile_sass();
}
