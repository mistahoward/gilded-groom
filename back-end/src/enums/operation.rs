#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Create,
    Update,
    Delete,
}

impl Operation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operation::Create => "CREATE",
            Operation::Update => "UPDATE",
            Operation::Delete => "DELETE",
        }
    }
}