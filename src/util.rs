use std::fs::File;
use std::io::prelude::*;

pub async fn save_file(text: &String, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(text.as_bytes())
}