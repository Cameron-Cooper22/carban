use std::{error::Error, path::PathBuf};

use clap::{Parser, ValueHint::FilePath};
use rusqlite::Connection;

#[derive(Debug, Parser)]
#[command(name = "carban")]
pub struct Args {
    #[arg(value_name="DATABASE", value_hint=FilePath, index=1)]
    pub path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let dbpath = Args::parse().path.unwrap_or(PathBuf::from("./carban.db"));

    let mut conn = Connection::open(dbpath.clone())?;

    if !dbpath.exists() {
        let sql_cmd = r#"
            PRAGMA foreign_keys = ON;

            create table if not exists task
            (
                id integer primary key autoincrement,
                title text not null,
                description text not null,
                sort_order integer not null,
                column_id integer,
                foreign key (column_id) references kb_column(id)
            );

            create table if not exists kb_column
            (
                id integer primary key autoincrement,
                title text not null,
            );
            "#;
        let cmds: Vec<&str> = sql_cmd.split(';').collect();
        let tx = conn.transaction()?;
        for m in cmds {
            if !m.trim().is_empty() {
                tx.execute(m, ())?;
            }
        }
        tx.commit()?;
    }

    Ok(())
}
