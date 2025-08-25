use crate::traits::{ContextTrait, SendMessage, Sendable};
use crate::{Context, Error, serenity};
use poise::CreateReply;


impl ContextTrait for Context<'_> {
    fn get_user_id(&self) -> u64 {
        self.author().id.get()
    }
    fn get_display_name(&self) -> String {
        self.author().display_name().to_string()
    }
}


impl SendMessage<CreateReply> for Context<'_> {
    async fn construct_message(&self, constructor: crate::types::MessageConstructor) -> CreateReply {
        CreateReply::default()
            .embed(
                // if the embed includes the author
                if constructor.include_author {
                    // create a new embed
                    serenity::CreateEmbed::new()
                        .title(constructor.title)
                        .description(constructor.description)
                        // create a new Author field
                        .author(serenity::CreateEmbedAuthor::new(self.get_display_name())
                            .icon_url(self.author().static_avatar_url().unwrap_or_else(|| self.author().default_avatar_url())
                        ))

                // if the embed does not include the author
                } else {
                    // the same embed but without the author
                    serenity::CreateEmbed::new()
                        .title(constructor.title)
                        .description(constructor.description)
                })
            .content(constructor.contents)
            .ephemeral(constructor.ephemeral);

    }

    async fn send_constructed(&self, constructed: CreateReply) -> Result<(), Error> {
        self.send(constructed);
        Ok(())

    }
        // title, description, ephemeral, include_author, contents
}
