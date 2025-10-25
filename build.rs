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

    println!("cargo:rerun-if-changed=templates_raw");

    fs::create_dir_all(out_dir).unwrap();

    process_directory(raw_dir, out_dir);
}

/// Recursively process templates_raw → templates
fn process_directory(src: &Path, dst: &Path) {
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let new_dir = dst.join(path.file_name().unwrap());
            fs::create_dir_all(&new_dir).unwrap();
            process_directory(&path, &new_dir);
        } else if path.is_file() {
            process_template_file(&path, &dst.join(path.file_name().unwrap()));
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

    // Basic validation — check if any unmatched delimiters remain
    if processed.contains("[[") || processed.contains("[!") || processed.contains("[%") {
        panic!(
            "Unclosed or malformed custom delimiters found in {}",
            input.display()
        );
    }

    fs::write(output, processed).unwrap();
}
