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
    }
    testing "parse_address() function" {
        let res = parse_address(&good1);
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res, 42);
    }

    #[scenario_02]
    having "example parse_px() inputs" {
        let goodcmd:[u8;15] = *b"px 20 @00000042";
    }
    testing "parse_px() function" {
        let res = parse_px(&goodcmd, (55,66));
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res.0, 55);
        assert_eq!(res.1, 0x42);
    }
);
