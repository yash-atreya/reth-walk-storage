use alloy_primitives::address;
use eyre::Ok;
use reth_db::{
    cursor::DbCursorRO, database::Database, open_db_read_only, tables, transaction::DbTx,
};
use std::path::Path;

fn main() -> Result<(), eyre::Report> {
    // Init DB
    let db = open_db_read_only(
        Path::new(&std::env::var("RETH_DB_PATH")?),
        Default::default(),
    )?;

    // Init tx
    let tx = db.tx()?;

    let mut cursor = tx.cursor_dup_read::<tables::PlainStorageState>()?;

    let mut walk_slots = cursor.walk(Some(address!("CBCdF9626bC03E24f779434178A73a0B4bad62eD")))?;

    let mut i: u64 = 0;
    let start_t = std::time::Instant::now();
    while let Some(_it) = walk_slots.next() {
        // let (key, value) = it?;
        i += 1;
    }
    println!("Time taken to walk {:?} slots: {:?}", i, start_t.elapsed());

    Ok(())
}
