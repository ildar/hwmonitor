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

// generally, command parsed: "px {} @{}"
pub fn parse_px(s: &[u8], defaults: (u32,u32)) -> Result<(u32,u32),&[u8]> {
    let mut iter_at = s.split(|c| *c==b'@');
    let mut iter_space = iter_at.next().unwrap().split(|c| *c==b' ');
    // start address
    let addr = iter_at.last();
    let start = if addr == None { defaults.1 } else { parse_address(addr.unwrap())? };
    // len
    iter_space.next(); // consume command "px"
    let mut addr = iter_space.next();
    while addr.is_some() && addr.unwrap().len() == 0 { addr = iter_space.next() };
    let len = if addr == None { defaults.0 } else { parse_address(addr.unwrap())? };
    Ok((len,start))
}

