use std::num::NonZeroU32;

macro_rules! typed_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(NonZeroU32);

        impl $name {
            pub fn new(v: u32) -> Option<Self> {
                NonZeroU32::new(v).map(Self)
            }

            pub fn get(self) -> u32 {
                self.0.get()
            }
        }
    };
}

typed_id!(NodeId);
typed_id!(DocumentId);
typed_id!(FrameId);
typed_id!(TabId);
typed_id!(ProcessId);
typed_id!(LayoutBoxId);
typed_id!(StyleNodeId);
typed_id!(LayerId);
typed_id!(ObjectId);
typed_id!(FunctionId);
typed_id!(RealmId);
typed_id!(EnvironmentId);
