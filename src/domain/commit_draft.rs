use crate::domain::commit_type::CommitType;

#[derive(Debug, Clone, Default)]
pub struct CommitDraft {
    pub commit_type: Option<CommitType>,
    pub scope: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub breaking_change: Option<String>,
    pub refs: Option<String>,
}
