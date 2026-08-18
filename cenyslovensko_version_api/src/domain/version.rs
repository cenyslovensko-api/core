use derive_more::{Constructor, Display};

#[derive(Display, Debug, Clone, PartialEq, Eq, Constructor)]
pub struct Version {
    version: String,
}
