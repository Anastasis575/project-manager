use std::{env::args, fs, fmt::Display, path};

use rusqlite::{Connection, Error};

fn main() -> Result<(), String> {
    match Connection::open("./projects.db") {
        Ok(conn) => {
            let _ = create_table(&conn);
            let args = Vec::from(&args().collect::<Vec<String>>()[1..]);
            match &args[0] {
                x if x == "add" => {
                    if args.len() < 3 {
                        return Err("Not enough arguments".to_owned());
                    }
                    let name = args[1].clone();
                    let path = args[2].clone();
                    match path::PathBuf::from(&path).canonicalize() {
                        Ok(path) => {
                            if let Err(err) = add_to_table(&conn, &name, &path.to_str().to_owned().unwrap()) {
                                return Err(err);
                            };
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
                x if x == "list" => {
                    let option:Option<String>;
                    if args.len() < 2 {
                        option=None;
                    }else{
                        option=Some(args[1].to_owned())
                    }
                    match list_projects(&conn,option) {
                        Ok(rows) => for row in rows { println!("{}",row.get) },
                        Err(err) => return Err(err),
                    }

                },
                _ => return Err(String::from("Could not find call")),
            }
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug)]
struct Row{
    project_name:String,
    path:String
}

impl Row {
    fn project_name(&self) -> &str {
        self.project_name.as_ref()
    }

    fn path(&self) -> &str {
        self.path.as_ref()
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}",self.project_name,self.path))
    }
}

fn list_projects(conn: &Connection, option: Option<String>) -> Result<Vec<Row>, String> {
    let query=format!("select * from  projects {}",if let Some(search)=option{format!("where project_name like '%{}%';",search)}else {";".to_owned()});
    println!("{}",&query);
    match conn.prepare(&query){
        Ok(mut finals) => {
            let rows=finals.query_map([],|row|{
                Ok(Row{
                    project_name:row.get_unwrap::<&str,String>("project_name"),
                    path:row.get_unwrap::<&str,String>("path")
                })
            }).unwrap().collect::<Vec<Result<Row,Error>>>();
            Ok(rows.into_iter().map(|f|f.unwrap()).collect::<Vec<Row>>())
        },
        Err(err) => Err(err.to_string()),
    }
}

fn add_to_table(conn: &Connection, name: &str, path: &str) -> Result<(), String> {
    if let Err(err) = conn.execute("insert into projects(project_name,path) values(?1,?2)", [name, path]) {
        Err(err.to_string())
    } else {
        Ok(())
    }
}

fn create_table(conn: &Connection) -> Result<(), Error> {
    if let Err(err) = conn.execute(
        "create table if not exists projects(
        id integer primary key autoincrement,
        project_name text not null unique,
        path text not null
    )",
        [],
    ) {
        Err(err)
    } else {
        Ok(())
    }
}
