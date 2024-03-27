use serenity::all::User;

// Types
type DateTimeUtc = chrono::DateTime<chrono::Utc>;

// Structs
#[derive(Clone, Debug)]
pub struct Player {
    pub user: User,
    pub end_dt: Option<DateTimeUtc>,
}

// Implementations
impl Player {
    pub fn new(user: User) -> Self {
        Self { user, end_dt: None }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user
    }
}
