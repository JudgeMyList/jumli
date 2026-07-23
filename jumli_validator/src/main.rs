use std::{env, fs::File, io::BufReader, path::PathBuf};

use anyhow::bail;
use jumli_gen::sources::jumli_data::{DatasetFile, RON_OPTIONS};
use thiserror::Error;
use tracing::{error, info};

use crate::Error::{InvalidArguments, IoError, ParsingFailures};

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments provided.")]
    InvalidArguments,
    #[error("IO Error: {0}")]
    IoError(std::io::Error),
    #[error("Failed to parse one or more JuMLi dataset files.")]
    ParsingFailures,
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    let records_dir = if let Some(path) = env::args().skip(1).next() {
        PathBuf::from(path)
    } else {
        error!("Missing required argument.\nExpected: jumli_validator <records_dir>");
        bail!(InvalidArguments);
    };

    let mut read_dir = std::fs::read_dir(&records_dir).map_err(|x| IoError(x))?;
    let mut errors = vec![];

    while let Some(Ok(entry)) = read_dir.next() {
        info!("Attempting to parse {}", entry.path().display());

        let reader = BufReader::new(File::open(entry.path()).map_err(|e| IoError(e))?);

        if let Err(e) = RON_OPTIONS.from_reader::<BufReader<File>, DatasetFile>(reader) {
            errors.push(format!("Unable to parse dataset {:?}: {e}", entry.path()));
        }
    }

    if errors.len() == 0 {
        info!("Everything seems to be in order.");
        return Ok(());
    }

    for error in errors {
        error!("{}", error);
    }

    bail!(ParsingFailures)
}
