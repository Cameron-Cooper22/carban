use rusqlite::Connection;

pub struct State {
    pub project_name: String,
    pub selected_column_id: usize,
    pub lists: Vec<List>,
    pub conn: Conn,
}

pub mod input;
pub mod ui;

pub struct Conn(pub Connection);

pub struct List {}
