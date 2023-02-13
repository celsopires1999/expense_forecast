use super::TeamMember;
use crate::prelude::*;
use crate::seedwork::UniqueEntityId;

#[derive(Debug)]
#[readonly::make]
pub struct Team {
    pub id: UniqueEntityId,
    pub name: String,
    pub members: Vec<TeamMember>,
}

impl Team {
    pub fn new(props: TeamProps) -> Result<Team> {
        let id = UniqueEntityId::new(None)?;

        Ok(Team::build(props, id)?)
    }

    pub fn new_with_id(props: TeamProps, id: &str) -> Result<Team> {
        let id = UniqueEntityId::new(Some(id.into()))?;

        Ok(Team::build(props, id)?)
    }

    fn build(props: TeamProps, id: UniqueEntityId) -> Result<Team> {
        let name = Team::validate_name(props.name)?;
        let members = Team::validate_members(props.members)?;

        Ok(Team { id, name, members })
    }

    pub fn change(&mut self, props: TeamProps) -> Result<()> {
        let name = Team::validate_name(props.name)?;
        let members = Team::validate_members(props.members)?;

        self.name = name;
        self.members = members;

        Ok(())
    }

    fn validate_name(name: String) -> Result<String> {
        if name.len() <= 3 {
            return Err(Error::EntityValidationError(
                "name must be more than 3 characters",
            ));
        };

        Ok(name)
    }

    fn validate_members(members: Vec<TeamMember>) -> Result<Vec<TeamMember>> {
        if members.len() == 0 {
            return Err(Error::EntityValidationError("No members"));
        };

        Ok(members)
    }
}

#[derive(Default)]
pub struct TeamProps {
    pub name: String,
    pub members: Vec<TeamMember>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::team::domain::{Member, Role};
    use matches::assert_matches;
    use uuid::{Uuid, Version};

    #[test]
    fn test_should_create_team() -> Result<()> {
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            members: vec![TeamMember::new(Member::new("John Doe")?, Role::Leader)?],
        };

        let team = Team::new(props)?;

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
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            members: vec![TeamMember::new(Member::new("John Doe")?, Role::Leader)?],
        };

        let expected_id = "5b3b22ec-5fdf-4a68-9880-1ca3eed22b82";
        let team = Team::new_with_id(props, expected_id)?;

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

    #[test]
    fn test_should_change_team() -> Result<()> {
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            members: vec![TeamMember::new(Member::new("John Doe")?, Role::Leader)?],
        };

        let mut team = Team::new(props)?;

        let props = TeamProps {
            name: "Engineering".to_owned(),
            members: vec![TeamMember::new(Member::new("Marie Doe")?, Role::Manager)?],
        };

        team.change(props)?;

        assert_eq!(
            Uuid::try_parse(&team.id.value).unwrap().get_version(),
            Some(Version::Random)
        );
        assert_eq!(team.name, "Engineering");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].member.name, "Marie Doe");
        assert_eq!(team.members[0].role, Role::Manager);

        Ok(())
    }

    #[test]
    fn test_should_not_change_team() -> Result<()> {
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            members: vec![TeamMember::new(Member::new("John Doe")?, Role::Leader)?],
        };

        let mut team = Team::new(props)?;

        let props = TeamProps {
            name: "Eng".to_owned(),
            members: vec![TeamMember::new(Member::new("Marie Doe")?, Role::Manager)?],
        };

        assert_matches!(
            team.change(props),
            Err(Error::EntityValidationError(
                "name must be more than 3 characters"
            ))
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].member.name, "John Doe");
        assert_eq!(team.members[0].role, Role::Leader);

        assert_matches!(
            team.change(TeamProps {
                name: "Engineering".to_owned(),
                members: vec![],
            }),
            Err(Error::EntityValidationError("No members"))
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].member.name, "John Doe");
        assert_eq!(team.members[0].role, Role::Leader);

        Ok(())
    }
}
