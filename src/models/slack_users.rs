use phf::phf_map;

#[derive(Debug, Clone)]
pub enum Users {
    Jesse(User),
}

#[derive(Debug, Clone)]
pub struct User {
    pub slack_name: &'static str,
    pub user_id: &'static str,
}

const JESSE: User = User {
    slack_name: "Jesse",
    user_id: "UBD4MB9RR"

};

static USERS: phf::Map<&'static str, Users>  = phf_map! {
    "UBD4MB9RR" => Users::Jesse(JESSE),
};

pub fn parse_user(user_id: &str) -> Option<Users> {
    USERS.get(user_id).cloned()
}
