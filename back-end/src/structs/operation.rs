use diesel::deserialize::{self, FromSql, Queryable};
use diesel::sql_types::{Integer, Nullable};
use diesel::sqlite::{Sqlite, SqliteValue};
use rocket::serde::ser::StdError;

pub struct NullableOperation(pub Option<Operation>);

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

impl FromSql<Nullable<Integer>, Sqlite> for NullableOperation {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let option: Option<i32> = Option::<i32>::from_sql(bytes)?;

        let operation: NullableOperation = match option {
            Some(int_value) => match int_value {
                1 => NullableOperation(Some(Operation::Create)),
                2 => NullableOperation(Some(Operation::Update)),
                3 => NullableOperation(Some(Operation::Delete)),
                _ => return Err("Unrecognized operation".into()),
            },
            None => NullableOperation(None),
        };

        Ok(operation)
    }
}


impl Queryable<Nullable<Integer>, Sqlite> for NullableOperation {
    type Row = Self;

    fn build(row: Self::Row) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        Ok(row)
    }
}
