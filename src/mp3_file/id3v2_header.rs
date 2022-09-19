use crate::utils::{is_valid_syncsafe_integer, log, read_syncsafe_integer, write_syncsafe_integer};
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
#[allow(dead_code)]
struct ID3v2HeaderFlags {
    has_extended_header: bool,
    has_unsynchronization: bool,
    has_experimental_indicator: bool,
    raw_flags_byte: u8,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ID3v2Header {
    flags: ID3v2HeaderFlags,
    size: u32,
    version: u16,
}

impl ID3v2Header {
    pub fn from_read_file(file: &mut File) -> Result<ID3v2Header, ()> {
        let mut buffer: [u8; 10] = [0; 10];

        file.read_exact(&mut buffer).or_else(|err| {
            match err.kind() {
                ErrorKind::UnexpectedEof => {
                    log::error("Could not read the ID3v2 Header. File ended too soon.".to_string())
                }
                _ => log::error("Could not read the ID3v2 Header. Unknown error.".to_string()),
            };
            Err(())
        })?;

        // According to the specs, the first 3 bytes must be "ID3"
        if buffer[0] != b'I' || buffer[1] != b'D' || buffer[2] != b'3' {
            log::error(
                "Invalid header. It does not start with \"ID3\" as it's first 3 bytes".to_string(),
            );
            return Err(());
        };

        let version = ((buffer[4] as u16) << 8) + buffer[3] as u16;
        if version != 4 {
            log::warn(format!(
                "Header version is {}, but this software only supports version 4",
                version
            ));
        };
        let flags = {
            let flags_byte = buffer[5];

            // Check if bits 1, 2, 3, 4 and 5 are set on the flags byte
            // According to the ID3v2 specs, they should be cleared.
            if flags_byte << 3 != 0 {
                log::warn("Header has unofficial flag bits set".to_string());
            }

            ID3v2HeaderFlags {
                raw_flags_byte: flags_byte,
                has_experimental_indicator: (flags_byte >> 7) & 1 == 1,
                has_extended_header: (flags_byte >> 6) & 1 == 1,
                has_unsynchronization: (flags_byte >> 5) & 1 == 1,
            }
        };

        let size = {
            // unwrap because this should never fail
            let size_bytes: [u8; 4] = buffer[6..10].try_into().unwrap();

            if !is_valid_syncsafe_integer(&size_bytes) {
                log::warn(
                    "Header size is not properly represented as a syncsafe integer".to_string(),
                );
            }
            read_syncsafe_integer(&size_bytes)
        };

        // Skip extended header if needed
        if flags.has_extended_header {
            let mut buffer: [u8; 4] = [0; 4];

            file.read_exact(&mut buffer).or_else(|err| {
                match err.kind() {
                    ErrorKind::UnexpectedEof => log::error(
                        "Could not read the extended header's size. File ended too soon."
                            .to_string(),
                    ),
                    _ => log::error(
                        "Could not read the exetended header's size. Unknown error.".to_string(),
                    ),
                };
                Err(())
            })?;
            let extended_header_size = read_syncsafe_integer(&buffer);

            file.seek(SeekFrom::Current(extended_header_size as i64))
                .or_else(|_| {
                    log::error("Failed to seek backwards while reading ID3v2 Header".to_string());
                    Err(())
                })?;
        }

        Ok(ID3v2Header {
            flags,
            size,
            version,
        })
    }

    pub fn write_to_file(&self, file: &mut File, size: u32) -> Result<(), ()> {
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

        file.write(&buffer).or_else(|error| {
            log::error(format!(
                "Failed to write to file. Uknown error. Error kind is {}",
                error.kind()
            ));
            Err(())
        })?;

        Ok(())
    }

    pub fn has_extended_header(&self) -> bool {
        self.flags.has_extended_header
    }
}
