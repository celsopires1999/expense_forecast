#![allow(unused)] // For beginning only

use uuid::Uuid;

use crate::prelude::*;

mod error;
mod prelude;
mod team;
mod utils;

fn main() -> Result<()> {
    print!("Expense Forecast Project");
    let my_uuid = Uuid::new_v4();

    println!("{}", my_uuid);
    Ok(())
}
