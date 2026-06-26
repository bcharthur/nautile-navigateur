/// Site key used for future process isolation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Site {
    pub registrable_domain: String,
}
