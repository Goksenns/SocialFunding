pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project(){
        let mut state = State::new();
        let user = Address::from([1; 20]);
        let project_name = "Test Project".to_string();
        let project_details = "Test project details".to_string();
        let project_fund = 100;
        let project_deadline = 100;
        let project_id = 1;
        state.create_project(user, project_name, project_details, project_fund, project_deadline);
        assert_eq!(state.projects[&project_id].name, project_name);
        assert_eq!(state.projects[&project_id].details, project_details);
        assert_eq!(state.projects[&project_id].fund, project_fund);
        assert_eq!(state.projects[&project_id].deadline, project_deadline);
        assert_eq!(state.projects[&project_id].owner, user);
    }
    
    #[test]
    fn test_fund_project(){
        let mut state = State::new();
        let user = Address::from([1; 20]);
        let project_name = "Test Project".to_string();
        let project_details = "Test project details".to_string();
        let project_fund = 100;
        let project_deadline = 100;
        let project_id = 1;
        state.create_project(user, project_name, project_details, project_fund, project_deadline);
        let funding_user = Address::from([2; 20]);
        let funding_amount = 50;
        state.fund_project(project_id, funding_user, funding_amount);
    
}
