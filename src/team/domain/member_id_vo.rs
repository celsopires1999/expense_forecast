use crate::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberId {
    pub value: String,
}

impl MemberId {
    pub fn new(id: &str) -> Result<Self> {
        let id = Uuid::parse_str(id)?;
        Ok(Self {
            value: id.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::{Uuid, Version};

    #[test]
    fn test_should_not_create_new_id() {
        if let Err(e) = MemberId::new("fake") {
            assert_eq!(e.to_string(), "invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `k` at 3")
        } else {
            panic!("it has not thrown an error")
        }
    }

    #[test]
    fn test_should_create_with_id() -> Result<()> {
        let expected_uuid = "5b3b22ec-5fdf-4a68-9880-1ca3eed22b82";

        let id = MemberId::new(expected_uuid)?;
        let uuid = Uuid::try_parse(&id.value)?;

        assert_eq!(uuid.get_version(), Some(Version::Random),);
        assert_eq!(id.value, expected_uuid);

        Ok(())
    }
}
