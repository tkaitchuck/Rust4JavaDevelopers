use skeptic::*;

fn main() {
    // Add all markdown files in directory "src/".
    let mdbook_files = markdown_files_of_directory("src/");
    generate_doc_tests(&mdbook_files);
}