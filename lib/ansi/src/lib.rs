pub use ::const_format::formatcp as __formatcp;
pub use gen::{c8bit, c8bit_bg, hex, hex_bg, rgb, rgb_bg};

use ansi_regex::ansi_regex;
use std::borrow::Cow;

pub fn remove<'t>(s: &'t str) -> Cow<'t, str> {
    ansi_regex().replace_all(s, "")
}

pub const RESET: &str = "\x1b[0m";

pub const CLEAR: &str = "\x1b[H\x1b[2J";

pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const BLINK: &str = "\x1b[5m";
pub const FAST_BLINK: &str = "\x1b[6m"; // not widely supported
pub const REVERSE: &str = "\x1b[7m";
pub const HIDDEN: &str = "\x1b[8m"; // not widely supported
pub const CROSS: &str = "\x1b[9m";

pub const DOUBLE_UNDERLINE: &str = "\x1b[21m"; // can instead disable BOLD

pub const NO_ITALIC: &str = "\x1b[23m";
pub const NO_UNDERLINE: &str = "\x1b[24m";
pub const NO_BLINK: &str = "\x1b[25m";
pub const NO_REVERSE: &str = "\x1b[27m";
pub const NO_HIDDEN: &str = "\x1b[28m"; // not widely supported
pub const NO_CROSS: &str = "\x1b[29m";

pub const NO_COLOR: &str = "\x1b[39m";

pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const B_BLACK: &str = "\x1b[1;30m";
pub const B_RED: &str = "\x1b[1;31m";
pub const B_GREEN: &str = "\x1b[1;32m";
pub const B_YELLOW: &str = "\x1b[1;33m";
pub const B_BLUE: &str = "\x1b[1;34m";
pub const B_MAGENTA: &str = "\x1b[1;35m";
pub const B_CYAN: &str = "\x1b[1;36m";
pub const B_WHITE: &str = "\x1b[1;37m";

pub const BG_NO_COLOR: &str = "\x1b[49m";

pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";

pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
pub const BG_BRIGHT_RED: &str = "\x1b[101m";
pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";

pub mod abbrev {
    use super::*;

    pub const D: &str = RESET; // D for default

    pub const C: &str = CLEAR;

    pub const B: &str = BOLD;
    pub const F: &str = DIM; // F for faint
    pub const I: &str = ITALIC;
    pub const U: &str = UNDERLINE;
    pub const BL: &str = BLINK;
    // pub const FB: &str = FAST_BLINK;	// not widely supported
    pub const RV: &str = REVERSE;
    // pub const H: &str = HIDDEN;	// not widely supported
    pub const X: &str = CROSS;

    pub const N_I: &str = NO_ITALIC;
    pub const N_U: &str = NO_UNDERLINE;
    pub const N_BL: &str = NO_BLINK;
    pub const N_R: &str = NO_REVERSE;
    // pub const N_H: &str = NO_HIDDEN;	// not widely supported
    pub const N_X: &str = NO_CROSS;

    pub const N_C: &str = NO_COLOR;

    pub const BLA: &str = BLACK;
    //
    //
    pub const R: &str = RED;
    pub const GRE: &str = GREEN;
    pub const G: &str = GREEN;
    pub const YEL: &str = YELLOW;
    pub const Y: &str = YELLOW;
    pub const BLU: &str = BLUE;
    //
    pub const MAG: &str = MAGENTA;
    pub const M: &str = MAGENTA;
    pub const CYA: &str = CYAN;
    //
}
