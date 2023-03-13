use crate::ansi::{BE, BW, C, E, EHE, EHS, H, M, V, W};
use crate::symlink::DEFAULT_SYMLINK_FILE;
use ansi::abbrev::{B, BLU, D, F, YEL};
use const_format::formatcp;
use miette::{Diagnostic, IntoDiagnostic, NamedSource, SourceSpan};
use std::fs;
use thiserror::Error;

const URL: &str = "https://github.com/Alexdelia/dotfile/tree/main/";
const HELP_EXAMPLE: &str = formatcp!("you can check the example file {V}{DEFAULT_SYMLINK_FILE}{D} {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}");

#[derive(Error, Diagnostic, Debug)]
#[error("{error}")]
#[diagnostic(
	code(parse::ParseTomlError),
    url("{URL}{origin_file}"),
    help("{advice}

{H}example:{D}
{F}```toml{D}
{C}# optional title{D}
{B}{YEL}[title]{D}
{C}# optional update frequency (default: 'always'){D}
{C}# key (string) = value ('always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]){D}
{BLU}update{D} = 'always'
{C}# symlink list{D}
{C}# key (string) = value (string){D}
\"~/path/to/dotfile/where/symlink/will/be\" = \"path/of/actual/dotfile/stored/in/data/directory\"
{F}```{D}
	
{HELP_EXAMPLE}")
)]
pub struct ParseTomlError {
    #[related]
    related: Vec<miette::Error>,

    #[source_code]
    file: NamedSource,
    #[label("in this table")]
    table_bit: Option<(usize, usize)>,
    #[label("wrong line")]
    line_bit: SourceSpan,

    error: String,
    advice: String,
    origin_file: String,
}

impl ParseTomlError {
    pub fn new(
        file: String,
        title: Option<&str>,
        key: &str,
        error: String,
        advice: String,
        origin_file: String,
        related: Option<Vec<miette::Error>>,
    ) -> Self {
        let content =
            fs::read_to_string(&file).unwrap_or_else(|_| String::from("ENABLE TO READ FILE\n"));

        let mut t: Option<(usize, usize)> = None;

        if let Some(title) = title {
            let ts = content.find(&title).unwrap_or(0);
            let te = content[ts..]
                .find(|c: char| c.is_whitespace() || c == ']')
                .unwrap_or(content.len() - ts);

            t = Some((ts, te));
        }

        let te = t.map(|(ts, te)| ts + te).unwrap_or(0);
        let ks = content[te..].find(key).unwrap_or(0) + te;
        let ke = content[ks..].find('\n').unwrap_or(content.len() - ks);

        Self {
            related: related.unwrap_or_else(Vec::new),
            file: NamedSource::new(file, content),
            table_bit: t,
            line_bit: (ks, ke).into(),
            error,
            advice,
            origin_file,
        }
    }
}

#[derive(Error, Diagnostic, Debug)]
#[error("could not {BW}read {M}{file}{D}")]
#[diagnostic(
    code(parse::read),
    url("{URL}{origin_file}"),
    help("the file {M}{file}{D} is the file that define symlink")
)]
pub struct ReadError {
    #[source]
    pub source: std::io::Error,
    pub file: String,
    pub origin_file: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("could not {BW}parse {M}{file}{D} to {BW}toml{D}")]
#[diagnostic(
    code(parse::str_to_toml),
    url("{URL}{origin_file}"),
    help(
        "the file {M}{file}{D} should be a {V}valid toml{D} file
{HELP_EXAMPLE}
or the {V}toml{D} documentation {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}"
    )
)]
pub struct ParseStrToTOMLError {
    #[source]
    pub source: toml::de::Error,
    pub file: String,
    pub origin_file: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("could not {BW}parse {M}{file}{D} to a valid list of {BW}symlink{D}")]
#[diagnostic(
    code(parse::toml_to_env),
    url("{URL}{origin_file}"),
    help(
        "the file {M}{file}{D} should be a {V}valid toml{D} that define the {V}symlink{D}s
{HELP_EXAMPLE}"
    )
)]
pub struct ParseTOMLToEnvError {
    pub file: String,
    pub origin_file: String,
}

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error(
        "{reason}

could not {BW}parse {M}{file_name}{D} to valid list of {BW}symlink{D}"
    )]
    #[diagnostic(
		code(parse::value_to_T),
		url("{}{}", URL, file!()),
		help("{advice}

the file {M}{file_name}{D} should be a {V}valid toml{D} that define the {V}symlink{D}s

{H}example:{D}
{F}```{D}
{C}# optional title{D}
{B}{YEL}[title]{D}
{C}# optional update frequency (default: 'always'){D}
{C}# key (string) = value ('always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]){D}
{BLU}update{D} = 'always'
{C}# symlink list{D}
{C}# key (string) = value (string){D}
\"~/path/to/dotfile/where/symlink/will/be\" = \"path/of/actual/dotfile/stored/in/data/directory\"
{F}```{D}

{HELP_EXAMPLE}")
	)]
    ParseSymlinkWrongType {
        file_name: String,
        #[source_code]
        file: String,
        #[label]
        wrong_bit: SourceSpan,

        reason: String,
        advice: String,
    },

    #[error("in {M}{title}{D}, {E}value {BE}{value}{D} for {W}key {BW}update{D} does not represent a {BW}valid update frequency{D}")]
    #[diagnostic(
		code(parse::to_update),
		url("{}{}", URL, file!()),
        help("the {E}value{D} should represent a {V}valid update frequency{D}

{H}possible update frequency are:{D} 'always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]
{H}default:{D} 'always'

{HELP_EXAMPLE}")
	)]
    ParseUpdate {
        file_name: String,
        #[source_code]
        file: String,
        #[label("in this table")]
        table_bit: SourceSpan,
        #[label("wrong update")]
        update_bit: SourceSpan,

        title: String,
        value: String,
    },
}

impl ParseError {
    pub fn wrong_type(
        file: String,
        title: Option<&str>,
        key: &str,
        reason: String,
        advice: String,
    ) -> Self {
        let content =
            fs::read_to_string(&file).unwrap_or_else(|_| String::from("ENABLE TO READ FILE\n"));

        // will not work if the key is repeated in the file
        // and the first one is not the one that cause the error
        let s = if let Some(t) = title {
            content.find(t).unwrap_or(0)
        } else {
            0
        };
        let s = content[s..].find(key).unwrap_or(content.len() - s) + s;
        let e = content[s..].find('\n').unwrap_or(content.len() - s) + s;

        Self::ParseSymlinkWrongType {
            file_name: file,
            file: content,
            wrong_bit: (s..e).into(),
            reason,
            advice,
        }
    }

    pub fn update(file: String, title: String, value: String) -> Self {
        let content =
            fs::read_to_string(&file).unwrap_or_else(|_| String::from("ENABLE TO READ FILE\n"));

        let ts = content.find(&title).unwrap_or(0);
        let te = content[ts..]
            .find(|c: char| c.is_whitespace() || c == ']')
            .unwrap_or(content.len() - ts)
            + ts;
        let us = content[ts..].find("update").unwrap_or(0) + ts;
        let ue = content[us..].find('\n').unwrap_or(content.len() - us) + us;

        Self::ParseUpdate {
            file_name: file,
            file: content,
            table_bit: (ts..te).into(),
            update_bit: (us..ue).into(),
            title,
            value,
        }
    }
}
