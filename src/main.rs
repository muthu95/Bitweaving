use std::fs::File;
use std::io::{Write, Error};
mod index_builder2;
mod index_builder;
mod scanner;
//mod bitgroup;

extern crate bit_vec;

fn fill_inp_file(arr: &[u32], inp_filename: &String) -> Result<(), Error> {
    let mut output = File::create(&inp_filename)?;
    for k in 0..128 {
        write!(output, "{}\n", arr[k])?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
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

    let inp_filename = String::from("int_column");
    //Use this once to create the "int_column" file for creating the input test file
    fill_inp_file(&arr, &inp_filename)?;

    let bg_filename = String::from("int_column_index");
    index_builder2::create_bg_file(&inp_filename, &bg_filename)?;
    
    let mut bit_groups: Vec<Vec<u32>> = Vec::new();
    index_builder2::read_bg_from_file(&bg_filename, &mut bit_groups)?;
    println!("BIT GROUP: {:?}", bit_groups);
    
    scanner::scan_between(bit_groups, 10, 50);
    Ok(())
}
