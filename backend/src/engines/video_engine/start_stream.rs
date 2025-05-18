use tokio::process::Command;

pub fn start_stream(rstpurl: String, path: String) -> Result<tokio::process::Child,std::io::Error>{
    let args = &[
        "-rtsp_transport","tcp",
        "-i", &rstpurl,
        "-c:v", "copy",
        "-hls_time", "1",
        "-hls_list_size", "2",
        "-hls_flags", "delete_segments+omit_endlist+discont_start",
        "-start_number", "1",
        &path        
    ];

    Command::new("ffmpeg").args(args).spawn()

}
