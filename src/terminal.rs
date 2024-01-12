use std::io::{stdin, stdout, Error, Stdout, Write};

use termion::{
    clear::{All, CurrentLine},
    color,
    cursor::{Goto, Hide, Show},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2), // two lines for status bar
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen() {
        print!("{}", All);
    }

    pub fn clear_current_line() {
        print!("{}", CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);

        let x = x as u16;
        let y = y as u16;
        print!("{}", Goto(x, y));
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn cursor_hide() {
        print!("{Hide}");
    }

    pub fn cursor_show() {
        print!("{Show}");
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
