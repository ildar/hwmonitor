pub fn parse_address(s: &[u8]) -> Result<u32,&[u8]> {
    let mode_is_hex = if s.starts_with(b"0x") { true } else { false };
    match mode_is_hex {
        false => { // decimal
            let temp = core::str::from_utf8(s);
            if temp.is_err() { return Err(b"Unknown str convertion error") };
            let temp = u32::from_str_radix(temp.unwrap(), 10);
            if temp.is_err() { return Err(b"Convertion error") };
            Ok(temp.unwrap())
        },
        true => { // hexadecimal
            let s = &s[2..]; // cut "0x"
            let temp = core::str::from_utf8(s);
            if temp.is_err() { return Err(b"Unknown str convertion error") };
            let temp = u32::from_str_radix(temp.unwrap(), 16);
            if temp.is_err() { return Err(b"Convertion error") };
            Ok(temp.unwrap())
        },
    }
}

// general command sytax: "command arg1 arg2 ... @addr"
pub fn parse_command(s: &[u8], out: &mut [Option<u32>], out_addr: &mut Option<u32>) {
    let mut iter_at = s.split(|c| *c==b'@');
    let mut iter_space = iter_at.next().unwrap().split(|c| *c==b' ');

    *out_addr = (||{ parse_address(iter_at.last()?).ok() })();

    iter_space.next(); // consume command
    for outv in out.iter_mut() {
        let mut s = iter_space.next();
        while s.is_some() && s.unwrap().len() == 0 { s = iter_space.next() };
        if s == None { return; } else {
            *outv = parse_address(s.unwrap()).ok();
        };
    };
}

use arrayvec::ArrayVec;

// command sytax: "command hexbyte(s)1 hexbyte(s)2 ... @addr"
pub fn parse_command_w_bytestream(s: &[u8], out_addr: &mut Option<u32>) -> ArrayVec<u8,32> {
    let mut iter_at = s.split(|c| *c==b'@');
    let mut iter_space = iter_at.next().unwrap().split(|c| *c==b' ');

    *out_addr = (||{ parse_address(iter_at.last()?).ok() })();

    iter_space.next(); // consume command

    let mut buf = ArrayVec::<u8,32>::new();
    let mut arg = iter_space.next();
    while arg.is_some() {
        arg.unwrap()
            .chunks_exact(2)
            .map(|xx| {
                let temp = core::str::from_utf8(xx);
                if temp.is_err() { return None; };
                let temp = u32::from_str_radix(temp.unwrap(), 16);
                if temp.is_err() { return None; };
                Some(temp.unwrap() as u8)
            })
            .filter(|oo| oo.is_some())
            .for_each(|nn| buf.push(nn.unwrap()));
        arg = iter_space.next();
    }
    buf
}


