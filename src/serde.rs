use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Brand;

impl<B, Inner> Serialize for Brand<B, Inner>
where
    Inner: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, B, Inner> Deserialize<'de> for Brand<B, Inner>
where
    Inner: for<'a> Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Inner::deserialize(deserializer).map(Self::unchecked_from_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::brand!(
        type TestId = i32;
    );

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Test {
        id: TestId,
        other: String,
    }

    #[test]
    fn test_serialize_deserialize() {
        let t = Test {
            id: TestId::unchecked_from_inner(123),
            other: "olá".into(),
        };

        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, r#"{"id":123,"other":"olá"}"#);

        let recovered: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, t);
    }
}
