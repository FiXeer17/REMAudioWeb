
use std::sync::Arc;
use futures_util::lock::Mutex;
use log::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use super::defs::camera_presets::{READ_PRESET, RECALL};
use super::defs::errors::Error;

pub fn call_preset(pq: String) -> Result<Vec<u8>, Error> {
    let Ok(pq) = pq.parse::<u8>() else {
        return Err(Error::InvalidPreset);
    };
    let mut recall = RECALL;
    if pq <= 0x9 {
        recall[5] |= pq;
        Ok(recall.to_vec())
    } else {
        Err(Error::InvalidPreset)
    }
}

pub async fn read_preset(stream: Arc<Mutex<TcpStream>>) -> Result<i32, Error> {
    let cmd = READ_PRESET.to_vec();
    let mut rsp = [0u8;128];
    // rsp has this shape: [144, 80, 4, 255] 4 is the preset in HEX
    let wrote= {
        let mut stream_guard = stream.lock().await;
        stream_guard.write(&cmd).await
    };
    let Ok(_) = wrote else { warn!("Closed by remote peer in video."); return Err(Error::ClosedByRemotePeer)};
    
    let read = {
        let mut stream_guard = stream.lock().await;
        stream_guard.read(&mut rsp).await
    };

    let Ok(read) = read else { warn!("Closed by remote peer in video."); return Err(Error::ClosedByRemotePeer)};
    if read == 0 {
        warn!("Closed by remote peer in video."); return Err(Error::ClosedByRemotePeer)
    }
    let rsp = &rsp[..read];

    let rsp_vec = rsp.to_vec();
    if rsp.len() != 4 {
        return Err(Error::InvalidPreset);
    }
    Ok(i32::from_str_radix(&rsp_vec.get(2).unwrap().to_string(), 16).unwrap())

}

