fn main() {
    println!("run `cargo test -p testbed --target x86_64-unknown-linux-gnu` instead");
}

use kukumba::kukumba;
mod text_parser;
use text_parser::*;

kukumba! (
    #[scenario_01]
    for_the "`text_parser` module"
    having "example parse_address() inputs" {
        let good1:[u8;2] = *b"42";
        let good2:[u8;4] = *b"0x42";
        let bad1:[u8;2] = *b"4x";
        let bad2:[u8;4] = *b"0x4x";
        let bad3:[u8;2] = *b"0x";
    }
    testing "parse_address() function" {
        let res = parse_address(&good1);
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res, 42);
        let res = parse_address(&good2);
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res, 0x42);
        assert_eq!(parse_address(&bad1).is_err(),true);
        assert_eq!(parse_address(&bad2).is_err(),true);
        assert_eq!(parse_address(&bad3).is_err(),true);
    }

    #[scenario_02]
    having "example parse_command() inputs" {
        let goodcmd1:[u8;11] = *b"px 20 @0x42";
        let goodcmd2:[u8;8] = *b"px @0x42";
        let goodcmd3:[u8;8] = *b"px    20";
        let goodcmd4:[u8;1] = *b"p";
        let goodcmd5:[u8;8] = *b"G 0x42 1";
        let badcmd1:[u8;13] = *b"g a r b a g e";
    }
    testing "parse_command() function" {
        let mut out = [None; 4];
        let mut addr = None;
        parse_command(&goodcmd1, &mut out, &mut addr);
        assert_eq!(addr, Some(0x42));
        assert_eq!(out[0], Some(20));
        assert_eq!(out[1], None);
        out = [None; 4]; parse_command(&goodcmd2, &mut out, &mut addr);
        assert_eq!(addr, Some(0x42));
        assert_eq!(out[0], None);
        out = [None; 4]; parse_command(&goodcmd3, &mut out, &mut addr);
        assert_eq!(addr, None);
        assert_eq!(out[0], Some(20));
        assert_eq!(out[1], None);
        out = [None; 4]; parse_command(&goodcmd4, &mut out, &mut addr);
        assert_eq!(addr, None);
        assert_eq!(out[0], None);
        out = [None; 4]; parse_command(&goodcmd5, &mut out, &mut addr);
        assert_eq!(addr, None);
        assert_eq!(out[0], Some(0x42));
        assert_eq!(out[1], Some(1));
        assert_eq!(out[2], None);
        out = [None; 4]; parse_command(&badcmd1, &mut out, &mut addr);
        assert_eq!(addr, None);
        assert_eq!(out[0], None);
    }

);
