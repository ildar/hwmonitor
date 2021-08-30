use hex;

// generally, command parsed: "px {} @{}"
pub fn parse_px(s: &[u8], defaults: (u32,u32)) -> Result<(u32,u32),&[u8]>
    {
    let addr = s.split(|c| *c==b'@').last();
    let start = if addr == None { defaults.1 } else {
        let mut start = [0u8;4];
        if hex::decode_to_slice(addr.unwrap(), &mut start) != Ok(()) {
            return Err(b"can't decode addr");
        };
        u32::from_be_bytes(start)
    };
    //FIXME: process 1st value
    Ok((defaults.0,start))
}

