use super::Error;
use super::Store;

const VERSION: usize = 1;
static MIGRATIONS: &'static [&'static str] = &[include_str!("1.sql")];
static VERSION_CHECK: &'static str = r#"SELECT value FROM wonderful_info WHERE key = "version" LIMIT 1"#;

fn check_version(store: &Store) -> Result<usize, Error> {
    match store.query(VERSION_CHECK, &[]) {
        Ok(q) => q.get(0).get::<usize, String>(0).parse::<usize>().map_err(|e| e.into()),
        Err(_) => Ok(0)
    }
}


pub fn check(store: &Store) -> Result<(), Error> {
    let current = check_version(store)?;
    if current < VERSION {
        for ref migration in &MIGRATIONS[current..VERSION] { store.batch_execute(migration)?; }
    }
    Ok(())
}
