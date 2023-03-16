use std::collections::HashSet;
use std::fmt::Display;
use std::io::Write;

pub struct AskKey {
    pub key: char,
    pub description: Option<String>,
    pub alt: bool,
    pub color: Option<String>,
}

impl AskKey {
    pub fn new(
        key: char,
        description: Option<impl Into<String>>,
        alt: bool,
        color: Option<impl Into<String>>,
    ) -> Self {
        Self {
            key,
            description: description.map(|d| d.into()),
            alt,
            color: color.map(|c| c.into()),
        }
    }
}

impl From<char> for AskKey {
    fn from(key: char) -> Self {
        Self {
            key,
            description: None,
            alt: true,
            color: None,
        }
    }
}

impl Display for AskKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = String::from("\x1b[1m");
        let c = self.color.as_ref().unwrap_or(&binding);

        if self.alt {
            write!(
                f,
                "{c}{}\x1b[0m|{c}{}\x1b[0m",
                self.key.to_ascii_lowercase(),
                self.key.to_ascii_uppercase()
            )?;
        } else {
            write!(f, "{c}{}\x1b[0m", self.key)?;
        }

        if let Some(description) = &self.description {
            write!(f, " ({c}{}\x1b[0m)", description)?;
        }

        Ok(())
    }
}

pub fn ask(question: &str, key: &[AskKey], enter_redirect: Option<char>) -> std::io::Result<char> {
    let key_set = ask_question(question, key, enter_redirect)?;

    let g = getch::Getch::new();

    loop {
        let c = g.getch()? as char;
        if c != '\n' {
            println!();
        }

        if key_set.contains(&c) {
            return Ok(c);
        }

        ask_keys(c, key)?;
    }
}

fn ask_question(
    question: &str,
    key: &[AskKey],
    enter_redirect: Option<char>,
) -> std::io::Result<HashSet<char>> {
    let mut key_set: HashSet<char> = HashSet::new();
    if enter_redirect.is_some() {
        key_set.insert('\n');
    }

    let b = String::from("\x1b[1m");

    let mut s = format!("{question}\x1b[0m [");
    for AskKey {
        key, alt, color, ..
    } in key
    {
        s.push_str(format!("{color}{key}\x1b[0m/", color = color.as_ref().unwrap_or(&b)).as_str());

        if !*alt {
            key_set.insert(*key);
        } else {
            key_set.insert(key.to_ascii_lowercase());
            key_set.insert(key.to_ascii_uppercase());
        }
    }
    s.pop();
    print!("{s}] ");
    std::io::stdout().flush()?;

    Ok(key_set)
}

fn ask_keys(ch: char, key: &[AskKey]) -> std::io::Result<()> {
    let b = String::from("\x1b[1m");

    let mut s = String::from("wating for ");
    for AskKey {
        key,
        alt,
        description,
        color,
    } in key
    {
        let c = color.as_ref().unwrap_or(&b);

        if *alt {
            s.push_str(format!("{c}{key}\x1b[0m").as_str());
        } else {
            s.push_str(
                format!(
                    "{c}{low}\x1b[0m|{c}{up}\x1b[0m",
                    low = key.to_ascii_lowercase(),
                    up = key.to_ascii_uppercase(),
                )
                .as_str(),
            );
        }

        if let Some(description) = description {
            s.push_str(format!(" ({c}{description}\x1b[0m)").as_str());
        }

        s.push_str(", ");
    }
    s.pop();
    s.pop();
    s.push_str(format!("\n\tnot '\x1b[1m{ch}\x1b[0m'").as_str());
    std::io::stdout().flush()
}
