use super::TeamMember;
use crate::prelude::*;
use crate::seedwork::UniqueEntityId;

#[derive(Debug)]
pub struct Team {
    pub id: UniqueEntityId,
    pub name: String,
    pub members: Vec<TeamMember>,
}

impl Team {
    pub fn new() -> TeamBuilder {
        TeamBuilder::default()
    }
}

#[derive(Default)]
pub struct TeamBuilder {
    id: Option<String>,
    name: Option<String>,
    members: Vec<TeamMember>,
}

impl TeamBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn member(mut self, member: TeamMember) -> Self {
        self.members.push(member);
        self
    }

    pub fn build(self) -> Result<Team> {
        let id = UniqueEntityId::new(self.id)?;

        let Some(name) = self.name else {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::domain::{Member, Role};

    use uuid::{Uuid, Version};

    #[test]
    fn test_should_create_team() -> Result<()> {
        let team = Team::new()
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
        let expected_id = "5b3b22ec-5fdf-4a68-9880-1ca3eed22b82".to_owned();
        let team = Team::new()
            .id(&expected_id)
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
        assert_eq!(team.id.value, expected_id);
        assert_eq!(team.id, UniqueEntityId::new(Some(expected_id))?);
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].member.name, "John Doe");
        assert_eq!(team.members[0].role, Role::Leader);

        Ok(())
    }
}
