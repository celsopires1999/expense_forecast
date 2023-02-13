use expense_forecast::prelude::*;
use expense_forecast::{seedwork::*, team::*};
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
        team_members: vec![team_member_id],
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
