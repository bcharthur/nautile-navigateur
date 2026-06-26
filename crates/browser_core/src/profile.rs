use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileId(u32);

#[derive(Debug)]
pub struct Profile {
    pub id: ProfileId,
    pub name: String,
    pub path: PathBuf,
}

impl Profile {
    pub fn default_profile(path: PathBuf) -> Self {
        Self { id: ProfileId(0), name: "Default".into(), path }
    }
}
