#![allow(unused)] // For beginning only

use expense_forecast::prelude::*;
use expense_forecast::*;

fn main() -> Result<()> {
    println!("Expense Forecast Project");
    let member = team::Member::new("John Doe")?;
    println!("{}", member.name);
    Ok(())
}
