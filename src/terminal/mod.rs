use crate::pty::Pty;
use anyhow::{anyhow, Result};
use std::{
    io::{Read, Write},
    sync::{Arc, Mutex},
    thread,
};

pub struct Terminal {
    pty: Pty,
    buf: Arc<Mutex<Vec<u8>>>,
}

impl Terminal {
    pub fn new() -> Result<Terminal> {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let pty = Pty::new("/bin/zsh")?;

        let buff2 = buffer.clone();
        let mut master = pty.master.try_clone()?;
        thread::spawn(move || {
            let mut buf = [0; 1];

            while let Ok(count) = master.read(&mut buf) {
                match count {
                    0 => break,
                    1 => {
                        if let Ok(mut locked) = buff2.lock() {
                            locked.push(buf[0]);
                        }
                    }
                    _ => unreachable!("there is only room for 1 byte in the array"),
                }
            }
        });

        return Ok(Terminal { pty, buf: buffer });
    }

    pub fn input(&mut self, char: u8) -> Result<()> {
        self.pty.master.write(&[char])?;
        return Ok(());
    }

    pub fn buffer(&self) -> Result<&Vec<u8>> {
        let locked = self.buf.lock();
        match locked {
            Ok(ref lock) => Ok(lock),
            Err(_) => Err(anyhow!("was unable to lock buffer")),
        }
    }
}
