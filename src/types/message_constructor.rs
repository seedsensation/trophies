pub struct MessageConstructor {
    pub title: String,
    pub description: String,
    pub ephemeral: bool,
    pub include_author: bool,
    pub contents: String,
}

impl Default for MessageConstructor {
    fn default() -> MessageConstructor {
        MessageConstructor {
            title: "".to_string(),
            description: "".to_string(),
            ephemeral: false,
            include_author: false,
            contents: "".to_string(),

        }
    }
}

