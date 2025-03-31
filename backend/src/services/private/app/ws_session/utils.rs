use crate::engine::{defs::errors::Error, lib::MatrixCommand};

#[derive(Debug,Clone)]
pub enum HandleText{
    Command(Result<MatrixCommand, Error>),
    Recache,
}