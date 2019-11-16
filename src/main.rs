use std::fs::File;
use std::io::{Write, Error};
mod index_builder2;
mod index_builder;
mod scanner;

extern crate bit_vec;

fn write_to_file(arr: &[u32]) -> Result<(), Error> {
    let mut output = File::create("int_column")?;
    for k in 0..128 {
        write!(output, "{}\n", arr[k])?;
    }
    Ok(())
}

fn main() {

    index_builder::create_column_store("src/sample.csv", "output_col", 3);
    let mut arr: [u32; 128] = [0; 128];
    for i in 0..64 {
        arr[i as usize] = i+1;
    }
    let mut j:usize = 64;
    for i in 0..64 {
        arr[j] = i+1;
        j += 1;
    }

    //Use this once to create the "int_column" file for creating the input test file
    write_to_file(&arr);

    match index_builder2::create_byte_code(String::from("int_column")) {
        Err(e) => println!("error creating bytecode: {:?}", e),
        Ok(()) => println!("Successfully created bytecode for the file"),
    }

}
