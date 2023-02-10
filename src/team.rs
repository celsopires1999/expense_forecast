use uuid::Uuid;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct UniqueEntityId {
    value: String,
}

impl UniqueEntityId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
        }
    }

    pub fn with_id(id: &str) -> Result<Self> {
        let id = Uuid::parse_str(id)?;

        Ok(Self {
            value: id.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Team {
    id: UniqueEntityId,
    name: String,
    members: Vec<TeamMember>,
}

#[derive(Default)]
pub struct TeamBuilder {
    id: Option<String>,
    name: Option<String>,
    members: Vec<TeamMember>,
}

impl TeamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn member(&mut self, member: TeamMember) -> &mut Self {
        self.members.push(member);
        self
    }

    pub fn build(&self) -> Result<Team> {
        let id = match self.id.as_ref() {
            Some(id) => UniqueEntityId::with_id(id)?,
            None => UniqueEntityId::new(),
        };

        let Some(name) = self.name.as_ref() else {
            return Err(Error::EntityValidationError("No name"));
        };

        if self.members.len() == 0 {
            return Err(Error::EntityValidationError("No members"));
        };

        Ok(Team {
            id,
            name: name.to_string(),
            members: self.members.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamMember {
    member: Member,
    role: Role,
}

impl TeamMember {
    pub fn new(member: Member, role: Role) -> Self {
        Self { member, role }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    name: String,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    Manager,
    Leader,
    Analyst,
}

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;
    use uuid::Version;

    // region:      UniqueEntityId

    #[test]
    fn test_should_not_create_new_id() {
        if let Err(e) = UniqueEntityId::with_id("fake") {
            assert_eq!(e.to_string(), "invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `k` at 3")
        } else {
            panic!("it has not thrown an error")
        }
    }

    #[test]
    fn test_should_create_id() -> Result<()> {
        let id = UniqueEntityId::new();
        let uuid = Uuid::try_parse(&id.value)?;

        assert_eq!(uuid.get_version(), Some(Version::Random));

        Ok(())
    }

    #[test]
    fn test_should_create_with_id() -> Result<()> {
        let expected_uuid = "5b3b22ec-5fdf-4a68-9880-1ca3eed22b82";

        let id = UniqueEntityId::with_id(expected_uuid)?;
        let uuid = Uuid::try_parse(&id.value)?;

        assert_eq!(uuid.get_version(), Some(Version::Random),);
        assert_eq!(id.value, expected_uuid);

        Ok(())
    }

    // endregion:      UniqueEntityId

    // region:      Member

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

    // endregion:   Member

    // region:      TeamMember

    #[test]
    fn test_should_create_team_member() {
        let member = Member::new("John Doe").unwrap();

        let team_member = TeamMember::new(member.clone(), Role::Analyst);

        assert_eq!(team_member.member, member);
        assert_eq!(team_member.role, Role::Analyst);
    }

    // endregion:   TeamMember

    // region:      Team

    #[test]
    fn test_should_create_team() -> Result<()> {
        let team = TeamBuilder::new()
            .name("Technical Documentation")
            .member(TeamMember::new(
                Member::new("John Doe").unwrap(),
                Role::Leader,
            ))
            .build()?;

        assert_eq!(
            Uuid::try_parse(&team.id.value).unwrap().get_version(),
            Some(Version::Random)
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].member.name, "John Doe");
        assert_eq!(team.members[0].role, Role::Leader);

        Ok(())
    }

    #[test]
    fn test_should_create_team_with_id() -> Result<()> {
        let team = TeamBuilder::new()
            .id("5b3b22ec-5fdf-4a68-9880-1ca3eed22b82")
            .name("Technical Documentation")
            .member(TeamMember::new(
                Member::new("John Doe").unwrap(),
                Role::Leader,
            ))
            .build()?;

        assert_eq!(
            Uuid::try_parse(&team.id.value).unwrap().get_version(),
            Some(Version::Random)
        );
        assert_eq!(team.id.value, "5b3b22ec-5fdf-4a68-9880-1ca3eed22b82");
        assert_eq!(
            team.id,
            UniqueEntityId::with_id("5b3b22ec-5fdf-4a68-9880-1ca3eed22b82")?
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].member.name, "John Doe");
        assert_eq!(team.members[0].role, Role::Leader);

        Ok(())
    }

    // endregion:   Team
}
