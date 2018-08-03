macro_rules! newt_component {
    ($type:ty) => {
        impl NewtComponent for $type {
            fn co(&self) -> NewtComponentPtr {
                self.co
            }
        }

        impl<NC: NewtComponent> std::cmp::PartialEq<NC> for $type {
            fn eq(&self, other: &NC) -> bool {
                self.co == other.co()
            }
        }

        impl std::ops::Deref for $type {
            type Target = NewtComponentPtr;
            fn deref(&self) -> &Self::Target {
                &self.co
            }
        }
    };
}
