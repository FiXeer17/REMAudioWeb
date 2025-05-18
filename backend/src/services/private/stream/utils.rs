use actix_web::{web, HttpResponse,};
use futures_util::stream::{self};
use tokio::sync::broadcast;


pub fn handle_stream(tx:broadcast::Sender<Vec<u8>>)-> HttpResponse{
    let rx =tx.subscribe();
    let stream = stream::unfold(rx, |mut rx| async {
        match rx.recv().await {
            Ok(frame) => {
                let headers = format!(
                    "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                    frame.len()
                ).into_bytes();

                let full_chunk = [headers, frame, b"\r\n".to_vec()].concat();

                Some((Ok::<_, actix_web::Error>(web::Bytes::from(full_chunk)), rx))
            }
            Err(_) => None, // stop streaming if the sender is closed or lagged
        }
    });
 // MUST FORCE THE CLIENT TO NOT SAVE FRAMES (disable cache)
    HttpResponse::Ok()
    .content_type("multipart/x-mixed-replace; boundary=frame")
        .insert_header(("Content-Type", "multipart/x-mixed-replace; boundary=frame"))
        .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Expires", "0"))
    .streaming(Box::pin(stream))

}