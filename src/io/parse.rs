use crate::escape::{BWARNING as W, GRE as G, MUT as M, RESET as D};
use crate::io::{ESCAPE_HYPERLINK_E as EHE, ESCAPE_HYPERLINK_S as EHS, URL};
use crate::symlink::DEFAULT_SYMLINK_FILE;
use miette::Diagnostic;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("could not {W}read {M}{file}{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} is the file that define symlink")
    )]
    ReadError {
        #[source]
        source: std::io::Error,
        #[source_code]
        file: String,
    },

    #[error("could not {W}parse {M}{file}{D} to {W}toml{D} table")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} should be a {G}valid toml{D} file
you can check the example file \x1b[32;1m{DEFAULT_SYMLINK_FILE}\x1b[0m {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}
or the {G}toml{D} documentation {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
    )]
    ParseError {
        #[source]
        source: toml::de::Error,
        #[source_code]
        file: String,
    },
}

pub fn read<P>(file: P) -> Result<toml::map::Map<String, toml::Value>, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    Ok(toml::from_str(
        &fs::read_to_string(&file).map_err(|e| ParseError::ReadError {
            source: e,
            file: file.to_string(),
        })?,
    )
    .map_err(|e| ParseError::ParseError {
        source: e,
        file: file.to_string(),
    })?)
}
