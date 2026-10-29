#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("invalid pattern: {0}")]
    Pattern(#[from] PatternError),
    #[error("command #{index} ({desc}) is missing `{slot}`, which the pattern requires")]
    MissingSlot {
        index: usize,
        desc: String,
        slot: String,
    },
    #[error(
        "command #{index} ({desc}) sets `{key}`, but the pattern has no `{{{{ {key} }}}}` slot"
    )]
    UnknownSlot {
        index: usize,
        desc: String,
        key: String,
    },
    #[error("command #{index} has an invalid {slot} value {value:?}: {reason}")]
    InvalidValue {
        index: usize,
        slot: String,
        value: String,
        reason: &'static str,
    },
    #[error("commands #{first} and #{second} both resolve to `{path}`")]
    Duplicate {
        first: usize,
        second: usize,
        path: String,
    },
    #[error(
        "command #{index} ({desc}) leaves every slot empty, so it has no words to invoke it with"
    )]
    EmptyPath { index: usize, desc: String },
    #[error(
        "`{leaf_path}` is a complete command (#{leaf}) but also the start of `{branch_path}` (#{branch}); \
         a command can't also have subcommands"
    )]
    LeafAndBranch {
        leaf: usize,
        leaf_path: String,
        branch: usize,
        branch_path: String,
    },
    #[error(
        "`{path}` is ambiguous: `{word}` is the {first_meaning} in command #{first} \
         but the {second_meaning} in command #{second}"
    )]
    AmbiguousWord {
        path: String,
        word: String,
        first: usize,
        first_meaning: String,
        second: usize,
        second_meaning: String,
    },
    #[error("`{context}`: parameter `{name}` {reason}")]
    InvalidParam {
        context: String,
        name: String,
        reason: String,
    },
}
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PatternError {
    #[error("pattern is empty")]
    Empty,
    #[error("unclosed `{{{{` starting at byte {0}")]
    Unclosed(usize),
    #[error("empty slot `{{{{ }}}}` at byte {0}")]
    EmptySlot(usize),
    #[error(
        "invalid slot name {name:?} at byte {at}: use letters, digits, `_` or `-`, starting with a letter"
    )]
    InvalidSlotName { name: String, at: usize },
    #[error("slot `{0}` appears more than once")]
    DuplicateSlot(String),
    #[error("tokens must be separated by whitespace (byte {0})")]
    NotSeparated(usize),
    #[error(
        "literal {0:?} is not allowed: literals cannot start with `-`, contain braces, or end in `?` (only slots can be optional)"
    )]
    InvalidLiteral(String),
}
