use crate::prelude::*;
use crate::seedwork::UniqueEntityId;

use super::TeamMemberId;

#[derive(Default)]
pub struct TeamProps {
    pub name: String,
    pub team_members: Vec<TeamMemberId>,
}

#[derive(Debug)]
#[readonly::make]
pub struct Team {
    pub id: UniqueEntityId,
    pub name: String,
    pub team_members: Vec<TeamMemberId>,
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
        let team_members = Team::validate_members(props.team_members)?;

        Ok(Team {
            id,
            name,
            team_members,
        })
    }

    pub fn change(&mut self, props: TeamProps) -> Result<()> {
        let name = Team::validate_name(props.name)?;
        let members = Team::validate_members(props.team_members)?;

        self.name = name;
        self.team_members = members;

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

    fn validate_members(members: Vec<TeamMemberId>) -> Result<Vec<TeamMemberId>> {
        if members.len() == 0 {
            return Err(Error::EntityValidationError("No members"));
        };

        Ok(members)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use matches::assert_matches;
    use uuid::{Uuid, Version};

    #[test]
    fn test_should_create_team() -> Result<()> {
        let team_member_id = TeamMemberId::new(&Uuid::new_v4().to_string())?;

        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            team_members: vec![team_member_id.clone()],
        };

        let team = Team::new(props)?;

        assert_eq!(
            Uuid::try_parse(&team.id.value).unwrap().get_version(),
            Some(Version::Random)
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.team_members.len(), 1);
        assert_eq!(team.team_members[0], team_member_id);

        Ok(())
    }

    #[test]
    fn test_should_create_team_with_id() -> Result<()> {
        let team_member_id = TeamMemberId::new(&Uuid::new_v4().to_string())?;
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            team_members: vec![team_member_id.clone()],
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
        assert_eq!(team.team_members.len(), 1);
        assert_eq!(team.team_members[0], team_member_id);

        Ok(())
    }

    #[test]
    fn test_should_change_team() -> Result<()> {
        let team_member_id = TeamMemberId::new(&Uuid::new_v4().to_string())?;
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            team_members: vec![team_member_id.clone()],
        };

        let mut team = Team::new(props)?;

        let expected_team_member_id = TeamMemberId::new(&Uuid::new_v4().to_string())?;
        let props = TeamProps {
            name: "Engineering".to_owned(),
            team_members: vec![expected_team_member_id.clone()],
        };

        team.change(props)?;

        assert_eq!(
            Uuid::try_parse(&team.id.value).unwrap().get_version(),
            Some(Version::Random)
        );
        assert_eq!(team.name, "Engineering");
        assert_eq!(team.team_members.len(), 1);
        assert_eq!(team.team_members[0], expected_team_member_id);

        Ok(())
    }

    #[test]
    fn test_should_not_change_team() -> Result<()> {
        let team_member_id = TeamMemberId::new(&Uuid::new_v4().to_string())?;
        let props = TeamProps {
            name: "Technical Documentation".to_owned(),
            team_members: vec![team_member_id.clone()],
        };

        let mut team = Team::new(props)?;

        let new_team_member_id = TeamMemberId::new(&Uuid::new_v4().to_string())?;
        let props = TeamProps {
            name: "Eng".to_owned(),
            team_members: vec![new_team_member_id.clone()],
        };

        assert_matches!(
            team.change(props),
            Err(Error::EntityValidationError(
                "name must be more than 3 characters"
            ))
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.team_members.len(), 1);
        assert_eq!(team.team_members[0], team_member_id);

        assert_matches!(
            team.change(TeamProps {
                name: "Engineering".to_owned(),
                team_members: vec![],
            }),
            Err(Error::EntityValidationError("No members"))
        );
        assert_eq!(team.name, "Technical Documentation");
        assert_eq!(team.team_members.len(), 1);
        assert_eq!(team.team_members[0], team_member_id);

        Ok(())
    }
}
