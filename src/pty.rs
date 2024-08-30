use anyhow::Result;
use rustix::{path::Arg, pty};
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    os::fd::{AsFd, BorrowedFd},
    process::{Child, Command},
};

pub struct Pty {
    pub process: Child,
    pub master: File,
}

#[derive(Debug)]
pub enum PtyInitError {
    CouldNotSetupSlaveProcess,
    CouldNotGetSlaveLoc,
    CouldNotOpenSlave,
    CouldNotOpenPtmx,
    CouldNotUnlockPtmx,
    CouldNotGrantPtmx,
}

impl Display for PtyInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var_name = match self {
            PtyInitError::CouldNotGetSlaveLoc => {
                write!(f, "Could not get the file path for slave.")
            }
            PtyInitError::CouldNotOpenSlave => write!(f, "Could not open the slave file."),
            PtyInitError::CouldNotOpenPtmx => write!(f, "Could not open ptmx file."),
            PtyInitError::CouldNotUnlockPtmx => write!(f, "Could not unlock ptmx."),
            PtyInitError::CouldNotGrantPtmx => write!(f, "Could not grant ptmx."),
            PtyInitError::CouldNotSetupSlaveProcess => write!(f, "Could not setup slave process."),
        };
        var_name
    }
}
impl Error for PtyInitError {}

fn setup_slave_unlock_rights(master_fd: BorrowedFd) -> Result<()> {
    if !pty::grantpt(master_fd).is_ok() {
        return Err(PtyInitError::CouldNotGrantPtmx.into());
    }

    if !pty::unlockpt(master_fd).is_ok() {
        return Err(PtyInitError::CouldNotUnlockPtmx.into());
    }

    return Ok(());
}

fn get_slave_location(master_fd: BorrowedFd) -> Result<String> {
    let Ok(slave_location) = pty::ptsname(master_fd, vec![]) else {
        return Err(PtyInitError::CouldNotGetSlaveLoc.into());
    };
    return Ok(slave_location.to_string_lossy().to_string());
}

fn setup_slave(shell: &str, slave: File) -> Result<Child> {
    let mut builder = Command::new(shell);
    builder.stdin(slave.try_clone()?);
    builder.stdout(slave.try_clone()?);
    builder.stderr(slave.try_clone()?);
    return Ok(builder.spawn()?);
}

impl Pty {
    pub fn new(shell: &str) -> Result<Pty> {
        let Ok(master) = File::options().read(true).write(true).open("/dev/ptmx") else {
            return Err(PtyInitError::CouldNotOpenPtmx.into());
        };
        let master_fd = master.as_fd();

        setup_slave_unlock_rights(master_fd)?;
        let location = get_slave_location(master_fd)?;

        let Ok(slave) = File::options().read(true).write(true).open(location) else {
            return Err(PtyInitError::CouldNotOpenSlave.into());
        };
        let Ok(process) = setup_slave(shell, slave) else {
            return Err(PtyInitError::CouldNotSetupSlaveProcess.into());
        };

        return Ok(Pty { master, process });
    }
}
