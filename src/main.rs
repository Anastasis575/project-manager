use std::{
    env::args,
    fs,
    path::{self, PathBuf},
};

use rusqlite::Connection;
pub mod actions;
fn main() -> Result<(), String> {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("project-manager");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    };
    match Connection::open(path.join("project.db")) {
        Ok(conn) => {
            let _ = actions::create_table(&conn);
            let args = Vec::from(&args().collect::<Vec<String>>()[1..]);
            if args.len() == 0 {
                return Err("No arguments provided".to_owned());
            }
            match &args[0] {
                x if x == "add" => {
                    if args.len() < 3 {
                        return Err("Not enough arguments".to_owned());
                    }
                    let name = args[1].clone();
                    let path = args[2].clone();
                    match path::PathBuf::from(&path).canonicalize() {
                        Ok(path) => {
                            actions::add_to_table(&conn, &name, path.to_str().to_owned().unwrap())?
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
                x if x == "list" => match actions::list_projects(&conn, &None) {
                    Ok(rows) => {
                        for row in rows {
                            println!("{}", &row)
                        }
                    }
                    Err(err) => return Err(err),
                },
                x if x == "delete" => {
                    if args.len() < 2 {
                        return Err("Not enough arguments".to_owned());
                    }
                    let index = args[1].clone().parse::<i32>();
                    if let Err(_) = index {
                        return Err(
                            "Please parse the integer id of the project to delete".to_owned()
                        );
                    }
                    if let Err(err) = actions::delete_from_table(&conn, index.unwrap()) {
                        return Err(err);
                    }
                }
                x => match actions::list_projects(&conn, &Some(x.to_string())) {
                    Ok(row) => match row.first() {
                        Some(item) => print!("{}", item.path()),
                        None => return Err("No row matched project name".to_owned()),
                    },
                    Err(err) => return Err(err),
                },
            }
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}
