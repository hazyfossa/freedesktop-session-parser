mod utils;

use std::io::{self, BufRead, BufReader, Lines, Read};

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

// pub struct LocaleString {
//     default: String,
//     lc_lookup: HashMap<String, String>,
// }

type LocaleString = String;

pub enum Kind {
    X11,
    // most commonly wayland
    Application,
}

with_builder!(
    pub struct SessionEntry {
        pub name: #required LocaleString,
        pub comment: #optional LocaleString,
        pub kind: #required Kind,
        pub desktop_names: #optional String,
    }
);

struct Parser<R> {
    builder: SessionEntryBuilder,
    reader: Lines<BufReader<R>>,
}

impl<R: Read> Parser<R> {
    fn new(reader: BufReader<R>) -> Self {
        Self {
            builder: SessionEntryBuilder::new(),
            reader: reader.lines(),
        }
    }

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
            "Name" => self.builder.set_name(v.to_string()),
            "Comment" => self.builder.set_comment(v.to_string()),
            "DesktopNames" => self.builder.set_desktop_names(v.to_string()),
            _skip_other => return Ok(()),
        };

        Ok(())
    }

    fn read_all(mut self) -> Result<SessionEntry, ParseError> {
        loop {
            match self.read_next() {
                Ok(()) => (),
                Err(ParseError::EOF) => break,
                Err(e) => return Err(e),
            }
        }

        self.builder.finalize()
    }
}

pub fn parse(reader: BufReader<impl Read>) -> Result<SessionEntry, ParseError> {
    Parser::new(reader).read_all()
}
