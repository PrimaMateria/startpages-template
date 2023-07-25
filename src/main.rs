#[macro_use]
extern crate lazy_static;

use fs_extra::dir::{copy, CopyOptions};
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

static OUT_DIR: &str = "_site";
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
fn get_startpages() -> Result<Vec<Startpage>, Box<dyn std::error::Error>> {
    let mut file = File::open(CONFIGURATION)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let startpages: Vec<Startpage> = serde_yaml::from_str(&contents)?;

    Ok(startpages)
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
fn prepare_out_dir() -> Result<(), Box<dyn std::error::Error>> {
    if fs::metadata(OUT_DIR).is_ok() {
        fs::remove_dir_all(OUT_DIR)?;
    }
    fs::create_dir(OUT_DIR)?;

    Ok(())
}

/// For each startpage it generates a HTML file from Tera template.
/// Each startpage contains navigation to other startpages
fn generate_startpages(
    startpages: &Vec<Startpage>,
    navigation: &Navigation,
) -> Result<(), Box<dyn std::error::Error>> {
    for startpage in startpages {
        // Use tera to generate the startpage html code
        let mut context = tera::Context::new();
        context.insert("startpage", &startpage);
        context.insert("navigation", &navigation);

        let html_code = TEMPLATES.render("startpage.html", &context)?;

        // Write generated html code to the file in the _site directory
        let startpage_file_name = navigation
            .get(&startpage.name)
            .ok_or("Navigation doesn't contain startpage name")?;

        let startpage_file_path = format!("{}/{}", OUT_DIR, startpage_file_name);
        let mut file = File::create(startpage_file_path)?;

        file.write_all(html_code.as_bytes())?;
    }

    Ok(())
}

/// Compiles sass/styles.scss to _site/css/styles.css
fn compile_sass() -> Result<(), Box<dyn std::error::Error>> {
    let css_dir = format!("{}/css", OUT_DIR);
    fs::create_dir(&css_dir)?;

    let path = "sass/styles.scss".as_ref();
    let format = output::Format {
        ..Default::default()
    };
    let css = compile_scss_path(path, format)?;

    let css_file_path = format!("{}/styles.css", css_dir);
    let mut file = File::create(css_file_path)?;

    file.write_all(&css.as_slice())?;

    Ok(())
}

/// Copies public dir to _site/public
fn copy_public_dir() -> Result<(), Box<dyn std::error::Error>> {
    let options = CopyOptions::new();
    copy("public", OUT_DIR, &options)?;

    Ok(())
}

fn main() {
    let startpages = get_startpages().expect("Failed to parse startpages content");
    println!("Startpages content parsed");

    let navigation = get_navigation(&startpages);
    println!("Navigation extracted from the content");

    prepare_out_dir().expect("Failed to prepare output directory");
    println!("Output directory prepared");

    generate_startpages(&startpages, &navigation).expect("Failed to generate startpages");
    println!("Startpages generated");

    compile_sass().expect("Failed to compile Sass");
    println!("Sass styles compiled");

    copy_public_dir().expect("Failed to copy public directory");
    println!("Public directory copied");
}
