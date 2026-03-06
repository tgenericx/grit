use crate::domain::CommitDraft;

pub enum State {
    SelectType,
    CollectScope(CommitDraft),
    CollectDescription(CommitDraft),
    AskBody(CommitDraft),
    CollectBody(CommitDraft),
    AskBreakingChange(CommitDraft),
    CollectBreakingChange(CommitDraft),
    CollectRefs(CommitDraft),
    Preview(CommitDraft),
    Done(CommitDraft),
    Abort,
}
