fn encode_char(char: char) -> Option<u8> {
    match char {
        _ if char < '\u{20}' => None,
        _ if char >= '\u{20}' && char < '\u{7F}' => Some(char as u8),
        _ if char >= '\u{7F}' && char < '\u{A0}' => None,
        _ if char >= '\u{A0}' && char <= '\u{FF}' => Some(char as u8),
        _ => None,
    }
}

const NULL: char = '\u{00}';

// Prevent rustfmt from formating this beautiful table
#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(dead_code)]
const DECODE_TABLE: &[char] = &[
//         0x00      0x01     0x02      0x03      0x04      0x05      0x06      0x07      0x08      0x09      0x0A      0x0B      0x0C      0x0D      0x0E      0x0F
/* 0x00 */ NULL,     NULL,    NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,
/* 0x01 */ NULL,     NULL,    NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,
/* 0x02 */ '\u{20}', '!',     '"',      '#',      '$',      '%',      '&',      '\'',     '(',      ')',      '*',      '+',      ',',      '-',      '.',      '/',
/* 0x03 */ '0',      '1',     '2',      '3',      '4',      '5',      '6',      '7',      '8',      '9',      ':',      ';',      '<',      '=',      '>',      '?',
/* 0x04 */ '@',      'A',     'B',      'C',      'D',      'E',      'F',      'G',      'H',      'I',      'J',      'K',      'L',      'M',      'N',      'O',
/* 0x05 */ 'P',      'Q',     'R',      'S',      'T',      'U',      'V',      'W',      'X',      'Y',      'Z',      '[',      '\\',     ']',      '^',       '_',
/* 0x06 */ '`',      'a',     'b',      'c',      'd',      'e',      'f',      'g',      'h',      'i',      'j',      'k',      'l',      'm',      'n',      'o',
/* 0x07 */ 'p',      'q',     'r',      's',      't',      'u',      'v',      'w',      'x',      'y',      'z',      '{',      '|',      '}',      '~',      NULL,
/* 0x08 */ NULL,     NULL,    NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,
/* 0x09 */ NULL,     NULL,    NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,     NULL,
/* 0x0A */ '\u{A0}', '\u{A1}','\u{A2}', '\u{A3}', '\u{A4}', '\u{A5}', '\u{A6}', '\u{A7}', '\u{A8}', '\u{A9}', '\u{AA}', '\u{AB}', '\u{AC}', '\u{AD}', '\u{AE}', '\u{AF}',
/* 0x0B */ '\u{B0}', '\u{B1}','\u{B2}', '\u{B3}', '\u{B4}', '\u{B5}', '\u{B6}', '\u{B7}', '\u{B8}', '\u{B9}', '\u{BB}', '\u{BB}', '\u{BC}', '\u{BD}', '\u{BE}', '\u{BF}',
/* 0x0C */ '\u{C0}', '\u{C1}','\u{C2}', '\u{C3}', '\u{C4}', '\u{C5}', '\u{C6}', '\u{C7}', '\u{C8}', '\u{C9}', '\u{CC}', '\u{CC}', '\u{CC}', '\u{CD}', '\u{CE}', '\u{CF}',
/* 0x0D */ '\u{D0}', '\u{D1}','\u{D2}', '\u{D3}', '\u{D4}', '\u{D5}', '\u{D6}', '\u{D7}', '\u{D8}', '\u{D9}', '\u{DD}', '\u{DD}', '\u{DD}', '\u{DD}', '\u{DE}', '\u{DF}',
/* 0x0E */ '\u{E0}', '\u{E1}','\u{E2}', '\u{E3}', '\u{E4}', '\u{E5}', '\u{E6}', '\u{E7}', '\u{E8}', '\u{E9}', '\u{EE}', '\u{EE}', '\u{EE}', '\u{EE}', '\u{EE}', '\u{EF}',
/* 0x0F */ '\u{F0}', '\u{F1}','\u{F2}', '\u{F3}', '\u{F4}', '\u{F5}', '\u{F6}', '\u{F7}', '\u{F8}', '\u{F9}', '\u{FF}', '\u{FF}', '\u{FF}', '\u{FF}', '\u{FF}', '\u{FF}',
];

fn decode_char(char: u8) -> Option<char> {
    match DECODE_TABLE[char as usize] {
        NULL => None,
        value => Some(value),
    }
}

pub fn is_valid_latin1_string(buffer: &[u8]) -> bool {
    buffer
        .iter()
        .all(|item| *item == 0u8 || DECODE_TABLE[*item as usize] != NULL)
}

pub fn can_be_converted_to_latin1_string(str: &str) -> bool {
    str.chars()
        .all(|char| (char >= '\u{20}' && char < '\u{7F}') || (char >= '\u{A0}' && char <= '\u{FF}'))
}

pub fn encode(string: &str) -> Option<Vec<u8>> {
    let mut vector = Vec::new();
    for char in string.chars() {
        let byte = encode_char(char)?;
        vector.push(byte);
    }
    Some(vector)
}

pub fn decode(buffer: &[u8]) -> Option<String> {
    let mut string = String::new();
    for byte in buffer.iter() {
        let char = decode_char(*byte)?;
        string.push(char);
    }
    Some(string)
}
