pub const RESET: &str = "\x1b[0m";
pub const D: &str = RESET;

pub const BOLD: &str = "\x1b[1m";
pub const B: &str = BOLD;
pub const FAINT: &str = "\x1b[2m";
pub const F: &str = FAINT;
pub const ITALIC: &str = "\x1b[3m";
pub const I: &str = ITALIC;

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const GRE: &str = GREEN;
pub const YELLOW: &str = "\x1b[33m";
pub const YEL: &str = YELLOW;
pub const BLUE: &str = "\x1b[34m";
pub const BLU: &str = BLUE;
pub const MAGENTA: &str = "\x1b[35m";
pub const MAG: &str = MAGENTA;
pub const CYAN: &str = "\x1b[36m";
pub const CYA: &str = CYAN;

pub const ERROR: &str = "\x1b[31m";
pub const E: &str = ERROR;
pub const BERROR: &str = "\x1b[31;1m";
pub const BE: &str = BERROR;

pub const WARNING: &str = "\x1b[33m";
pub const W: &str = WARNING;
pub const BWARNING: &str = "\x1b[33;1m";
pub const BW: &str = BWARNING;

pub const VALID: &str = "\x1b[32;1m";
pub const V: &str = VALID;
pub const MUT: &str = "\x1b[35;1m";
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
