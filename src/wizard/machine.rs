use crate::{
    domain::{CommitDraft, CommitType},
    wizard::state::State,
};

pub enum Event {
    TypeSelected(CommitType),
    Input(String),
    Skipped,
    Yes,
    No,
    Confirm,
    Abort,
}

pub fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::SelectType, Event::TypeSelected(t)) => State::CollectScope(CommitDraft {
            commit_type: Some(t),
            ..Default::default()
        }),

        (State::CollectScope(mut draft), Event::Input(scope)) => {
            draft.scope = Some(scope);
            State::CollectDescription(draft)
        }
        (State::CollectScope(draft), Event::Skipped) => State::CollectDescription(draft),

        (State::CollectDescription(mut draft), Event::Input(desc)) => {
            draft.description = Some(desc);
            State::AskBody(draft)
        }

        (State::AskBody(draft), Event::Yes) => State::CollectBody(draft),
        (State::AskBody(draft), Event::No) => State::AskBreakingChange(draft),

        (State::CollectBody(mut draft), Event::Input(body)) => {
            draft.body = Some(body);
            State::AskBreakingChange(draft)
        }

        (State::AskBreakingChange(draft), Event::Yes) => State::CollectBreakingChange(draft),
        (State::AskBreakingChange(draft), Event::No) => State::CollectRefs(draft),

        (State::CollectBreakingChange(mut draft), Event::Input(bc)) => {
            draft.breaking_change = Some(bc);
            State::CollectRefs(draft)
        }

        (State::CollectRefs(mut draft), Event::Input(refs)) => {
            draft.refs = Some(refs);
            State::Preview(draft)
        }
        (State::CollectRefs(draft), Event::Skipped) => State::Preview(draft),

        (State::Preview(draft), Event::Confirm) => State::Done(draft),
        (State::Preview(_), Event::Abort) => State::Abort,
        (_, Event::Abort) => State::Abort,

        (state, _) => state,
    }
}
