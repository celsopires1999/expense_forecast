use crate::{prelude::*, seedwork::UniqueEntityId};
#[derive(Debug, Clone, PartialEq)]
#[readonly::make]
pub struct Member {
    pub id: UniqueEntityId,
    pub name: String,
}

impl Member {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let id = UniqueEntityId::new(None)?;

        Ok(Member::build(name, id)?)
    }

    pub fn new_with_id(name: impl Into<String>, id: Option<&str>) -> Result<Self> {
        let id = UniqueEntityId::new(id)?;
        Ok(Member::build(name, id)?)
    }

    fn build(name: impl Into<String>, id: UniqueEntityId) -> Result<Self> {
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
            Member::new(""),
            Err(Error::EntityValidationError(
                "name must be more than 3 characters"
            ))
        );
    }

    #[test]
    fn test_should_create_member() {
        let member = Member::new("John Doe").unwrap();
        assert_eq!(member.name, "John Doe");
    }
}
