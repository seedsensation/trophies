use crate::types::MessageConstructor;
use crate::Error;

#[allow(dead_code)]
pub trait ContextTrait {
    fn get_user_id(&self) -> u64;
    fn get_display_name(&self) -> String;
}

pub trait SendMessage<M> where M: Sendable {
    async fn construct_message(&self, constructor: MessageConstructor) -> M;
    async fn send_constructed(&self, constructed: M) -> Result<(), Error>;

}

pub trait Sendable {}

impl Sendable for poise::CreateReply {}
impl Sendable for String {}
