use rig::completion::Message::User;
use rig::message::UserContent;
use rig::{completion::Message, OneOrMany};

pub(super) fn split_prompt_and_history(user_prompts: Vec<&str>) -> (Option<Message>, Vec<Message>) {
    if let Some((&last_prompt, history)) = user_prompts.split_last() {
        let last_msg = User {
            content: OneOrMany::one(UserContent::text(last_prompt)),
        };
        let hist_msg: Vec<Message> = history
            .iter()
            .map(|&p| User {
                content: OneOrMany::one(UserContent::text(p)),
            })
            .collect();
        return (Some(last_msg), hist_msg);
    }
    (None, Vec::new())
}

#[cfg(test)]
mod test {
    use crate::llm::internal::split_prompt_and_history;
    use rig::completion::Message::User;
    use rig::message::UserContent;
    use rig::OneOrMany;

    #[test]
    fn split_last_prompt_and_histories() {
        let prompts = vec!["history 2", "history 1", "current"];
        let prompt_history = split_prompt_and_history(prompts);
        let last_prompt = prompt_history.0.unwrap();
        assert_eq!(
            last_prompt,
            User {
                content: OneOrMany::one(UserContent::text("current"))
            }
        )
    }
}
