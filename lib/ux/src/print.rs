use std::fmt::Display;

pub fn start_end(start: impl Display, end: impl Display) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

    let s = format!("{start}");
    let e = format!("{end}");

    if s.len() + e.len() >= cols as usize {
        println!("\r{s}\n{e:>w$}", w = cols as usize);
    } else {
        println!("\r{e:>w$}\r{s}", w = cols as usize);
    }
}

pub fn end(s: impl Display) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

    print!("\r{s:>w$}", w = cols as usize);
}
