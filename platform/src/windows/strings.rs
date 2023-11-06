use windows::core::PCSTR;

pub fn windows_string(s: &str) -> PCSTR {
    let mut s = s.to_string();

    if s.chars().last() != Some('\0') {
        s += "\0";
    }

    PCSTR::from_raw(s.as_ptr())
}
