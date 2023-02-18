use rusqlite::params;

fn main() {
    let conn = rusqlite::Connection::open("foo.sqlite3").unwrap();

    fn get_row(row: &rusqlite::Row) -> rusqlite::Result<Option<String>> {
        row.get(0)
    }

    conn.query_row("SELECT 1", params![], get_row).unwrap();
}
