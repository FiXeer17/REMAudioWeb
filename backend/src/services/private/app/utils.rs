

use crate::services::private::socket::utils::Device;

use super::messages::Commands;



pub trait CommandsExt {
    fn compatibility(&self)->Device;
}

impl CommandsExt for Commands{
    fn compatibility(&self)->Device {
        match self {
            Commands::ReCache => Device::Audio,
            Commands::SetLabel(_) => Device::Audio,
            Commands::SetMatrixCommand(_)=> Device::Audio,
            Commands::SetVisibility(_)=> Device::Audio,
        }
    }
}

