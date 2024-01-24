mod v1beta1 {
    use crate::types::v1beta1::{evidence, Evidence};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de> Deserialize<'de> for Evidence {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let sum = Option::<evidence::Sum>::deserialize(deserializer)?;
            Ok(Self { sum })
        }
    }

    impl Serialize for Evidence {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.sum.serialize(serializer)
        }
    }
}

mod v1 {
    use crate::types::v1::{evidence, Evidence};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de> Deserialize<'de> for Evidence {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let sum = Option::<evidence::Sum>::deserialize(deserializer)?;
            Ok(Self { sum })
        }
    }

    impl Serialize for Evidence {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.sum.serialize(serializer)
        }
    }
}
