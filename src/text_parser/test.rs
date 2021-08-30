fn main() {
    println!("run `cargo test` instead");
}

use kukumba::kukumba;
mod text_parser;
use text_parser::*;

kukumba! (
    #[scenario_01]

    for_the "`text_parser` module"
    having "example inputs" {
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
