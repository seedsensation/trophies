use crate::types::MessageConstructor;
use crate::traits;
use crate::Error;

pub struct LocalContext {
}

impl LocalContext {
    pub fn new() -> Self { LocalContext {} }
}

impl traits::ContextTrait for LocalContext {
    fn get_user_id(&self) -> u64 {
        0u64
    }
    fn get_display_name(&self) -> String {
        "local_username".to_string()
    }

}


impl traits::SendMessage<String> for LocalContext {
    fn message_constructor(&self, constructor: MessageConstructor) -> String {
        format!("
title: {}
description: {}
ephemeral: {}
include_author: {}

{}",
            constructor.title,
            constructor.description,
            constructor.ephemeral,
            constructor.include_author,
            constructor.contents
        )
    }


    async fn send_constructed(&self, constructed: String) -> Result<(), Error> {
        println!("{}",constructed);
        Ok(())
    }
}
