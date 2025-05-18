use tokio::{io::AsyncReadExt, process::ChildStdout, sync::broadcast::Sender};

pub async fn bufferer(buffer: &mut Vec<u8>, stdout: &mut ChildStdout, tx: Sender<Vec<u8>>) {
    let mut byte = [0u8; 1];
    loop { 
        buffer.clear();
        let mut found_start = false;

        while !found_start {
            match stdout.read_exact(&mut byte).await {
                Ok(_) => {
                    if byte[0] == 0xFF {
                        match stdout.read_exact(&mut byte).await {
                            Ok(_) => {
                                if byte[0] == 0xD8 {
                                    buffer.push(0xFF);
                                    buffer.push(0xD8);
                                    found_start = true;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                Err(_) => return, 
            }
        }

        if !found_start {
            continue; 
        }

        // Read until JPEG end (0xFF 0xD9)
        loop {
            match stdout.read_exact(&mut byte).await {
                Ok(_) => {
                    buffer.push(byte[0]);
                    let len = buffer.len();

                    if len >= 2 && buffer[len - 2] == 0xFF && buffer[len - 1] == 0xD9 {
                        break; // Found the end marker
                    }
                }
                Err(_) => return, // Exit if we can't read from stdout
            }
        }

        let _ = tx.send(buffer.clone());
    }
}
