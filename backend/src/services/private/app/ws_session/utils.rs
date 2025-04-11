use crate::{engine::{defs::{datas, errors::Error}, lib::MatrixCommand}, services::private::app::schemas::{SetState, SetVisibility}};

#[derive(Debug,Clone)]
pub enum HandleText{
    Command(Result<MatrixCommand, Error>),
    Visibility(SetVisibility),
    Recache,
    Error(String)
}

pub fn deserialize_text(text:String)-> HandleText{
    if text == String::from("recache"){
        return HandleText::Recache;
    }

    if let Ok(set_state) = serde_json::from_str::<SetState>(&text){
        let rw = datas::rw::WRITE.to_string();
        return HandleText::Command(MatrixCommand::new_from_client(rw, set_state));
    }
    if let Ok(set_visibility) = serde_json::from_str::<SetVisibility>(&text){
        if let Err(e) = set_visibility.channel.parse::<u32>(){
            return HandleText::Error(e.to_string())
        }
        if let Err(e) = set_visibility.value.parse::<bool>(){
            return HandleText::Error(e.to_string())
        }
        return HandleText::Visibility(set_visibility);
    }
    return HandleText::Error("invalid command".to_string());
}