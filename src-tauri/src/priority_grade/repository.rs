use super::PriorityGrade;
use crate::error::Result;
use rusqlite::{named_params, Connection};

pub fn list_priority_grades(connection: &Connection) -> Result<()> {
    let query = "SELECT * FROM priority_grades".to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute([])?;

    Ok(())
}

pub fn create_priority_grade(
    connection: &Connection,
    priority_grade: &PriorityGrade,
) -> Result<PriorityGrade> {
    let query = "INSERT INTO priority_grades VALUES (:grade)".to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! { ":grade": priority_grade.grade })?;

    let row_id = connection.last_insert_rowid();
    let mut created_priority_grade = priority_grade.clone();
    created_priority_grade.grade = row_id.to_string();

    Ok(created_priority_grade)
}
