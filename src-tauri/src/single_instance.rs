use byteorder::{ReadBytesExt, WriteBytesExt};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::slice::SliceIndex;
use tauri::{Config, PathResolver, Window};

const LOCAL_SOCEKT_NAME: &str = "SINGLE_INSTANCE";

pub fn run_sev(window: Window) {
    let socket = LocalSocketListener::bind(LOCAL_SOCEKT_NAME).expect("Init Local Socket Failure");

    while let Ok(mut stream) = socket.accept() {
        while let Ok(packet) = Packet::read(&mut stream) {
            if packet.msg.as_str() == "CREATE_INSTANCE" {
                window.show().expect("Cannot Show Windows");
                break;
            }
        }
    }
}

pub fn try_start() -> io::Result<bool> {
    Ok(match LocalSocketStream::connect(LOCAL_SOCEKT_NAME) {
        Ok(mut conn) => {
            Packet {
                msg: "CREATE_INSTANCE".into(),
            }.write(&mut conn)?;
            false
        }
        Err(err) => {
            true
        }
    }
    )
}

struct Packet {
    msg: String,
}

impl Packet {
    fn read<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let data = r.read_u8()?;
        let mut vec = vec![0u8; data as usize];
        r.read_exact(&mut vec)?;

        Ok(Self {
            msg: String::from_utf8(vec).unwrap(),
        })
    }

    fn write<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_u8(self.msg.as_bytes().len() as _)?;
        w.write_all(self.msg.as_bytes())?;

        Ok(())
    }
}
