pub fn url(file: &str) -> &'static str {
    let url = format!("https://github.com/Alexdelia/dotfile/tree/main/{file}");
    Box::leak(url.into_boxed_str())
}
