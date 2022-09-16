use crate::utils::{check_bit, read_syncsafe_intefer, write_syncsafe_integer};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
struct ID3v2FrameFlags {
    /**
     * This flag tells the software what to do with this frame if it is unknown and
     * the tag is altered in any way. This applies to all kinds of alterations,
     * including adding more padding and reordering the frames.
     *  0 - Frame should be preserved.
     *  1 - Frame should be discarded.
     */
    tag_alter_preservation: bool,

    /**
     * This flag tells the software what to do with this frame if it is unknown and
     * the file, excluding the tag, is altered. This does not apply when the audio
     * is completely replaced with other audio data.
     *  0 - Frame should be preserved.
     *  1 - Frame should be discarded.
     */
    file_alter_preservation: bool,

    /**
     * This flag, if set, tells the software that the contents of this frame is
     * intended to be read only. Changing the contents might break something,
     * e.g. a signature. If the contents are changed, without knowledge in why the
     * frame was flagged read only and without taking the proper means to compensate,
     * e.g. recalculating the signature, the bit should be cleared.
     */
    read_only: bool,

    /**
     * This flag indicates whether or not the frame is compressed.
     *  0 - Frame is not compressed.
     *  1 - Frame is compressed using [#ZLIB zlib] with 4 bytes for
     *      'decompressed size' appended to the frame header.
     */
    compression: bool,

    /**
     * This flag indicates wether or not the frame is enrypted. If set one byte
     * indicating with which method it was encrypted will be appended to the frame
     * header. See section 4.26. for more information about encryption method
     * registration.
     *  0 - Frame is not encrypted.
     *  1 - Frame is encrypted.
     */
    encryption: bool,

    /**
     * This flag indicates whether or not this frame belongs in a group with other
     * frames. If set a group identifier byte is added to the frame header. Every
     * frame with the same group identifier belongs to the same group.
     *  0 - Frame does not contain group information
     *  1 - Frame contains group information
     */
    grouping_identity: bool,

    raw_flags_byte: [u8; 2],
}

#[derive(Debug)]
pub struct ID3v2Frame {
    flags: ID3v2FrameFlags,
    pub size: u32,
    pub id: String,
    pub data: String,
}

impl ID3v2Frame {
    pub fn from_user_input(id: String, data: String) -> ID3v2Frame {
        ID3v2Frame {
            size: data.len() as u32,
            id,
            data,
            flags: ID3v2FrameFlags {
                tag_alter_preservation: false,
                file_alter_preservation: false,
                read_only: false,
                compression: false,
                encryption: false,
                grouping_identity: false,
                raw_flags_byte: [0, 0],
            },
        }
    }
    pub fn from_read_file(file: &mut File) -> ID3v2Frame {
        let mut buffer = [0u8; 10];
        file.read_exact(&mut buffer).expect("Failed to read buffer");
        let id = String::from_utf8(buffer[0..4].to_vec()).unwrap();
        let size = read_syncsafe_intefer(&buffer[4..8].try_into().unwrap());
        let flags = &buffer[8..10];
        let mut data_buffer = vec![0u8; size.try_into().unwrap()];
        file.read_exact(&mut data_buffer)
            .expect("Could not read entire contents of frame");
        let data = String::from_utf8(data_buffer).unwrap();
        ID3v2Frame {
            flags: ID3v2FrameFlags {
                tag_alter_preservation: check_bit(flags[0], 7),
                file_alter_preservation: check_bit(flags[0], 6),
                read_only: check_bit(flags[0], 5),
                compression: check_bit(flags[1], 7),
                encryption: check_bit(flags[1], 6),
                grouping_identity: check_bit(flags[1], 5),
                raw_flags_byte: flags.try_into().unwrap(),
            },
            size,
            id,
            data,
        }
    }

    pub fn format_id(&self) -> String {
        format!("{}", self.id)
    }

    pub fn format_data(&self) -> String {
        format!("{}", self.data)
    }

