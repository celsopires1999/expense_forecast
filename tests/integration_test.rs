use expense_forecast::prelude::*;
use expense_forecast::seedwork::*;
use expense_forecast::team::*;
use uuid::Uuid;
use uuid::Version;

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
    let team = Team::new()
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
        UniqueEntityId::new(Some("5b3b22ec-5fdf-4a68-9880-1ca3eed22b82".to_owned()))?
    );
    assert_eq!(team.name, "Technical Documentation");
    assert_eq!(team.members.len(), 1);
    assert_eq!(team.members[0].member.name, "John Doe");
    assert_eq!(team.members[0].role, Role::Leader);

    Ok(())
}
