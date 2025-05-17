use std::{net::SocketAddrV4, process::Stdio};

use actix::{Actor, Addr, AsyncContext, Context, SpawnHandle};
use tokio::{
    process::Command,
    sync::broadcast::{self, Sender},
};

use crate::services::private::stream::{
    messages::AttachStream, streams_manager::streams_manager::StreamManager,
};

pub struct StreamHandler {
    pub rtsp_url: SocketAddrV4,
    pub stream_manager: Addr<StreamManager>,
    pub tx: Sender<Vec<u8>>,
    pub ffmpeg_process: Option<tokio::process::Child>,
    pub bufferer:Option<SpawnHandle>
}

impl StreamHandler {
    pub fn new(rtsp_url: SocketAddrV4, stream_manager: Addr<StreamManager>) -> Self {
        let channel: Sender<Vec<u8>> = broadcast::channel(16).0;
        Self {
            rtsp_url,
            stream_manager,
            tx: channel,
            ffmpeg_process: None,
            bufferer: None
        }
    }
}

impl Actor for StreamHandler {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {

        let ffmpeg = Command::new("ffmpeg")
                .args([
                    "-rtsp_transport", "tcp", "-i",
                    &format!("rtsp://{}/av0/live", self.rtsp_url),
                    "-f", "mjpeg",
                    "-q:v","5",
                    "-r","30",
                    "pipe:1",
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start ffmpeg");

         
            ctx.address().do_send(AttachStream { stream: ffmpeg });
    }
}
