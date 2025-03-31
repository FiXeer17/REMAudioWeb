use std::time::Duration;


pub const COMMAND_DELAY: Duration = Duration::from_millis(20);
pub const RECONNECT_DELAY: Duration = Duration::from_secs(1);
pub const READ_TIMEOUT: Duration = Duration::from_secs(10);
pub const TIMEDOUT_TIME: Duration = Duration::from_secs(10);
pub const MAX_RETRIES: u8 = 3;