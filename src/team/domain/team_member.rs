use crate::{prelude::*, seedwork::UniqueEntityId};

use super::MemberId;

#[derive(Debug)]
#[readonly::make]
pub struct TeamMember {
    pub id: UniqueEntityId,
    pub member_id: MemberId,
    pub role: Role,
}

impl TeamMember {
    pub fn new(member_id: MemberId, role: Role) -> Result<Self> {
        let id = UniqueEntityId::new(None)?;

        Ok(Self {
            id,
            member_id,
            role,
        })
    }

    pub fn new_with_id(member_id: MemberId, role: Role, id: Option<&str>) -> Result<Self> {
        let id = UniqueEntityId::new(id)?;

        Ok(Self {
            id,
            member_id,
            role,
        })
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
    use uuid::Uuid;
    // use matches::assert_matches;

    #[test]
    fn test_should_create_team_member() -> Result<()> {
        let member_id = MemberId::new(&Uuid::new_v4().to_string())?;

        let team_member = TeamMember::new(member_id.clone(), Role::Analyst)?;

        assert_eq!(team_member.member_id, member_id);
        assert_eq!(team_member.role, Role::Analyst);

        Ok(())
    }
}
