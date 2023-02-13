use super::Member;
use crate::{prelude::*, seedwork::UniqueEntityId};

#[derive(Debug)]
#[readonly::make]
pub struct TeamMember {
    pub id: UniqueEntityId,
    pub member: Member,
    pub role: Role,
}

impl TeamMember {
    pub fn new(member: Member, role: Role) -> Result<Self> {
        let id = UniqueEntityId::new(None)?;

        Ok(Self { id, member, role })
    }

    pub fn new_with_id(member: Member, role: Role, id: Option<&str>) -> Result<Self> {
        let id = UniqueEntityId::new(id)?;

        Ok(Self { id, member, role })
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
    // use matches::assert_matches;

    #[test]
    fn test_should_create_team_member() -> Result<()> {
        let member = Member::new("John Doe").unwrap();

        let team_member = TeamMember::new(member.clone(), Role::Analyst)?;

        assert_eq!(team_member.member, member);
        assert_eq!(team_member.role, Role::Analyst);

        Ok(())
    }
}
