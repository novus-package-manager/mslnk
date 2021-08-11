use crate::LinkFlags;
use byteorder::{ByteOrder, LE};

pub fn to_data<S: Into<String>>(str_data: S, flags: LinkFlags) -> Vec<u8> {
    let s = str_data.into();
    if !flags.contains(LinkFlags::IS_UNICODE) {
        let mut bytes = vec![0u8; 2];
        for c in s.chars() {
            bytes.push(c as u8); // FIXME: clips non-Latin-1 characters!
        }
        let len = bytes.len() - 2;
        LE::write_u16(&mut bytes, len as u16); // writes u16 len at the start
        bytes
    } else {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let mut bytes = vec![0u8; 2 + utf16.len() * 2];
        LE::write_u16(&mut bytes, utf16.len() as u16);
        LE::write_u16_into(&utf16, &mut bytes[2..]);
        bytes
    }
}
