use std::{fs::File, io::Read, path::Path};

pub fn read_file_content(path: &Path) -> Result<String, std::io::Error> {
    let content = {
        let mut file = File::open(path)?;

        let mut out = String::new();

        file.read_to_string(&mut out)?;

        out
    };

    Ok(content)
}

pub fn write_file_content(s: &str, output_path: &Path) -> std::io::Result<usize> {
    let mut file = std::fs::File::create(output_path)?;

    std::io::Write::write(&mut file, s.as_bytes())
}

pub fn replace_file_content(
    path: &Path,
    content_to_replace: &str,
    new_content: &str,
) -> std::io::Result<()> {
    let file_content = read_file_content(path)?;

    let new_file_content = file_content.replace(content_to_replace, new_content);

    write_file_content(&new_file_content, path)?;

    Ok(())
}
