use std::path::{Path, PathBuf};
use std::str::{self, FromStr};

use std::error::{Error, Result};

#[derive(Clone, Debug)]
pub enum Command {
    Auth,
    Cwd(PathBuf),
    List(Option<PathBuf>),
    Mkd(PathBuf),
    NoOp,
    Port(u16),
    Pasv,
    Pwd,
    Quit,
    Retr(PathBuf),
    Rmd(PathBuf),
    Stor(PathBuf),
    Syst,
    Type(TransferType),
    CdUp,
    Unknown(String),
    User(String),
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Command::Auth => "AUTH",
            Command::Cwd(_) => "CWD",
            Command::List(_) => "LIST",
            Command::Pasv => "PASV",
            Command::Port(_) => "PORT",
            Command::Pwd => "PWD",
            Command::Quit => "QUIT",
            Command::Retr(_) => "RETR",
            Command::Stor(_) => "STOR",
            Command::Syst => "SYST",
            Command::Type(_) => "TYPE",
            Command::User(_) => "USER",
            Command::CdUp => "CDUP",
            Command::Mkd(_) => "MKD",
            Command::Rmd(_) => "RMD",
            Command::NoOp => "NOOP",
            Command::Unknown(_) => "UNKN", // doesn't exist
        }
    }
}


impl Command {
    pub fn new(input: Vec<u8>) -> Result<Self, E> {
        let mut iter = input.split(|&byte| byte == b' ');
        let mut command = iter.next().ok_or_else(
            || Error::Msg("empty command".to_string()))?.to_vec();
        to_uppercase(&mut command);
        let data = iter.next().ok_or_else(|| Error::Msg("no command parameter".to_string()));
        let command = match command.as_slice() {
            b"AUTH" => Command::Auth,
            b"CWD" => Command::Cwd(data.and_then(|bytes| Ok(Path::new(str::from_utf8(bytes)?).to_path_buf()))?),
            b"LIST" => Command::List(data.and_then(|bytes| Ok(Paht::new(str::from_utf8(bytes)?).to_path_buf())).ok()),
            b"PASV" => Command::Pasv,
            b"PORT" => {
                let addr = data?.split(|&byte| byte == b',')
                    .filter_map(|bytes| str::from_utf8(bytes).ok()
                        .and_then(|string| u8::from_str(string).ok()))
                    .collect::<Vec<u8>>();
                if addr.len() != 6 {
                    return Err("Invalid address/port".into());
                }

                let port = (addr[4] as u16) << 8 | (addr[5] as u16);
                if port <= 1024 {
                    return Err("Port can't be less than 10025".into());
                }
                Command::Port(port)
            },

        }
    }
}