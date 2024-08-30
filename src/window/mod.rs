use crate::terminal::Terminal;
use anyhow::Result;
use raylib::{color::Color, prelude::RaylibDraw, RaylibHandle};

pub struct Window {
    terminal: Terminal,
}

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;

impl Window {
    pub fn new() -> Result<Window> {
        return Ok(Window {
            terminal: Terminal::new()?,
        });
    }

    pub fn event(&mut self, rl: &mut RaylibHandle) -> Result<()> {
        let Some(key) = rl.get_key_pressed() else {
            return Ok(());
        };

        match key {
            raylib::ffi::KeyboardKey::KEY_NULL => todo!(),
            raylib::ffi::KeyboardKey::KEY_APOSTROPHE => todo!(),
            raylib::ffi::KeyboardKey::KEY_COMMA => todo!(),
            raylib::ffi::KeyboardKey::KEY_MINUS => todo!(),
            raylib::ffi::KeyboardKey::KEY_PERIOD => todo!(),
            raylib::ffi::KeyboardKey::KEY_SLASH => todo!(),
            raylib::ffi::KeyboardKey::KEY_ZERO => todo!(),
            raylib::ffi::KeyboardKey::KEY_ONE => todo!(),
            raylib::ffi::KeyboardKey::KEY_TWO => todo!(),
            raylib::ffi::KeyboardKey::KEY_THREE => todo!(),
            raylib::ffi::KeyboardKey::KEY_FOUR => todo!(),
            raylib::ffi::KeyboardKey::KEY_FIVE => todo!(),
            raylib::ffi::KeyboardKey::KEY_SIX => todo!(),
            raylib::ffi::KeyboardKey::KEY_SEVEN => todo!(),
            raylib::ffi::KeyboardKey::KEY_EIGHT => todo!(),
            raylib::ffi::KeyboardKey::KEY_NINE => todo!(),
            raylib::ffi::KeyboardKey::KEY_SEMICOLON => todo!(),
            raylib::ffi::KeyboardKey::KEY_EQUAL => todo!(),
            raylib::ffi::KeyboardKey::KEY_A => todo!(),
            raylib::ffi::KeyboardKey::KEY_B => todo!(),
            raylib::ffi::KeyboardKey::KEY_C => todo!(),
            raylib::ffi::KeyboardKey::KEY_D => todo!(),
            raylib::ffi::KeyboardKey::KEY_E => todo!(),
            raylib::ffi::KeyboardKey::KEY_F => todo!(),
            raylib::ffi::KeyboardKey::KEY_G => todo!(),
            raylib::ffi::KeyboardKey::KEY_H => todo!(),
            raylib::ffi::KeyboardKey::KEY_I => todo!(),
            raylib::ffi::KeyboardKey::KEY_J => todo!(),
            raylib::ffi::KeyboardKey::KEY_K => todo!(),
            raylib::ffi::KeyboardKey::KEY_L => todo!(),
            raylib::ffi::KeyboardKey::KEY_M => todo!(),
            raylib::ffi::KeyboardKey::KEY_N => todo!(),
            raylib::ffi::KeyboardKey::KEY_O => todo!(),
            raylib::ffi::KeyboardKey::KEY_P => todo!(),
            raylib::ffi::KeyboardKey::KEY_Q => todo!(),
            raylib::ffi::KeyboardKey::KEY_R => todo!(),
            raylib::ffi::KeyboardKey::KEY_S => todo!(),
            raylib::ffi::KeyboardKey::KEY_T => todo!(),
            raylib::ffi::KeyboardKey::KEY_U => todo!(),
            raylib::ffi::KeyboardKey::KEY_V => todo!(),
            raylib::ffi::KeyboardKey::KEY_W => todo!(),
            raylib::ffi::KeyboardKey::KEY_X => todo!(),
            raylib::ffi::KeyboardKey::KEY_Y => todo!(),
            raylib::ffi::KeyboardKey::KEY_Z => todo!(),
            raylib::ffi::KeyboardKey::KEY_LEFT_BRACKET => todo!(),
            raylib::ffi::KeyboardKey::KEY_BACKSLASH => todo!(),
            raylib::ffi::KeyboardKey::KEY_RIGHT_BRACKET => todo!(),
            raylib::ffi::KeyboardKey::KEY_GRAVE => todo!(),
            raylib::ffi::KeyboardKey::KEY_SPACE => todo!(),
            raylib::ffi::KeyboardKey::KEY_ESCAPE => todo!(),
            raylib::ffi::KeyboardKey::KEY_ENTER => self.terminal.input(b'\n'),
            raylib::ffi::KeyboardKey::KEY_TAB => todo!(),
            raylib::ffi::KeyboardKey::KEY_BACKSPACE => todo!(),
            raylib::ffi::KeyboardKey::KEY_INSERT => todo!(),
            raylib::ffi::KeyboardKey::KEY_DELETE => todo!(),
            raylib::ffi::KeyboardKey::KEY_RIGHT => todo!(),
            raylib::ffi::KeyboardKey::KEY_LEFT => todo!(),
            raylib::ffi::KeyboardKey::KEY_DOWN => todo!(),
            raylib::ffi::KeyboardKey::KEY_UP => todo!(),
            raylib::ffi::KeyboardKey::KEY_PAGE_UP => todo!(),
            raylib::ffi::KeyboardKey::KEY_PAGE_DOWN => todo!(),
            raylib::ffi::KeyboardKey::KEY_HOME => todo!(),
            raylib::ffi::KeyboardKey::KEY_END => todo!(),
            raylib::ffi::KeyboardKey::KEY_CAPS_LOCK => todo!(),
            raylib::ffi::KeyboardKey::KEY_SCROLL_LOCK => todo!(),
            raylib::ffi::KeyboardKey::KEY_NUM_LOCK => todo!(),
            raylib::ffi::KeyboardKey::KEY_PRINT_SCREEN => todo!(),
            raylib::ffi::KeyboardKey::KEY_PAUSE => todo!(),
            raylib::ffi::KeyboardKey::KEY_F1 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F2 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F3 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F4 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F5 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F6 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F7 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F8 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F9 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F10 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F11 => todo!(),
            raylib::ffi::KeyboardKey::KEY_F12 => todo!(),
            raylib::ffi::KeyboardKey::KEY_LEFT_SHIFT => todo!(),
            raylib::ffi::KeyboardKey::KEY_LEFT_CONTROL => todo!(),
            raylib::ffi::KeyboardKey::KEY_LEFT_ALT => todo!(),
            raylib::ffi::KeyboardKey::KEY_LEFT_SUPER => todo!(),
            raylib::ffi::KeyboardKey::KEY_RIGHT_SHIFT => todo!(),
            raylib::ffi::KeyboardKey::KEY_RIGHT_CONTROL => todo!(),
            raylib::ffi::KeyboardKey::KEY_RIGHT_ALT => todo!(),
            raylib::ffi::KeyboardKey::KEY_RIGHT_SUPER => todo!(),
            raylib::ffi::KeyboardKey::KEY_KB_MENU => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_0 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_1 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_2 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_3 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_4 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_5 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_6 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_7 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_8 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_9 => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_DECIMAL => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_DIVIDE => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_MULTIPLY => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_SUBTRACT => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_ADD => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_ENTER => todo!(),
            raylib::ffi::KeyboardKey::KEY_KP_EQUAL => todo!(),
            raylib::ffi::KeyboardKey::KEY_BACK => todo!(),
            raylib::ffi::KeyboardKey::KEY_VOLUME_UP => todo!(),
            raylib::ffi::KeyboardKey::KEY_VOLUME_DOWN => todo!(),
        }
    }

    pub fn run(mut self) {
        let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("terminal").build();

        while !rl.window_should_close() {
            if let Err(err) = self.event(&mut rl) {
                panic!("terminal failed: {err}");
            };

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
        }
    }
}
