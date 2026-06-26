/// Web origin tuple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Origin {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
}
