pub(crate)

use std::fmt::Display;

use rusqlite::{Connection, Error};



#[derive(Debug)]
pub(crate) struct Row {
    id:i32,
    project_name: String,
    path: String,
}

impl Row {
    /// Returns a reference to the project name of this [`Row`].
    pub fn project_name(&self) -> &str {
        self.project_name.as_ref()
    }

    /// Returns a reference to the path of this [`Row`].
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}, {}", self.id, self.project_name, self.path))
    }
}

pub(crate) fn list_projects(conn: &Connection, option: &Option<String>) -> Result<Vec<Row>, String> {
    let query = format!(
        "select * from  projects {}",
        if let Some(search) = option {
            format!("where project_name like '%{}%';", search)
        } else {
            ";".to_owned()
        }
    );
    match conn.prepare(&query) {
        Ok(mut finals) => {
            let rows = finals
                .query_map([], |row| {
                    Ok(Row {
                        id:row.get_unwrap::<&str,i32>("id"),
                        project_name: row.get_unwrap::<&str, String>("project_name"),
                        path: row.get_unwrap::<&str, String>("path"),
                    })
                })
                .unwrap()
                .collect::<Vec<Result<Row, Error>>>();
            Ok(rows.into_iter().map(|f| f.unwrap()).collect::<Vec<Row>>())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn add_to_table(conn: &Connection, name: &str, path: &str) -> Result<(), String> {
    if let Err(err) = conn.execute(
        "insert into projects(project_name,path) values(?1,?2)",
        [name, path],
    ) {
        Err(err.to_string())
    } else {
        Ok(())
    }
}

pub fn create_table(conn: &Connection) -> Result<(), String> {
    if let Err(err) = conn.execute(
        "create table if not exists projects(
        id integer primary key autoincrement,
        project_name text not null unique,
        path text not null
    )",
        [],
    ) {
        Err(err.to_string())
    } else {
        Ok(())
    }
}

pub(crate) fn delete_from_table(conn: &Connection, index: i32) -> Result<(), String> {
    match conn.execute("delete from projects where id=?1", [index]) {
        Ok(_) => return Ok(()) ,
        Err(err) => return Err(err.to_string()),
    }
}
