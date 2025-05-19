use std::{net::SocketAddrV4, process::Stdio};

use actix::{Actor, Addr, AsyncContext, Context, SpawnHandle};
use log::{info, warn};
use tokio::{
    process::Command,
    sync::broadcast::{self, Sender},
};

use crate::services::private::stream::{
    messages::{EndStream, ReadStdout},
    streams_manager::streams_manager::StreamManager,
};

pub struct StreamHandler {
    pub rtsp_url: SocketAddrV4,
    pub stream_manager: Addr<StreamManager>,
    pub tx: Sender<Vec<u8>>,
    pub stdout: Option<tokio::process::ChildStdout>,
    pub bufferer: Option<SpawnHandle>,
}

impl StreamHandler {
    pub fn new(rtsp_url: SocketAddrV4, stream_manager: Addr<StreamManager>) -> Self {
        let channel: Sender<Vec<u8>> = broadcast::channel(16).0;
        Self {
            rtsp_url,
            stream_manager,
            tx: channel,
            stdout: None,
            bufferer: None,
        }
    }
}

impl Actor for StreamHandler {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let mut ffmpeg = Command::new("ffmpeg")
            .args([
                "-rtsp_transport", "tcp",
                "-i", &format!("rtsp://{}/av0/live", self.rtsp_url),
                "-f", "mjpeg",
                "-q:v","5",
                "-r","30",
                "pipe:1",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start ffmpeg");

        self.stdout = ffmpeg.stdout.take();
        ctx.address().do_send(ReadStdout {});

        let (srv_addr, socket) = (self.stream_manager.clone(), self.rtsp_url.clone());
        tokio::spawn(async move {
            match ffmpeg.wait().await{
                Ok(s) => info!("The ffmpeg process stopped succesfully: {}",s.success()),
                Err(_) => warn!("The ffmpeg process stopped unexpectedly")
            };
            srv_addr.do_send(EndStream { socket });
        });
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Stream handler rtsp://{}/av0/live stopped succesfully",self.rtsp_url.to_string());
    }
}
