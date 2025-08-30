use rig::message::{Text, UserContent};
use rig::{completion::Message as RigMessage, OneOrMany};
use rig::completion::Message::User;

pub(super) fn convert_to_messages(user_prompts: Vec<&str>) -> OneOrMany<RigMessage> {
    let rig_msgs: Vec<RigMessage> = user_prompts
        .iter()
        .map(|&p| User {
            content: OneOrMany::one(
                UserContent::Text(Text { text: p.to_string() })
            ) 
        })
        .collect();
    OneOrMany::many(rig_msgs).unwrap()
}