    pub fn format_flags(&self, human_readable: bool) -> String {
        if human_readable {
            let mut result: Vec<&str> = Vec::new();
            if self.flags.read_only {
                result.push("read-only");
            };
            if self.flags.compression {
                result.push("compression")
            };
            if self.flags.encryption {
                result.push("encryption");
            };
            if self.flags.file_alter_preservation {
                result.push("file-alter-preservation");
            };
            if self.flags.tag_alter_preservation {
                result.push("tag-alter-preservation");
            };
            if self.flags.grouping_identity {
                result.push("grouping-identity");
            };
            let result_string = result.iter().fold(String::new(), |mut acc, s| {
                if acc.len() != 0 {
                    acc.push_str(", ")
                }
                acc.push_str(s);
                acc
            });

            if result_string.len() != 0 {
                format!("({})", result_string)
            } else {
                String::new()
            }
        } else {
            [
                if self.flags.read_only { 'r' } else { '.' },
                if self.flags.compression { 'c' } else { '.' },
                if self.flags.encryption { 'e' } else { '.' },
                if self.flags.file_alter_preservation {
                    'f'
                } else {
                    '.'
                },
                if self.flags.tag_alter_preservation {
                    'r'
                } else {
                    '.'
                },
                if self.flags.grouping_identity {
                    'g'
                } else {
                    '.'
                },
            ]
            .iter()
            .collect()
        }
    }
    pub fn write_to_file(&self, file: &mut File) {
        let mut id_chars = self.id.chars();
        let id_buffer: Vec<u8> = [
            id_chars.next(),
            id_chars.next(),
            id_chars.next(),
            id_chars.next(),
        ]
        .iter()
        .map(|option| option.expect("Failed to extract ID3v2 ID from frame") as u8)
        .collect();

        let size_buffer = write_syncsafe_integer(self.size);
        let err_message = "Failed to write frame to fileA";
        file.write(&id_buffer).expect(err_message);
        file.write(&size_buffer).expect(err_message);
        file.write(&self.flags.raw_flags_byte).expect(err_message);
        file.write(&self.data.as_bytes()).expect(err_message);
    }
}

pub fn is_string_valid_id3v2_id(value: &str) -> bool {
    let bytes = value.as_bytes();

    (bytes.len() == 4)
        && ((bytes[0] >= b'A' && bytes[0] <= b'Z') || (bytes[0] >= b'0' && bytes[0] <= b'9'))
        && ((bytes[1] >= b'A' && bytes[1] <= b'Z') || (bytes[1] >= b'0' && bytes[1] <= b'9'))
        && ((bytes[2] >= b'A' && bytes[2] <= b'Z') || (bytes[2] >= b'0' && bytes[2] <= b'9'))
        && ((bytes[3] >= b'A' && bytes[3] <= b'Z') || (bytes[3] >= b'0' && bytes[3] <= b'9'))
}

pub fn is_string_known_id3v2_id(value: &str) -> bool {
    if !is_string_valid_id3v2_id(value) {
        return false;
    }

    if let Some(_) = KNOWN_ID3V2_IDS.iter().find(|item| (**item).0 == value) {
        true
    } else {
        false
    }
}

