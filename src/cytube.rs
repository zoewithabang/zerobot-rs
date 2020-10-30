use easy_reader::EasyReader;
use regex::Regex;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::Error as IoError,
};

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

impl Display for MediaService {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            MediaService::SoundCloud => write!(f, "SoundCloud"),
            MediaService::YouTube => write!(f, "YouTube"),
            MediaService::Unknown => write!(f, "Website"),
        }
    }
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

    pub fn get_url(&self) -> String {
        match self.service {
            MediaService::SoundCloud => self.id.clone(),
            MediaService::YouTube => format!("https://www.youtube.com/watch?v={}", self.id),
            MediaService::Unknown => self.id.clone(),
        }
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

        if let Some(captures) = regex.captures(&line) {
            return Ok(Some(CytubeMedia::new(
                captures[1].to_string(),
                captures[2].to_string(),
                captures[3].to_string(),
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
