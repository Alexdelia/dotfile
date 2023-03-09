use ansi::{c8bit, c8bit_bg, hex, hex_bg, rgb, rgb_bg};

/*
#[test]
fn no_compile() {
    rgb!("hey", 0, 0);
    rgb!(-1, 0, 0);
    rgb!(256, 0, 0);
    hex!(#ff00f);
    hex!(0xFF00FF, some);
}
*/

#[test]
fn test_rgb() {
    assert_eq!(rgb!(42, 255, 0), "\x1b[38;2;42;255;0m");
}

#[test]
fn test_rgb_bg() {
    assert_eq!(rgb_bg!(42, 255, 0), "\x1b[48;2;42;255;0m");
}

#[test]
fn test_c8bit() {
    assert_eq!(c8bit!(42), "\x1b[38;5;42m");
}

#[test]
fn test_c8bit_bg() {
    assert_eq!(c8bit_bg!(42), "\x1b[48;5;42m");
}

#[test]
fn test_hex() {
    assert_eq!(hex!("FF00FF"), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!("#FF00FF"), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!("ff00ff"), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!("#ff00ff"), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!(0xFF00FF), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!(0xff00ff), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!(#FF00FF), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!(#ff00ff), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!(FF00FF), "\x1b[38;2;255;0;255m");
    assert_eq!(hex!(ff00ff), "\x1b[38;2;255;0;255m");
}

#[test]
fn test_hex_bg() {
    assert_eq!(hex_bg!("FF00FF"), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!("#FF00FF"), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!("ff00ff"), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!("#ff00ff"), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!(0xFF00FF), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!(0xff00ff), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!(#FF00FF), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!(#ff00ff), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!(FF00FF), "\x1b[48;2;255;0;255m");
    assert_eq!(hex_bg!(ff00ff), "\x1b[48;2;255;0;255m");
}
