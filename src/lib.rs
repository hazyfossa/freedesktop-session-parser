mod utils;

use std::{
    collections::HashMap,
    io::{self, BufReader, Lines, Read},
};

use snafu::{OptionExt, Snafu, ensure_whatever, whatever};

#[derive(Debug, Snafu)]

pub enum ParseError {
    EOF,
    #[snafu(context(false))]
    IoError {
        source: io::Error,
    },
    #[snafu(whatever)]
    ParseError {
        message: String,
    },
}

pub struct LocaleString {
    default: String,
    lc_lookup: HashMap<String, String>,
}

pub enum Kind {
    X11,
    // most commonly wayland
    Application,
}

with_builder!(
    pub struct SessionEntry {
        name: #required LocaleString,
        comment: #optional LocaleString,
        kind: #required Kind,
        desktop_names: #optional Vec<String>,
    }
);

struct Parser<R> {
    builder: SessionEntryBuilder,
    reader: Lines<BufReader<R>>,
}

impl<R: Read> Parser<R> {
    // Reads the next line, skipping comments
    fn read_line(&mut self) -> Result<String, ParseError> {
        let line = self.reader.next().ok_or(ParseError::EOF)??;

        ensure_whatever!(
            !line.starts_with("["),
            "This parser does not support groups"
        );

        let skip = line.is_empty() || line.starts_with("#");

        match skip {
            true => self.read_line(),
            false => Ok(line),
        }
    }

    fn read_next(&mut self) -> Result<(), ParseError> {
        let line = self.read_line()?;

        let (k, v) = line.split_once("=").whatever_context::<_, ParseError>(
            "Cannot parse as a key-value pair: cannot split at =",
        )?;

        let (k, v) = (k.trim_end(), v.trim_start());

        match k {
            "Type" => self.builder.set_kind(match v {
                "Application" => Kind::Application,
                "XSession" => Kind::X11,
                other => whatever!("Unsupported entry kind: {other}"),
            }),
            _skip_other => return Ok(()),
        };

        Ok(())
    }
}
