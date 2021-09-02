use hex;

pub fn parse_address(s: &[u8]) -> Result<u32,&[u8]> {
    let mut start = [0u8;4];
    if hex::decode_to_slice(s, &mut start).is_err() {
        return Err(b"can't decode addr");
    };
    Ok(u32::from_be_bytes(start))
}

// generally, command parsed: "px {} @{}"
pub fn parse_px(s: &[u8], defaults: (u32,u32)) -> Result<(u32,u32),&[u8]>
    {
    let addr = s.split(|c| *c==b'@').last();
    let start = if addr == None { defaults.1 } else { parse_address(addr.unwrap())? };
    //FIXME: process 1st value
    Ok((defaults.0,start))
}

