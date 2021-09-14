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
    having "example parse_px() inputs" {
        let goodcmd1:[u8;11] = *b"px 20 @0x42";
        let goodcmd2:[u8;8] = *b"px @0x42";
        let goodcmd3:[u8;8] = *b"px    20";
        let goodcmd4:[u8;1] = *b"p";
    }
    testing "parse_px() function" {
        let res = parse_px(&goodcmd1, (55,66));
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res.0, 20);
        assert_eq!(res.1, 0x42);
        let res = parse_px(&goodcmd2, (55,66));
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), (55, 0x42));
        let res = parse_px(&goodcmd3, (55,66));
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), (20, 66));
        let res = parse_px(&goodcmd4, (55,66));
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), (55, 66));
    }

    #[scenario_03]
    having "example parse_g() inputs" {
        let goodcmd1:[u8;4] = *b"g 20";
        let goodcmd2:[u8;6] = *b"g 0x42";
        let goodcmd3:[u8;6] = *b"g   20";
        let goodcmd4:[u8;1] = *b"g";
        let goodcmd5:[u8;8] = *b"G 0x42 1";
    }
    testing "parse_g() function" {
        let (res,_) = parse_g(&goodcmd1);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 20);
        let (res,_) = parse_g(&goodcmd2);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 0x42);
        let (res,_) = parse_g(&goodcmd3);
        assert_ne!(res, None);
        assert_eq!(res.unwrap(), 20);
        let (res,_) = parse_g(&goodcmd4);
        assert_eq!(res, None);
        let (res,res2) = parse_g(&goodcmd5);
        assert_eq!(res.unwrap(), 0x42);
        assert_eq!(res2.unwrap(), 1);
    }

);
