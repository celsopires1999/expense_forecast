use crate::prelude::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
}

impl Member {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.len() == 0 {
            return Err(Error::EntityValidationError(
                "member without name is not allowed",
            ));
        }
        Ok(Self { name })
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
                "member without name is not allowed"
            ))
        );
    }

    #[test]
    fn test_should_create_member() {
        let member = Member::new("John Doe").unwrap();
        assert_eq!(member.name, "John Doe");
    }

    #[test]
    fn test_getter_name() -> Result<()> {
        let member = Member::new("John Doe")?;
        assert_eq!(member.name, "John Doe");

        Ok(())
    }
}
