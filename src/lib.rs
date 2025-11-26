pub mod drivers;

use std::path::Path;

use thiserror::Error;

pub struct CsvRecords {
    pub header: Vec<String>,
    pub records: Vec<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum ParseCsvFileError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Expected a vocab record to have {0} fields but got {1} for line: {2}")]
    FieldCountMismatch(usize, usize, String),

    #[error("Csv content must not be empty")]
    ContentEmpty,
}

pub fn parse_csv_file(filepath: &Path) -> Result<CsvRecords, ParseCsvFileError> {
    type FnErr = ParseCsvFileError;

    let content = drivers::file_io::read_file_content(filepath)?;

    let first_line = content.lines().next().ok_or(FnErr::ContentEmpty)?;

    let heading_tokens = first_line
        .split(",")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let tokens_per_line = {
        content
            .lines()
            .skip(1)
            .map(|line| {
                let tokens = line.split(",").map(|s| s.to_owned()).collect::<Vec<_>>();

                if tokens.len() != heading_tokens.len() {
                    return Err(FnErr::FieldCountMismatch(
                        heading_tokens.len(),
                        tokens.len(),
                        line.to_owned(),
                    ));
                }

                Ok(tokens)
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    Ok(CsvRecords {
        header: heading_tokens,
        records: tokens_per_line,
    })
}

pub fn get_random_csv_record(records: &CsvRecords) -> Option<&[String]> {
    if records.records.is_empty() {
        return None;
    }

    let randint = rand::random_range(0..records.records.len());

    Some(&records.records[randint])
}

pub fn get_csv_record_column(record: &[String], header: &[String], column: &str) -> Option<String> {
    let n = header.iter().enumerate().find(|(_, s)| *s == column)?.0;

    if record.len() <= n {
        return None;
    }

    Some(record[n].to_owned())
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static G_INIT_ONCE: Once = Once::new();

    fn init() {
        G_INIT_ONCE.call_once(|| {
            let _ = crate::drivers::logging::init_logging_with_level(log::LevelFilter::Trace);
        });
    }

    #[test]
    fn test_add() {
        init();

        assert_eq!(add(1, 1), 2);
    }
}
