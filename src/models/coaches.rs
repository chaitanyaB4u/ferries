use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Coach {
    pub id: String,
    pub user_id: String,
    pub full_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Fields that we can safely expose to APIs
#[juniper::object(description = "The exposed attributes of the Coach Structure.")]
impl Coach {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn user_id(&self) -> &str {
        self.user_id.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn name(&self) -> &str {
        self.full_name.as_str()
    }
}
