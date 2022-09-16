use clap::{Parser, Subcommand};
use std::path::PathBuf;

use colored::*;
use mp3_file::{is_string_known_id3v2_id, is_string_valid_id3v2_id, Mp3File};

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
}

fn main() -> () {
    let args = Args::parse();
    match args.command {
        Command::Read { file, frame_flags } => {
            let mut mp3_file = Mp3File::from_path(&file);
            mp3_file.read();
            // Unwrap used because this literally should never panic
            println!(
                "{}",
                mp3_file.format_frames(frame_flags, args.human_readable)
            );
        }
        Command::Write {
            file,
            data,
            frame_id,
        } => {
            if !validate_frame_id(&frame_id) {
                return ();
            }
            let mut write_temp_path = file.clone();
            write_temp_path.set_extension(".mp3.temp");

            let mut mp3_file = Mp3File::from_path(&file);
            mp3_file.read();
            mp3_file.add_frame(frame_id, data);
            mp3_file.write_to_file(&write_temp_path);
        }
        Command::Edit {
            file,
            frame_id,
            data,
            frame_index,
        } => {
            let unwraped_frame_index = frame_index.unwrap_or(1);
            if !validate_frame_id(&frame_id) || !validate_frame_index(unwraped_frame_index) {
                return ();
            }
            let zero_indexed_frame = unwraped_frame_index - 1;
            let mut write_temp_path = file.clone();
            write_temp_path.set_extension(".mp3.temp");

            let mut mp3_file = Mp3File::from_path(&file);
            mp3_file.read();

            match mp3_file.edit_frame(&frame_id, data, zero_indexed_frame) {
                Ok(_) => (),
                Err(largest_found_index) => {
                    error_no_frame_with_id_found(
                        &frame_id,
                        unwraped_frame_index,
                        largest_found_index,
                    );
                    return ();
                }
            };

            mp3_file.write_to_file(&write_temp_path);
        }
        Command::Delete {
            file,
            frame_index,
            frame_id,
        } => {
            let unwraped_frame_index = frame_index.unwrap_or(1);
            if !validate_frame_id(&frame_id) || !validate_frame_index(unwraped_frame_index) {
                return ();
            }
            let zero_indexed_frame = unwraped_frame_index - 1;
            println!(
                "Removing the {}{} frame of ID \"{}\"",
                unwraped_frame_index,
                ordinal_numeral(unwraped_frame_index),
                frame_id
            );
            let mut write_temp_path = file.clone();
            write_temp_path.set_extension("mp3.temp");

            let mut mp3_file = Mp3File::from_path(&file);
            mp3_file.read();
            match mp3_file.remove_frame(&frame_id, zero_indexed_frame) {
                Ok(_) => (),
                Err(largest_found_index) => {
                    error_no_frame_with_id_found(
                        &frame_id,
                        zero_indexed_frame,
                        largest_found_index,
                    );
                    return ();
                }
            }
            mp3_file.write_to_file(&write_temp_path);
        }
    }
}

fn ordinal_numeral(number: u32) -> &'static str {
    if number == 1 {
        "st"
    } else if number == 2 {
        "nd"
    } else if number == 3 {
        "rd"
    } else {
        "th"
    }
}

fn validate_frame_index(frame_index: u32) -> bool {
    if frame_index == 0 {
        println!(
            "{}",
            "Error: The frame index starts at one, not zero.".red()
        );
        false
    } else {
        true
    }
}

fn error_no_frame_with_id_found(frame_id: &str, index: u32, largest_found_index: u32) {
    if largest_found_index == 0 {
        println!(
            "{}{}{}",
            "Error: No frame found with id \"".red(),
            frame_id.red(),
            "\"".red()
        );
    } else if largest_found_index == 1 {
        println!(
            "{}{}{}{}{}",
            "Error: There is only 1 frame with id \"".red(),
            frame_id.red(),
            "\". You tried to remove the ".red(),
            index.to_string().red(),
            ordinal_numeral(index).red()
        )
    } else {
        println!(
            "{}{}{}{}{}{}{}",
            "Error: There are only ".red(),
            largest_found_index.to_string().red(),
            " frames with id \"".red(),
            frame_id.red(),
            "\". You tried to remove the ".red(),
            index.to_string().red(),
            ordinal_numeral(index).red()
        )
    }
}

fn validate_frame_id(frame_id: &str) -> bool {
    if !is_string_valid_id3v2_id(&frame_id) {
        println!(
            "{}{}{}",
            "Error: Provided frame id \"".red(),
            frame_id.red(),
            "\" is not valid. It must be a four-character word composed exclusively of numbers or uppercase letters".red()
        );
        return false;
    }
    if !is_string_known_id3v2_id(&frame_id) {
        println!(
            "{}{}{}",
            "Warning: Provided frame id \"".yellow(),
            frame_id.yellow(),
            "\" is not a known id. The operation will still be executed.".yellow()
        );
        return false;
    }
    true
}
