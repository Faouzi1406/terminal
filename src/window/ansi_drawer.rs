use ansi_parser::AnsiParser;
use anyhow::Result;
use raylib::{color::Color, drawing::RaylibDrawHandle, prelude::RaylibDraw};

use super::cursor::Cursor;

pub struct AnsiDrawer {
    cursor: Cursor,
}

impl AnsiDrawer {
    pub fn new() -> Self {
        return Self {
            cursor: Cursor::default(),
        };
    }

    pub fn draw_ansi_str<'a>(&mut self, rl: RaylibDrawHandle, str: &'a str) -> Result<()> {
        let output: Vec<ansi_parser::Output> = str.ansi_parse().map(|x| x).collect();
        self.draw_output(rl, output);
        Ok(())
    }

    fn draw_output(&mut self, mut rl: RaylibDrawHandle, output: Vec<ansi_parser::Output>) {
        println!("{:#?}", output);
        for element in output {
            match element {
                ansi_parser::Output::TextBlock("\r\n") => self.cursor.y += 1,
                ansi_parser::Output::TextBlock(text) => rl.draw_text(
                    text,
                    (14 * self.cursor.x) as i32,
                    (14 * self.cursor.y) as i32,
                    14,
                    Color::WHITE,
                ),
                ansi_parser::Output::Escape(escape) => match escape {
                    ansi_parser::AnsiSequence::CursorUp(amount) => self.cursor.y -= amount,
                    ansi_parser::AnsiSequence::CursorDown(amount) => self.cursor.y += amount,
                    ansi_parser::AnsiSequence::CursorPos(x, y) => {
                        self.cursor.x = x;
                        self.cursor.y = y;
                    }
                    ansi_parser::AnsiSequence::CursorForward(amount) => self.cursor.x += amount,
                    ansi_parser::AnsiSequence::CursorBackward(amount) => self.cursor.x -= amount,
                    _ => (),
                },
            }
        }
    }
}
