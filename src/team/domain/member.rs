use crate::{prelude::*, seedwork::UniqueEntityId};
#[derive(Debug, Clone, PartialEq)]
#[readonly::make]
pub struct Member {
    pub id: UniqueEntityId,
    pub name: String,
}

impl Member {
    pub fn new(name: impl Into<String>, id: Option<String>) -> Result<Self> {
        let id = UniqueEntityId::new(id)?;
        let name = Member::validate_name(name)?;

        Ok(Self { id, name })
    }

    fn validate_name(name: impl Into<String>) -> Result<String> {
        let name = name.into();
        if name.len() <= 3 {
            return Err(Error::EntityValidationError(
                "name must be more than 3 characters",
            ));
        };

        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;

    #[test]
    fn test_should_not_create_member() {
        assert_matches!(
            Member::new("", None),
            Err(Error::EntityValidationError(
                "name must be more than 3 characters"
            ))
        );
    }

    #[test]
    fn test_should_create_member() {
        let member = Member::new("John Doe", None).unwrap();
        assert_eq!(member.name, "John Doe");
    }
}
