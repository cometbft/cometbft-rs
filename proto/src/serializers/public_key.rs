mod v1 {
    use crate::crypto::v1::{public_key, PublicKey};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de> Deserialize<'de> for PublicKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let sum = Option::<public_key::Sum>::deserialize(deserializer)?;
            Ok(Self { sum })
        }
    }

    impl Serialize for PublicKey {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.sum.serialize(serializer)
        }
    }
}
