use ansi_regex::ansi_regex;
use std::borrow::Cow;
use std::fmt::Display;

pub fn start_end(start: impl Display, end: impl Display) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

    let s = format!("{start}");
    let e = format!("{end}");
    let s_len = remove(&s).len();
    let e_len = remove(&e).len();

    if s_len + e_len >= cols as usize {
        println!("\r{s}\n{n:>w$}{e}", w = cols as usize - e_len, n = "");
    } else {
        println!("\r{s}{n:>w$}{e}", w = cols as usize - e_len - s_len, n = "");
    }
}

pub fn end(s: impl Display) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

    let f = format!("{s}");
    println!("\r{n:>w$}{f}", w = cols as usize - remove(&f).len(), n = "");
}

fn remove<'t>(s: &'t str) -> Cow<'t, str> {
    ansi_regex().replace_all(s, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_end() {
        start_end("foo", "bar");
        start_end("\x1b[1m\x1b[31mfoo\x1b[0m", "\x1b[1m\x1b[32mbar\x1b[0m");
    }

    #[test]
    fn test_end() {
        end("foo");
        end("\x1b[1m\x1b[31mfoo\x1b[0m");
    }
}
