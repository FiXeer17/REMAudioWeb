use super::defs::status_codes::StatusCode;
use crate::configs::tcp_comunication_settings;
use futures_util::lock::Mutex;
use log::warn;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::TcpStream};
use super::defs::errors::Error;


pub async fn read_from_video(stream: Arc<Mutex<TcpStream>>) -> Result<StatusCode, Error> {
    let mut buffer = [0u8; 128];
    let read_bytes = {
        let mut stream = stream.lock().await;
        tokio::time::timeout(
            tcp_comunication_settings::get_read_timeout(),
            stream.read(&mut buffer),
        )
        .await
    };

    match read_bytes {
        Ok(Ok(length)) => {
            if length > 0 {
                StatusCode::try_from(&buffer[..length])
            } else {
                println!("closed by remote peer");
                Err(Error::ClosedByRemotePeer)
            }
        }
        Ok(Err(_)) => Err(Error::InvalidStatusCode),
        Err(_) => Err(Error::TimedOut),
    }
}

pub async fn successfull(stream: Arc<Mutex<TcpStream>>) -> Result<bool, Error> {
    loop {
        let status_code = read_from_video(stream.clone()).await?;
        match status_code {
            StatusCode::Accepted => continue,
            StatusCode::Executed => return Ok(true),
            StatusCode::NotExecutable => {
                warn!("Cannot execute command");
                return Ok(false);
            }
            StatusCode::SyntaxError => {
                warn!("Invalid syntax");
                return Ok(false);
            }
        }
    }
}
