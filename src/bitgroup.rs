use std::io;
use std::fs::File;
use std::io::{BufReader, Error, Write};
use std::io::prelude::*;

fn write_file(bit_groups: &Vec<Vec<u32>>) -> Result<(), Error> {
    let mut outputF = File::create("int_column_index")?;
    write!(outputF, "{}\n", k)?; //First line in the output file contains k.
    write!(outputF, "{}\n", B)?; //Second line in the output file contains B.
    for i in 0..bit_groups.len() {
        write!(outputF, "{}\n", bit_groups[i].len())?; //Writing the size of i'th bit group.
        for j in 0..bit_groups[i].len() {
            write!(outputF, "{}\n", bit_groups[i][j])?; //Writing words of i'th bit group.
        }
    }
    println!("Successfully wrote the index into file.");
    Ok(())
}
