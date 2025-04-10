use std::net::SocketAddrV4;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::utils::configs::ping_socket_settings;

pub async fn try_connection(socket: SocketAddrV4) -> bool {
    let max_retries = ping_socket_settings::get_ping_socket_max_retries();
    let timeout_interval = ping_socket_settings::get_ping_socket_timeout();
    let mut i=0;
    while i < max_retries {
        match timeout(timeout_interval,TcpStream::connect(socket)).await {
            Ok(not_timedout) => {
                if let Ok(tcp_stream) = not_timedout{
                    drop(tcp_stream);
                    return true;
                }else{
                    return false;
                }
            }
            Err(_) => {
                i+=1;
                
            }
        }
    }
    return false;
}

pub fn check_in_connections(socket:SocketAddrV4,connections:Option<Vec<SocketAddrV4>>)-> bool{
    match connections{
        Some(connections)=>{
            connections.contains(&socket)
        },
        None => false
    }
}
