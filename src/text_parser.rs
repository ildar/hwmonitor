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

pub fn parse_g(s: &[u8]) -> (Option<u32>,Option<u32>) {
    let mut iter = s.split(|c| *c == b' ');
    iter.next(); // skip command
    let mut arg = iter.next();
    while arg.is_some() && arg.unwrap().len() == 0 { arg = iter.next(); };
    if arg == None { return (None,None); };
    let arg = parse_address(arg.unwrap());
    if arg.is_err() { return (None,None); };

    let mut arg2 = iter.next();
    while arg2.is_some() && arg2.unwrap().len() == 0 { arg2 = iter.next(); };
    if arg2 == None { return (Some(arg.unwrap()),None); };
    let arg2 = parse_address(arg2.unwrap());
    if arg2.is_err() { return (Some(arg.unwrap()),None); };
    ( Some(arg.unwrap()), Some(arg2.unwrap()) )
}
