use clap::{Parser, Subcommand};
use std::path::PathBuf;
use utils::log;

use mp3_file::{is_string_known_id3v2_id, is_string_valid_id3v2_id, Mp3File, KNOWN_ID3V2_IDS};

mod mp3_file;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        value_parser,
        help = "Try to make all printed values as human readable as possible",
        long,
        short
    )]
    human_readable: bool,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Read {
        #[clap(value_parser, help = "The MP3 file to be used.")]
        file: PathBuf,
        #[clap(value_parser, long, help = "Also prints the frame's flags")]
        frame_flags: bool,
    },
    Write {
        #[clap(value_parser, help = "The MP3 file to be used.")]
        file: PathBuf,
        #[clap(value_parser, help = "The ID of the frame")]
        frame_id: String,
        #[clap(value_parser, help = "The data of the frame")]
        data: String,
    },
    Delete {
        #[clap(value_parser, help = "The MP3 file to be used.")]
        file: PathBuf,
        #[clap(value_parser, help = "The ID of the frame to delete")]
        frame_id: String,
        #[clap(value_parser, short, long, help = "The ID of the frame to delete")]
        frame_index: Option<u32>,
    },
    Edit {
        #[clap(value_parser, help = "The MP3 file to be used.")]
        file: PathBuf,
        #[clap(value_parser, help = "The ID of the frame to delete")]
        frame_id: String,
        #[clap(value_parser, help = "The data of the frame that'll be replaced")]
        data: String,
        #[clap(value_parser, short, long, help = "The ID of the frame to delete")]
        frame_index: Option<u32>,
    },
    ShowKnownFrameIds {},
}

fn main() -> Result<(), ()> {
    log::init(None);
    let args = Args::parse();
    match args.command {
        Command::Read {
            file: file_path,
            frame_flags,
        } => {
            let mp3_file = Mp3File::from_path(&file_path)?;
            println!(
                "{}",
                mp3_file.format_frames(frame_flags, args.human_readable)
            );
        }
        Command::Write {
            file: file_path,
            data,
            frame_id,
        } => {
            if !validate_frame_id(&frame_id) {
                return Err(());
            }
            let mut mp3_file = Mp3File::from_path(&file_path)?;
            mp3_file.add_frame(frame_id, data);
            mp3_file.write_to_file(&file_path)?;
        }
        Command::Edit {
            file: file_path,
            frame_id,
            data,
            frame_index,
        } => {
            let unwraped_frame_index = frame_index.unwrap_or(1);
            if !validate_frame_id(&frame_id) || !validate_frame_index(unwraped_frame_index) {
                return Err(());
            }
            let zero_indexed_frame = unwraped_frame_index - 1;

            let mut mp3_file = Mp3File::from_path(&file_path)?;

            mp3_file
                .edit_frame(&frame_id, data, zero_indexed_frame)
                .or_else(|largest_found_index| {
                    error_no_frame_with_id_found(
                        &frame_id,
                        unwraped_frame_index,
                        largest_found_index,
                    );
                    Err(())
                })?;

            mp3_file.write_to_file(&file_path)?;
        }
        Command::Delete {
            file: file_path,
            frame_index,
            frame_id,
        } => {
            let unwraped_frame_index = frame_index.unwrap_or(1);
            if !validate_frame_id(&frame_id) || !validate_frame_index(unwraped_frame_index) {
                return Ok(());
            }
            let zero_indexed_frame = unwraped_frame_index - 1;
            println!(
                "Removing the {}{} frame of ID \"{}\"",
                unwraped_frame_index,
                ordinal_numeral(unwraped_frame_index),
                frame_id
            );
            let mut mp3_file = Mp3File::from_path(&file_path)?;
            match mp3_file.remove_frame(&frame_id, zero_indexed_frame) {
                Ok(_) => (),
                Err(largest_found_index) => {
                    error_no_frame_with_id_found(
                        &frame_id,
                        zero_indexed_frame,
                        largest_found_index,
                    );
                    return Ok(());
                }
            }
            mp3_file.write_to_file(&file_path)?;
        }
        Command::ShowKnownFrameIds {} => {
            for (id, description) in KNOWN_ID3V2_IDS.iter() {
                println!("{} - {}", id, description);
            }
        }
    };
    Ok(())
}

fn ordinal_numeral(number: u32) -> &'static str {
    match number {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

fn validate_frame_index(frame_index: u32) -> bool {
    if frame_index == 0 {
        log::error("The frame index starts at one, not zero.".to_string());
        false
    } else {
        true
    }
}

fn error_no_frame_with_id_found(frame_id: &str, index: u32, largest_found_index: u32) {
    if largest_found_index == 0 {
        log::error(format!("Error: No frame found with id \"{}\"", frame_id,));
    } else if largest_found_index == 1 {
        log::error(format!(
            "There is only 1 frame with id \"{}\". You tried to remove the {}{}",
            frame_id,
            index,
            ordinal_numeral(index)
        ))
    } else {
        log::error(format!(
            "There are only {} frames with id \"{}\". You tried to remove the {}{}",
            largest_found_index.to_string(),
            frame_id,
            index,
            ordinal_numeral(index)
        ))
    }
}

fn validate_frame_id(frame_id: &str) -> bool {
    if !is_string_valid_id3v2_id(&frame_id) {
        log::error(format!(
            "Provided frame id \"{}\" is not valid. It must be a four-character word composed exclusively of numbers or uppercase letters",
            frame_id,
        ));
        return false;
    }
    if !is_string_known_id3v2_id(&frame_id) {
        log::warn(format!(
            "Provided frame id \"{}\" is not a known id. The operation will still be executed.",
            frame_id,
        ));
    }
    true
}
