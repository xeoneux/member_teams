struct Member {
    id: i32,
    name: String,
}

#[juniper::object(description = "A member of team")]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
