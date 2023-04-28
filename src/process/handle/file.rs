use crate::ansi::{BW, M, VALID};
use crate::unix::{remove_file, Symlink};
use ansi::abbrev::D;
use ansi::{B_BLUE, B_RED};
use ux::{ask, ask_yn, AskKey};

use std::fs::File;
use std::io::{BufReader, Read};

pub fn handle_file(symlink: &Symlink, interactive: bool) -> std::io::Result<()> {
    println!(
        "found already existing file:
\t{M}{path}{D}",
        path = symlink.path.display()
    );

    if !diff(symlink, interactive)? {
        return Ok(());
    }

    if interactive
        && !ask_yn(
            &format!(
                "should {M}{path}{D} be replaced with the symlink:
\t{M}{path}{D} -> {VALID}{target}{D}
?",
                path = symlink.path.display(),
                target = symlink.target.display(),
            ),
            true,
        )
        .unwrap()
    {
        return Ok(());
    }

    remove_file(&symlink.path)?;
    symlink.create()
}

#[derive(PartialEq)]
enum Diff<'a, T: ToOwned + ?Sized> {
    Utf8(diffy::Patch<'a, T>),
    NonUtf8,
    Identical,
}

fn diff(symlink: &Symlink, interactive: bool) -> std::io::Result<bool> {
    let original = std::fs::read_to_string(&symlink.target);
    let modified = std::fs::read_to_string(&symlink.path);

    let diff: Diff<str> = match (&original, &modified) {
        (Ok(original), Ok(modified)) => {
            let patch = diffy::create_patch(original, modified);
            if patch.hunks().is_empty() {
                Diff::Identical
            } else {
                Diff::Utf8(patch)
            }
        }
        _ => {
            if diff_non_utf8(symlink)? {
                Diff::Identical
            } else {
                Diff::NonUtf8
            }
        }
    };

    let c: char = if diff == Diff::Identical {
        println!("both files are identical");
        Some('k')
    } else if !interactive {
        Some('I')
    } else if let Diff::Utf8(patch) = diff {
        println!(
            "diff:\n--- {B_RED}{system}{D}\n+++ {B_BLUE}{data}{D}\n{patch}\n",
            data = symlink.target.display(),
            system = symlink.path.display(),
        );
        None
    } else {
        println!(
            "{B_RED}{system}{D} and {B_BLUE}{data}{D} are not identical",
            data = symlink.target.display(),
            system = symlink.path.display()
        );
        None
    }
    .unwrap_or_else(|| {
        ask(
            &format!(
                "what should be done on {M}{path}{D}:
\t{B_BLUE}K{D}eep file in data
\t{B_RED}R{D}eplace file in data with the already existing file in the system\n",
                path = symlink.path.display(),
            ),
            &[
                AskKey::new('k', Some("keep"), true, Some(B_BLUE)),
                AskKey::new('r', Some("replace"), true, Some(B_RED)),
            ],
            None,
        )
        .unwrap() // panic on purpose
    });

    match c {
        'k' => symlink.print_action("keeped", Some(B_BLUE)),
        'I' => {
            symlink.print_action("skipped", Some(BW));
            return Ok(false);
        }
        'r' => {
            std::fs::copy(&symlink.target, &symlink.path)?;
            symlink.print_action("replaced", Some(B_RED));
        }
        _ => unreachable!("diff() should find char: k, r or I"),
    }

    Ok(true)
}

fn diff_non_utf8(symlink: &Symlink) -> std::io::Result<bool> {
    let original = File::open(&symlink.target)?;
    let modified = File::open(&symlink.path)?;

    if original.metadata()?.len() != modified.metadata()?.len() {
        return Ok(false);
    }

    let original = BufReader::new(original);
    let modified = BufReader::new(modified);

    for (b1, b2) in original.bytes().zip(modified.bytes()) {
        if b1? != b2? {
            return Ok(false);
        }
    }

    Ok(true)
}
