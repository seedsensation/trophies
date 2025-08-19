#[macro_export]
macro_rules! custom_send {
    (
        context: $ctx:expr,
        $(title: $title:expr,)?
        $(description: $description:expr,)?
        $(ephemeral: $ephemeral:expr,)?
        $(include_author: $include_author:expr,)?
        contents: $contents:expr
    ) => {
        $ctx.send_constructed(construct_message(crate::types::MessageConstructor {
            $(title: $title,)?
            $(description: $description,)?
            $(ephemeral: $ephemeral,)?
            $(include_author: $include_author,)?
            contents: $contents,
            ..Default::default()
        })).await
    }
}
