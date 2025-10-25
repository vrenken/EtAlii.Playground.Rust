use std::{fs, path::{Path, PathBuf}};
use regex::Regex;

/// Custom delimiters — you can modify these!
const EXPR_START: &str = r"\{\{";
const EXPR_END: &str = r"\}\}";
const BLOCK_START: &str = r"<!--\{%";
const BLOCK_END: &str = r"%\}-->";
const COMMENT_START: &str = r"<!--\{!";
const COMMENT_END: &str = r"!\}-->";

fn main() {
    let raw_dir = Path::new("templates-raw");
    let out_dir = Path::new("templates");

    // Build triggering
    println!("cargo:rerun-if-changed=templates-raw");

    // Safe creation of folders
    if !raw_dir.exists() {
        fs::create_dir_all(raw_dir).expect("Failed to create templates-raw folder");
        println!("Created missing templates-raw folder");
    }
    fs::create_dir_all(out_dir).expect("Failed to create templates folder");

    // Process all templates
    println!("Processing templates...");
    process_directory(raw_dir, out_dir);
    println!("Template processing completed.");
}

/// Recursively process templates-raw → templates
fn process_directory(src: &Path, dst: &Path) {
    for entry in fs::read_dir(src).unwrap_or_else(|_| panic!("Failed to read directory {}", src.display())) {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_dir() {
            let new_dir = dst.join(path.file_name().unwrap());
            fs::create_dir_all(&new_dir).unwrap();
            process_directory(&path, &new_dir);
        } else if path.is_file() {
            let out_file = dst.join(path.file_name().unwrap());
            process_template_file(&path, &out_file);
        }
    }
}

/// Transform a template file
fn process_template_file(input: &PathBuf, output: &PathBuf) {
    let contents = fs::read_to_string(input).unwrap();

    // Regex for each delimiter type
    let expr_re = Regex::new(&format!(r"{}(.*?){}", EXPR_START, EXPR_END)).unwrap();
    let block_re = Regex::new(&format!(r"{}(.*?){}", BLOCK_START, BLOCK_END)).unwrap();
    let comment_re = Regex::new(&format!(r"{}(.*?){}", COMMENT_START, COMMENT_END)).unwrap();

    let mut processed = contents;

    // Apply transformations
    processed = expr_re.replace_all(&processed, "{{$1}}").into_owned();
    processed = block_re.replace_all(&processed, "{%$1%}").into_owned();
    processed = comment_re.replace_all(&processed, "{#$1#}").into_owned();

    if input.file_name().unwrap().to_str().unwrap() != "layout.html" {

        let start_pieces = processed.split("<body>").collect::<Vec<&str>>();
        processed = start_pieces[1].split("</body>").collect::<Vec<&str>>()[0].to_string();
        processed = processed.trim().to_string();
    }

    // Basic validation — check if any unmatched delimiters remain
    if processed.contains("[[") || processed.contains("[!") || processed.contains("[%") {
        panic!(
            "Unclosed or malformed custom delimiters found in {}",
            input.display()
        );
    }

    fs::write(output, processed).unwrap();
}
