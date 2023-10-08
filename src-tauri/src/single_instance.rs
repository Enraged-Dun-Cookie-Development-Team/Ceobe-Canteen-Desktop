use byteorder::{ReadBytesExt, WriteBytesExt};
use interprocess::local_socket::LocalSocketStream;
use std::io;
use std::io::{Read, Write};

use tauri::Window;
use tracing::{info, instrument};

#[allow(dead_code)]
const LOCAL_SOCKET_NAME: &str = "CEOBE_LISTENING";
const TEST_CEOBE_IN: &str = "TEST_CEOBE_IN";
const CEOBE_IN: &str = "CEOBE_IN";
#[instrument(name = "SingletonService", skip_all)]
#[allow(unused_variables)]
pub fn run_sev(window: Window) {
    #[cfg(not(unix))]
    {
        let socket = match interprocess::local_socket::LocalSocketListener::bind(LOCAL_SOCKET_NAME)
        {
            Ok(socket) => socket,
            Err(err) => {
                tracing::error!(step="CreateLocalSocket", error = %err);
                tauri::Manager::app_handle(&window).exit(1);
                info!(step = "CeobeExit", action = "QuitSelf");
                return;
            }
        };
        info!(step = "WaitingForCeobe");
        'listen: while let Ok(mut stream) = socket.accept() {
            while let Ok(packet) = Packet::read(&mut stream) {
                if packet.msg.as_str() == CEOBE_IN {
                    info!(step = "IncomeCeobe", action = "ShowSelf");
                    show_window(&window).expect("Cannot Reopen window");
                    break;
                } else if packet.msg.as_str() == TEST_CEOBE_IN {
                    info!(step = "IncomeTestCeobe", action = "Close");
                    tauri::Manager::app_handle(&window).exit(1);
                    break 'listen;
                }
            }
        }
        drop(socket);
    }
}

#[allow(dead_code)]
fn show_window(window: &Window) -> tauri::Result<()> {
    window.show()?;
    if window.is_minimized()? {
        window.unminimize()?
    }
    window.set_focus()?;
    Ok(())
}
#[instrument(name = "SingletonClient", err)]
pub fn try_start() -> io::Result<bool> {
    Ok(match LocalSocketStream::connect(LOCAL_SOCKET_NAME) {
        Ok(mut conn) => {
            if cfg!(debug_assertions) {
                info!(step = "FindListenCeobe", action = "SendTestCeobeIn");
                Packet {
                    msg: TEST_CEOBE_IN.into(),
                }
            } else {
                info!(step = "FindListenCeobe", action = "SendCeobeIn");
                Packet {
                    msg: CEOBE_IN.into(),
                }
            }
            .write(&mut conn)?;
            false
        }
        Err(_) => {
            info!(step = "NotListenCeobeFind");
            true
        }
    })
}

struct Packet {
    msg: String,
}

impl Packet {
    #[allow(dead_code)]
    fn read<R: Read>(r: &mut R) -> io::Result<Self> {
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
