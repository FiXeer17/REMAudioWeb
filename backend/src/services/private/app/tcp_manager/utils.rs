use actix::Addr;

use crate::services::private::app::{schemas::MatrixStates, ws_session::session::WsSession};

pub fn attach_availability(
    mut states: MatrixStates,
    availability: &Option<Addr<WsSession>>,
    session: &Addr<WsSession>,
) ->MatrixStates {
    if let Some(wsocket) = availability {
        if wsocket != session {
            states.available = Some(false)
        } else {
            states.available = Some(true)
        }
    } else {
        states.available = Some(true)
    }
    states
}
