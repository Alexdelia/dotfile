use ansi::{B_GREEN, B_MAGENTA, B_RED, B_YELLOW, CYAN, RED, YELLOW};

pub const ERROR: &str = RED;
pub const E: &str = ERROR;
pub const BERROR: &str = B_RED;
pub const BE: &str = BERROR;

pub const WARNING: &str = YELLOW;
pub const W: &str = WARNING;
pub const BWARNING: &str = B_YELLOW;
pub const BW: &str = BWARNING;

pub const VALID: &str = B_GREEN;
pub const V: &str = VALID;
pub const MUT: &str = B_MAGENTA;
pub const M: &str = MUT;

pub const HELP: &str = CYAN;
pub const H: &str = HELP;
pub const COMMENT: &str = "\x1b[32;2;3m";
pub const C: &str = COMMENT;

// pub const LINK: &str = "\x1b[36;1;4m";
// pub const L: &str = LINK;

pub const ESCAPE_HYPERLINK_S: &str = "\x1b[36;1;4m\x1B]8;;";
pub const EHS: &str = ESCAPE_HYPERLINK_S;
pub const ESCAPE_HYPERLINK_E: &str = "\x1B\\(link)\x1B]8;;\x1B\\\x1b[0m";
pub const EHE: &str = ESCAPE_HYPERLINK_E;
