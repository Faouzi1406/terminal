pub mod cursor;

use crate::terminal::cursor::Cursor;
use anyhow::Result;

pub struct Terminal {
    cursor: Cursor,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            cursor : Cursor::default()
        }
    }

    pub fn input(&mut self, char: u8) -> Result<()> {
        self.pty.master.write(&[char])?;
        return Ok(());
    }

    pub fn rec(&mut self) -> Result<TerminalCommand> {
        Ok(self.rec.0.recv()?)
    }
}
