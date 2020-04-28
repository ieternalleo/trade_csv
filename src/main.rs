extern crate chrono;
extern crate csv;

use chrono::prelude::*;
use std::error::Error;
use std::fs::File;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    //Create a variable that points to our data file
    let file = File::open("data/Orders_042820.csv")?;
    //Build the CSV reader and iterate over each record
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

//TODO Create a ticket struct as a interface for account reports
fn main() {
    if let Err(err) = example() {
        println!("Error running example: {}", err);
        process::exit(1);
    }
}
