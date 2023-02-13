use expense_forecast::prelude::*;
use expense_forecast::{seedwork::*, team::*};
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
