use rusqlite::{
    types::{FromSql, ToSqlOutput, Value, ValueRef},
    ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Set,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Set
    }
}

impl ToSql for Operation {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let val = match self {
            Operation::Add => Value::Text("Add".into()),
            Operation::Sub => Value::Text("Sub".into()),
            Operation::Mul => Value::Text("Mul".into()),
            Operation::Div => Value::Text("Div".into()),
            Operation::Set => Value::Text("Set".into()),
        };

        Ok(ToSqlOutput::Owned(val))
    }
}

impl FromSql for Operation {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(text) => match text {
                b"Add" => Ok(Operation::Add),
                b"Sub" => Ok(Operation::Sub),
                b"Mul" => Ok(Operation::Mul),
                b"Div" => Ok(Operation::Div),
                b"Set" => Ok(Operation::Set),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            },
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Grade {
    A,
    B,
    C,
    D,
    E,
    #[serde(rename = "*")]
    Star,
}

impl Default for Grade {
    fn default() -> Self {
        Self::Star
    }
}

impl ToSql for Grade {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let val = match self {
            Grade::A => Value::Text("A".into()),
            Grade::B => Value::Text("B".into()),
            Grade::C => Value::Text("C".into()),
            Grade::D => Value::Text("D".into()),
            Grade::E => Value::Text("E".into()),
            Grade::Star => Value::Text("*".into()),
        };

        Ok(ToSqlOutput::Owned(val))
    }
}

impl FromSql for Grade {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(text) => match text {
                b"A" => Ok(Grade::A),
                b"B" => Ok(Grade::B),
                b"C" => Ok(Grade::C),
                b"D" => Ok(Grade::D),
                b"E" => Ok(Grade::E),
                b"*" => Ok(Grade::Star),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            },
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
