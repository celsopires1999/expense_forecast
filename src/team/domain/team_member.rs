use super::Member;

#[derive(Debug, Clone, PartialEq)]
pub struct TeamMember {
    pub member: Member,
    pub role: Role,
}

impl TeamMember {
    pub fn new(member: Member, role: Role) -> Self {
        Self { member, role }
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
    fn test_should_create_team_member() {
        let member = Member::new("John Doe").unwrap();

        let team_member = TeamMember::new(member.clone(), Role::Analyst);

        assert_eq!(team_member.member, member);
        assert_eq!(team_member.role, Role::Analyst);
    }
}
