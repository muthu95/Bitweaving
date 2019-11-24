use std::fs::File;
use std::io::{Write, Error};
mod bitgroup;
mod index_builder;

use self::bitgroup::BitGroup;
use self::bitgroup::index_builder2;
use self::bitgroup::scanner;
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
        arr[i as usize] = i;
    }
    let mut j:usize = 64;
    for i in 0..64 {
        arr[j] = i;
        j += 1;
    }

    let inp_filename = String::from("int_column");
    //Use this once to create the "int_column" file for creating the input test file
    fill_inp_file(&arr, &inp_filename)?;


    let mut bit_group = BitGroup {k: 0, b: 0, segment_size: 0, bit_groups: Vec::new()};

    let bg_filename = String::from("int_column_index");
    index_builder2::create_bg_file(&mut bit_group, &inp_filename, &bg_filename)?;
    
    //let mut bit_groups: Vec<Vec<u32>> = Vec::new();
    //index_builder2::read_bg_from_file(&bg_filename, &mut bit_groups)?;

    println!("Reading from file");
    bit_group.read_file(&bg_filename);

    //println!("BIT GROUP: {:?}", bit_groups);
    for i in 0..bit_group.bit_groups.len() {
        for j in 0..bit_group.bit_groups[i].len() {
            println!("val  {}", bit_group.bit_groups[i][j]);
        }
    }
    
    scanner::scan_between(bit_group, 30, 40);
    Ok(())
}
