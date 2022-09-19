use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};

use super::{id3v2_frame::ID3v2Frame, id3v2_header::ID3v2Header};
use crate::utils::log;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Mp3File {
    header: ID3v2Header,
    frames: Vec<ID3v2Frame>,
    read_file: File,
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
                self.frames[index].edit_data(new_data);
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

    pub fn from_path(path: &Path) -> Result<Mp3File, ()> {
        let mut read_file = File::open(path).or_else(|error| {
            let path = path.to_string_lossy();
            match error.kind() {
                ErrorKind::NotFound => log::error(format!("File {} does not exist.", path)),
                ErrorKind::PermissionDenied => {
                    log::error(format!("Failed to open file {}. Permission denied", path))
                }
                _ => log::error(format!(
                    "Failed to open file {}. Unknown error: {}",
                    path,
                    error.kind()
                )),
            }
            Err(())
        })?;

        let header = ID3v2Header::from_read_file(&mut read_file)?;
        let mut frames: Vec<ID3v2Frame> = vec![];
        while ID3v2Frame::has_new_frame(&mut read_file) {
            let new_frame = ID3v2Frame::from_read_file(&mut read_file)?;
            frames.push(new_frame);
        }

        Ok(Mp3File {
            frames,
            header,
            read_file,
        })
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

    pub fn write_to_file(&mut self, read_file_path: &PathBuf) -> Result<(), ()> {
        let mut write_file_path = read_file_path.clone();
        write_file_path.set_extension("mp3.temp");
        let mut write_file = File::create(&write_file_path).or_else(|error| {
            let file_path = write_file_path.to_string_lossy();
            match error.kind() {
                ErrorKind::PermissionDenied => log::error(format!(
                    "Failed to open file {} for writing. Permission denied",
                    file_path,
                )),
                _ => log::error(format!(
                    "Failed to open file {}. Unknown error: {}",
                    file_path,
                    error.kind()
                )),
            };
            Err(())
        })?;
        let size = self.calculate_id3v2_size();
        self.header.write_to_file(&mut write_file, size)?;
        for frame in self.frames.iter() {
            frame.write_to_file(&mut write_file)?;
        }

        let mut buffer = [0; 16 * (2 ^ 10)];
        while self
            .read_file
            .read(&mut buffer)
            .expect("Failed to read file chunk")
            > 0
        {
            write_file
                .write(&buffer)
                .expect("Failed to write to file chunk");
        }
        fs::rename(write_file_path, read_file_path)
            .expect("Failed to rename temp file to actual file.");
        Ok(())
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
            .unwrap_or("No frames in file.".to_string())
    }

    pub fn add_frame(&mut self, id: String, data: String) {
        let new_frame = ID3v2Frame::from_user_input(id, data);
        self.frames.push(new_frame);
    }
}
