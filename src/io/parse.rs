use crate::escape::{B, BE, BLU, BW, C, D, E, EHE, EHS, F, H, M, V, W, YEL};
use crate::io::URL;
use crate::symlink::{Env, EnvType, Grouped, Symlink, DEFAULT_SYMLINK_FILE};
use const_format::formatcp;
use miette::{Diagnostic, SourceSpan};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

const HELP_EXAMPLE: &str = formatcp!("you can check the example file {V}{DEFAULT_SYMLINK_FILE}{D} {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}");

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("could not {BW}read {M}{file}{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} is the file that define symlink")
    )]
    Read {
        #[source]
        source: std::io::Error,
        file: String,
    },

    #[error("could not {BW}parse {M}{file}{D} to {BW}toml{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} should be a {V}valid toml{D} file
{HELP_EXAMPLE}
or the {V}toml{D} documentation {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
    )]
    ParseStrToTOML {
        #[source]
        source: toml::de::Error,
        file: String,
    },

    #[error("could not {BW}parse {M}{file}{D} to valid list of {BW}symlink{D}")]
    #[diagnostic(
		code(parse::toml_to_env),
		url("{}{}", URL, file!()),
		help("the file {M}{file}{D} should be a {V}valid toml{D} that define the {V}symlink{D}s
{HELP_EXAMPLE}")
	)]
    ParseTOMLToEnv { file: String },

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
}

impl ParseError {
    pub fn wrong_type(file: String, key: &str, reason: String, advice: String) -> Self {
        let content =
            fs::read_to_string(&file).unwrap_or_else(|_| String::from("ENABLE TO READ FILE\n"));

        // will not work if the key is repeated in the file
        // and the first one is not the one that cause the error
        let s = content.find(key).unwrap_or(0);
        let e = content[s..].find('\n').unwrap_or(content.len() - s) + s;

        Self::ParseSymlinkWrongType {
            file_name: file,
            file: content,
            wrong_bit: (s..e).into(),
            reason,
            advice,
        }
    }
}

pub fn parse<P>(file: P) -> Result<Env, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let env = toml_to_env(file.to_string().as_str(), read(file)?)?;

    Ok(env)
}

fn read<P>(file: P) -> Result<toml::Value, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    Ok(
        toml::from_str(&fs::read_to_string(&file).map_err(|e| ParseError::Read {
            source: e,
            file: file.to_string(),
        })?)
        .map_err(|e| ParseError::ParseStrToTOML {
            source: e,
            file: file.to_string(),
        })?,
    )
}

fn toml_to_env(file: &str, toml: toml::Value) -> Result<Env, ParseError> {
    let table = toml.as_table().ok_or_else(|| ParseError::ParseTOMLToEnv {
        file: file.to_string(),
    })?;

    let mut env = Env::new();

    for (k, v) in table {
        match v {
            toml::Value::Table(table) => env.push(EnvType::Grouped(table_to_grouped(file, k.to_owned(), table.to_owned())?)),
            toml::Value::String(string) => env.push(EnvType::Alone(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            })),
            _ => {
                return Err(ParseError::wrong_type(
                    file.to_string(),
                    k,
                    format!(
                        "{E}value {BE}{v}{D} for {W}key {BW}{k}{D} is not a {BW}string{D} or {BW}table{D}"
                    ),
                    format!(
                        "the {E}value{D} should either be:
	- a {V}string{D} that represent the path to the actual dotfile
	- a {V}table{D} that represent a list of symlink"
                    ),
                ))
            }
        }
    }

    Ok(env)
}

fn table_to_grouped(file: &str, title: String, table: toml::Table) -> Result<Grouped, ParseError> {
    let mut symlink: Vec<Symlink> = Vec::new();

    for (k, v) in table {
        if k == "update" {
            continue;
        }

        match v {
            toml::Value::String(string) => symlink.push(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            }),
            _ => {
                return Err(ParseError::wrong_type(
                    file.to_string(),
                    k,
                    format!("{E}value {BE}{v}{D} for {W}key {BW}{k}{D} is not a {BW}string{D}"),
                    format!(
						"the {E}value{D} should be a {V}string{D} that represent the path to the actual dotfile"
					),
                ))
            }
        }
    }
}

// return Err(ParseError::wrong_type(
// 	file.to_string(),
// 	k.as_str(),
// 	format!("{E}value {BE}{v}{D} for {W}key {BW}{k}{D} is not a {BW}string{D}"),
// 	format!("the {E}value{D} should be a {V}string{D} that represent the path to the actual dotfile")
// ))
