use super::*;
use tempfile::{NamedTempFile, tempfile};
use std::io::Write;

#[test]
fn find_final_dir_success() {
    let input_term = ".cargo";
    let mut tempfile = NamedTempFile::new().unwrap();

    writeln!(tempfile, "cd .cargo\ncd .ssh").unwrap();

    let final_dir = find_final_dir(input_term, tempfile.path().to_str().unwrap());

    assert_eq!(final_dir.unwrap(), String::from(format!("/home/{}/.cargo", std::env::var("USER").unwrap())));
}