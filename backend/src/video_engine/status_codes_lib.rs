/* use futures_util::lock::Mutex;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::TcpStream};
use super::defs::status_codes::{Error, StatusCode};

use crate::configs::tcp_comunication_settings;

pub async fn read_from_video() -> Result<StatusCode,Error>{
    let mut buffer = [0u8; 128];
    let read_bytes = {
        let mut stream = stream.lock().await;
        tokio::time::timeout(
            tcp_comunication_settings::get_read_timeout(),
            stream.read(&mut buffer),
        )
        .await
    };
    if let Ok(status) = StatusCode::try_from(&buffer[..])
}

pub async fn successfull(stream: Arc<Mutex<TcpStream>>) -> Result<bool,Error> {
    let mut buffer = [0u8; 128];
    let read_bytes = {
        let mut stream = stream.lock().await;
        tokio::time::timeout(
            tcp_comunication_settings::get_read_timeout(),
            stream.read(&mut buffer),
        )
        .await
    };
    if let Ok(status) = StatusCode::try_from(&buffer[..]){
        match status {
            StatusCode::Accepted  => {
                ()
            },
            StatusCode::Executed => return Ok(true) ,
            _ => return Ok(false)
        }
    } 
    else {return  Err(Error::InvalidStatusCode);}
} */
