use hex;

pub fn parse_px(s: &[u8], defaults: (u32,u32)) -> Option<(u32,u32)>
    {
    let addr = s.split(|c| *c==b'@').last();
    let start = if addr == None { defaults.0 } else {
        let mut start = [0u8;4];
        if hex::decode_to_slice(addr.unwrap(), &mut start) != Ok(()) {
            return None;
        };
        u32::from_be_bytes(start)
    };
    Some((start,16))
}

