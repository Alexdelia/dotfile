pub fn start_end(start: &str, end: &str) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });
    if start.len() + end.len() >= cols as usize {
        println!("{start}");
        crate::print::end(end);
    } else {
        println!("\r{end:>w$}\r{start}", w = cols as usize);
    }
}

pub fn end(s: &str) {
    let termsize::Size { cols, .. } =
        termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

    print!("\r{s:>w$}", w = cols as usize);
}
