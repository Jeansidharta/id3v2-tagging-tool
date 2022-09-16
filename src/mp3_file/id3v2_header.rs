use crate::utils::{read_syncsafe_intefer, write_syncsafe_integer};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Default, Debug)]
#[allow(dead_code)]
struct ID3v2HeaderFlags {
    has_extended_header: bool,
    has_unsynchronization: bool,
    has_experimental_indicator: bool,
    raw_flags_byte: u8,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct ID3v2Header {
    flags: ID3v2HeaderFlags,
    size: u32,
    version: u16,
}

impl ID3v2Header {
    pub fn from_read_file(file: &mut File) -> ID3v2Header {
        let mut buffer: [u8; 10] = [0; 10];
        file.read_exact(&mut buffer)
            .expect("Could not read enough bytes for the file header");

        if String::from_utf8(buffer[0..3].to_vec()).unwrap() != "ID3" {
            println!("Invalid header type {:?}", &buffer[0..3]);
            panic!();
        }

        let version = ((buffer[4] as u16) << 8) + buffer[3] as u16;
        let flags = buffer[5];
        let raw_flags_byte = flags;
        let has_experimental_indicator = (flags >> 7) & 1 == 1;
        let has_extended_header = (flags >> 6) & 1 == 1;
        let has_unsynchronization = (flags >> 5) & 1 == 1;

        let size = read_syncsafe_intefer(&buffer[6..10].try_into().unwrap());

        ID3v2Header {
            flags: ID3v2HeaderFlags {
                has_extended_header,
                has_unsynchronization,
                has_experimental_indicator,
                raw_flags_byte,
            },
            size,
            version,
        }
    }

    pub fn skip_extended_header_if_needed(&self, file: &mut File) {
        if self.flags.has_extended_header {
            let mut buffer: [u8; 4] = [0; 4];
            file.read_exact(&mut buffer).expect("Failed to read file");
            let extended_header_size = read_syncsafe_intefer(&buffer);
            file.seek(SeekFrom::Current(extended_header_size as i64))
                .expect("failed to seek");
        }
    }

    pub fn write_to_file(&self, file: &mut File, size: u32) {
        let size_bytes = write_syncsafe_integer(size);
        let buffer = [
            b'I',
            b'D',
            b'3',
            self.version as u8,
            (self.version >> 8) as u8,
            self.flags.raw_flags_byte,
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
        ];
        file.write(&buffer).expect("Failed to write to file");
    }

    pub fn has_extended_header(&self) -> bool {
        self.flags.has_extended_header
    }
}
