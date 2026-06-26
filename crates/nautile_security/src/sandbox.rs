/// Sandbox flags for iframe/process restrictions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SandboxFlags(pub u32);
impl SandboxFlags {
    pub const SCRIPTS: Self = Self(1);
    pub const FORMS: Self = Self(1 << 1);
    pub const SAME_ORIGIN: Self = Self(1 << 2);
}
