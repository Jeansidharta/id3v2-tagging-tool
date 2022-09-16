use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use super::{id3v2_frame::ID3v2Frame, id3v2_header::ID3v2Header};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Mp3File {
    header: ID3v2Header,
    frames: Vec<ID3v2Frame>,
    read_file: File,
    write_file: Option<File>,
}

impl Mp3File {
    pub fn remove_frame(&mut self, frame_id: &str, user_frame_index: u32) -> Result<(), u32> {
        self.find_index_of_frame_with_id(frame_id, user_frame_index)
            .and_then(|index| {
                self.frames.remove(index);
                Ok(())
            })
    }

    pub fn edit_frame(
        &mut self,
        frame_id: &str,
        new_data: String,
        user_frame_index: u32,
    ) -> Result<(), u32> {
        self.find_index_of_frame_with_id(frame_id, user_frame_index)
            .and_then(|index| {
                self.frames[index].data = new_data;
                Ok(())
            })
    }

    fn find_index_of_frame_with_id(&self, frame_id: &str, frame_index: u32) -> Result<usize, u32> {
        let mut frames_found_counter = 0u32;

        self.frames
            .iter()
            .position(|item| {
                if item.id == frame_id {
                    if frames_found_counter < frame_index {
                        frames_found_counter += 1;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .ok_or(frames_found_counter)
    }

    fn is_valid_frame_header(bytes: &[char; 4]) -> bool {
        ((bytes[0] >= 'A' && bytes[0] <= 'Z') || (bytes[0] >= '0' && bytes[0] <= '9'))
            && ((bytes[1] >= 'A' && bytes[1] <= 'Z') || (bytes[1] >= '0' && bytes[1] <= '9'))
            && ((bytes[2] >= 'A' && bytes[2] <= 'Z') || (bytes[2] >= '0' && bytes[2] <= '9'))
            && ((bytes[3] >= 'A' && bytes[3] <= 'Z') || (bytes[3] >= '0' && bytes[3] <= '9'))
    }

    fn has_new_frame(&mut self) -> bool {
        let mut buffer = [0u8; 4];
        self.read_file
            .read_exact(&mut buffer)
            .expect("Failed to read file");
        if let Err(_e) = self.read_file.seek(SeekFrom::Current(-4)) {
            false
        } else if !Mp3File::is_valid_frame_header(&mut buffer.map(|x| x as char)) {
            false
        } else {
            true
        }
    }

    pub fn from_path(path: &Path) -> Mp3File {
        let read_file = File::open(path).expect("Couldn't find file");
        Mp3File {
            frames: vec![],
            header: ID3v2Header::default(),
            read_file,
            write_file: None,
        }
    }

    pub fn read(&mut self) {
        self.header = ID3v2Header::from_read_file(&mut self.read_file);
        self.header
            .skip_extended_header_if_needed(&mut self.read_file);
        while self.has_new_frame() {
            self.frames
                .push(ID3v2Frame::from_read_file(&mut self.read_file))
        }
    }

    fn calculate_id3v2_size(&self) -> u32 {
        let extended_header_size = if self.header.has_extended_header() {
            10
        } else {
            0
        };
        let mut frames_size = 0;
        for frame in self.frames.iter() {
            frames_size += 10;
            frames_size += frame.size;
        }

        extended_header_size + frames_size
    }

    pub fn write_to_file(&mut self, file_path: &PathBuf) {
        let mut write_file = File::create(file_path).expect("Could not open file for writing");
        let size = self.calculate_id3v2_size();
        self.header.write_to_file(&mut write_file, size);
        for frame in self.frames.iter() {
            frame.write_to_file(&mut write_file)
        }

        let mut buffer = [0; 4096];
        while self
            .read_file
            .read(&mut buffer)
            .expect("Failed to read file")
            > 0
        {
            write_file.write(&buffer).expect("Failed to write to file");
        }

        self.write_file = Some(write_file);
    }

    pub fn format_frames(&self, frame_flags: bool, human_readable: bool) -> String {
        self.frames
            .iter()
            .map(|frame| -> String {
                if frame_flags {
                    let mut flags_str = frame.format_flags(human_readable);
                    if flags_str.len() != 0 {
                        flags_str.push(' ');
                    };
                    format!("{} {}{}", frame.format_id(), flags_str, frame.format_data())
                } else {
                    format!("{} {}", frame.format_id(), frame.format_data())
                }
            })
            .reduce(|a, b| format!("{}\n{}", a, b))
            .expect("Could not format frames")
    }

    pub fn add_frame(&mut self, id: String, data: String) {
        let new_frame = ID3v2Frame::from_user_input(id, data);
        self.frames.push(new_frame);
    }
}
