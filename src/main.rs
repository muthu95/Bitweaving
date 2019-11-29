#![feature(asm)]

use std::fs::File;
use std::io::{Write, Error};

use packed_simd::u32x8;

mod bitgroup;
mod naivescan;
mod index_builder;
mod simd_scanner;

use self::bitgroup::BitGroup;
use self::bitgroup::index_builder2;
use self::bitgroup::scanner;
use self::naivescan::naive_scanner;

extern crate bit_vec;

fn fill_inp_file(arr: &[u32], inp_filename: &String) -> Result<(), Error> {
    let mut output = File::create(&inp_filename)?;
    for k in 0..1280000 {
        write!(output, "{}\n", arr[k])?;
    }
    Ok(())
}


fn main() -> Result<(), Error> {
    index_builder::create_column_store("src/sample.csv", "output_col", 3);
    let mut arr: [u32; 1280000] = [0; 1280000];
    for i in 0..640000 {
        arr[i as usize] = i;
    }
    let mut j:usize = 640000;
    for i in 0..640000 {
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
            //println!("val  {}", bit_group.bit_groups[i][j]);
        }
    }
    
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    unsafe {
        asm!("
                    rdtscp\n
                    shl rdx, 32\n
                    or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

        //scanner::scan_between(bit_group, 30, 40);
        simd_scanner::scan_between(bit_group, 30, 40);
        asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");    
    }

    println!("Bitweaving scan - cpu cycles: {}", diff_late - diff_early);

    diff_early = 0;
    diff_late = 0;

    unsafe {
        asm!("
                    rdtscp\n
                    shl rdx, 32\n
                    or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

        naivescan::naive_scanner::scan_between(&arr, 30, 40);

        asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
    }

    println!("Naive scan - cpu cycles: {}", diff_late - diff_early);

    
    Ok(())
}
