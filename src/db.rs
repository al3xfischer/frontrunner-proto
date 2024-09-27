use crate::timer::LapTime;
use odbc::odbc_safe::{AutocommitMode, AutocommitOn};
use odbc::{Connection, ResultSetState, Statement};
use std::fmt::{Display, Formatter};

type DbResult<T> = Result<T, DbError>;

#[derive(Debug)]
pub enum DbError {
    Internal(String),
    NoId,
    Message(String),
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Internal(msg) => write!(f, "Inner: {}", msg),
            DbError::NoId => write!(f, "No Id found"),
            DbError::Message(msg) => write!(f, "Message: {}", msg),
        }
    }
}

pub fn create_laptime(conn: &Connection<AutocommitOn>, lap: &LapTime) -> DbResult<()> {
    let stmt = Statement::with_parent(conn).map_err(|e| DbError::Internal(e.to_string()))?;
    let sql = format!(
        "insert into tbZeit (LfdNr,Kennung,Zeit) values ('{}','{}','{}')",
        lap.seq_number, lap.channel, lap.time
    );

    match stmt.exec_direct(&sql) {
        Ok(ResultSetState::NoData(_)) => Ok(()),
        _ => Err(DbError::Message("Unable to insert lap".into())),
    }
}

pub fn update_lap<T>(conn: &Connection<T>, id: usize, lap: &LapTime) -> DbResult<()>
where
    T: AutocommitMode,
{
    let stmt = Statement::with_parent(conn).map_err(|e| DbError::Internal(e.to_string()))?;
    let sql = format!(
        "update tbZeit set Zeit = '{}', LfdNr = {}, Kennung = '{}' where ID = {}",
        lap.time, lap.seq_number, lap.channel, id
    );

    match stmt.exec_direct(&sql) {
        Ok(ResultSetState::NoData(_)) => Ok(()),
        Ok(_) => unreachable!(),
        Err(e) => Err(DbError::Internal(e.to_string())),
    }
}

pub fn fetch_id(conn: &Connection<AutocommitOn>) -> Result<usize, DbError> {
    let stmt = Statement::with_parent(conn).map_err(|e| DbError::Internal(e.to_string()))?;
    let sql: String =
        "select top 1 ID from tbZeit where Nr is not null and Zeit is null order by ID asc".into();
    const ID_INDEX: u16 = 1;

    match stmt.exec_direct(&sql) {
        Ok(ResultSetState::Data(mut stmt)) => {
            let mut cursor = match stmt.fetch() {
                Ok(Some(c)) => c,
                Ok(None) => return Err(DbError::NoId),
                Err(e) => return Err(DbError::Internal(e.to_string())),
            };

            match cursor.get_data::<&str>(ID_INDEX) {
                Ok(Some(val)) => Ok(val.parse::<usize>().expect("Invalid ID")),
                _ => Err(DbError::Message("No data found".into())),
            }
        }
        _ => Err(DbError::Message("Unable to fetch Id".into())),
    }
}