const KNOWN_ID3V2_IDS: [(&str, &str); 92] = [
    ("AENC", "Audio encryption"),
    ("ASPI", "Audio seek point index (v4 only)"),
    ("APIC", "Attached picture"),
    ("COMM", "Comments"),
    ("COMR", "Commercial frame"),
    ("ENCR", "Encryption method registration"),
    ("EQUA", "Equalization (v3)"),
    ("EQU2", "Equalization (v4)"),
    ("ETCO", "Event timing codes"),
    ("GEOB", "General encapsulated object"),
    ("GRID", "Group identification registration"),
    ("LINK", "Linked information"),
    ("MCDI", "Music CD identifier"),
    ("MLLT", "MPEG location lookup table"),
    ("OWNE", "Ownership frame"),
    ("PRIV", "Private frame"),
    ("PCNT", "Play counter"),
    ("POPM", "Popularimeter"),
    ("POSS", "Position synchronisation frame"),
    ("RBUF", "Recommended buffer size"),
    ("RVAD", "Relative volume adjustment (v3)"),
    ("RVA2", "Relative volume adjustment (v4)"),
    ("RVRB", "Reverb"),
    ("SEEK", "Seek frame (v4 only)"),
    ("SIGN", "Signature frame (v4 only)"),
    ("SYLT", "Synchronized lyric/text"),
    ("SYTC", "Synchronized tempo codes"),
    ("TBPM", "Beats per minute (BPM)"),
    ("TKEY", "Initial key"),
    ("TCON", "Content type"),
    ("TMOO", "Mood (v4 only)"),
    ("TCOP", "Copyright message"),
    ("TDRC", "Recording time (V4 only)"),
    ("TDAT", "Date (v3) (replaced by TDRC in V4)"),
    ("TRDA", "Recording dates (v3) (replaced by TDRC in V4)"),
    ("TIME", "Time (v3) (replaced by TDRC in V4)"),
    ("TYER", "Year (v3) (replaced by TDRC in V4)"),
    ("TDRL", "Release time (v4 only)"),
    ("TDTG", "Tagging time (v4 only)"),
    ("TDEN", "Encoding time (v4 only)"),
    ("TENC", "Encoded by"),
    ("TSSE", "Software/Hardware and settings used for encoding"),
    ("TDLY", "Playlist delay"),
    ("TIT1", "Content group description"),
    ("TIT2", "Title/songname/content description"),
    ("TIT3", "Subtitle/Description refinement"),
    ("TALB", "Album/Movie/Show title"),
    ("TLAN", "Language(s)"),
    ("TLEN", "Length"),
    ("TSIZ", "Size (V3 only)"),
    ("TFLT", "File type"),
    ("TMED", "Media type"),
    ("TOWN", "File owner/licensee"),
    (
        "TPE1",
        "Lead performer(s)/Soloist(s) (can be separated by \"/\")",
    ),
    ("TPE2", "Band/orchestra/accompaniment"),
    ("TPE3", "Conductor/performer refinement"),
    ("TPE4", "Interpreted, remixed, or otherwise modified by"),
    ("TCOM", "Composer (can be separated by \"/\")"),
    ("TEXT", "Lyricist/Text writer (can be separated by \"/\")"),
    ("IPLS", "Involved people list (v3)"),
    ("TIPL", "Involved people list (v4)"),
    ("TMCL", "Musician credits list (v4 only)"),
    ("TOAL", "Original album/movie/show title"),
    ("TOFN", "Original filename"),
    (
        "TOLY",
        "Original lyricist(s)/text writer(s) (can be separated by \"/\")",
    ),
    (
        "TOPE",
        "Original artist(s)/performer(s) (can be separated by \"/\")",
    ),
    ("TORY", "Original release year (v3)"),
    ("TDOR", "Original release year (v4)"),
    ("TRCK", "Track number/Position in set"),
    ("TPOS", "Part of a set"),
    ("TSST", "Set subtitle (v4 only)"),
    ("TPRO", "Produced notice (v4 only)"),
    ("TPUB", "Publisher"),
    ("TRSN", "Internet radio station name"),
    ("TRSO", "Internet radio station owner"),
    ("TSOA", "Album sort order (v4 only)"),
    ("TSOP", "Performer sort order (v4 only)"),
    ("TSOT", "Title sort order (v4 only)"),
    ("TSRC", "International Standard Recording Code (ISRC)"),
    ("TXXX", "User defined text information frame"),
    ("UFID", "Unique file identifier"),
    ("USER", "Terms of use"),
    ("USLT", "Unsynchronized lyric/text transcription"),
    ("WCOM", "Commercial information"),
    ("WCOP", "Copyright/Legal information"),
    ("WOAF", "Official audio file webpage"),
    ("WOAR", "Official artist/performer webpage"),
    ("WOAS", "Official audio source webpage"),
    ("WORS", "Official internet radio station homepage"),
    ("WPAY", "Payment"),
    ("WPUB", "Publishers official webpage"),
    ("WXXX", "User defined URL link frame"),
];
