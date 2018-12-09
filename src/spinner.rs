//! the thing which shows we're waiting for a long task

use std::io::{self, Write};
use termion::color;

use screens::Screen;

pub trait Spinner {
    fn write_spinner(&mut self, spinning: bool) -> io::Result<()>;
}

impl Spinner for Screen {
    fn write_spinner(&mut self, spinning: bool) -> io::Result<()> {
        let y = self.h - 1;
        write!(
            self.stdout,
            "{}{}{}{}{}{}",
            termion::cursor::Goto(1, y),
            color::Bg(color::AnsiValue::grayscale(2)),
            color::Fg(color::AnsiValue::grayscale(10)),
            match spinning {
                true => "⌛",
                false => " ",
            },
            color::Bg(color::Reset),
            color::Fg(color::Reset),
        )?;
        self.stdout.flush()?;
        Ok(())
    }
}

