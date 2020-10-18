use easy_reader::EasyReader;
use regex::Regex;
use std::{fs::File, io::Error as IoError};

pub enum MediaService {
    SoundCloud,
    YouTube,
    Unknown,
}

pub struct CytubeMedia {
    pub title: String,
    pub service: MediaService,
    pub id: String,
}

impl CytubeMedia {
    pub fn new(title: String, service_string: String, id: String) -> CytubeMedia {
        let service = match service_string.as_str() {
            "sc" => MediaService::SoundCloud,
            "yt" => MediaService::YouTube,
            _ => MediaService::Unknown,
        };

        CytubeMedia { title, service, id }
    }
}

pub fn get_now_playing(cytube_log: &str) -> Result<Option<CytubeMedia>, IoError> {
    let regex = Regex::new(r"Now playing: (.+) \(([a-z]{2}):([\w_\-]{11,})\)").unwrap();
    let log_file = File::open(cytube_log)?;
    let mut reader = EasyReader::new(log_file)?;
    reader.eof();

    let mut attempts = 0;
    let max_attempts = 10;

    while let Some(line) = reader.prev_line()? {
        attempts += 1;

        if let Some(foo) = regex.captures(&line) {
            return Ok(Some(CytubeMedia::new(
                foo[1].to_string(),
                foo[2].to_string(),
                foo[3].to_string(),
            )));
        }

        if attempts >= max_attempts {
            log::warn!(
                "Unable to find CyTube now playing line in {} attempts!",
                max_attempts
            );
        }
    }

    Ok(None)
}
