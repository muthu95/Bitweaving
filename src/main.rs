#![feature(asm)]

use std::fs::File;
use std::io::{BufReader};
use std::io::{Write, Error};
use std::io::prelude::*;

mod bitgroup;
mod naivescan;
mod simd_scanner;
mod simd_scanner_128;
mod simd_scanner_256;

use self::bitgroup::BitGroup;
use self::bitgroup::index_builder;
use self::bitgroup::scanner;
use self::naivescan::naive_scanner;

use std::collections::HashMap;

extern crate core_affinity;
 
use std::thread;

extern crate config;
extern crate bit_vec;

//Can be used to create a custom index file
fn fill_input_file(arr: &[u32], inp_filename: &String) -> Result<(), Error> {
    let mut output = File::create(&inp_filename)?;
    for k in 0..1280000 {
        write!(output, "{}\n", arr[k])?;
    }
    Ok(())
}

fn print_bitgroup(bit_group: &BitGroup) {
    for i in 0..bit_group.bit_groups.len() {
        for j in 0..bit_group.bit_groups[i].len() {
            print!("{} ", bit_group.bit_groups[i][j]);
        }
        println!();
    }
}

fn main() -> Result<(), Error> {

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();
    let input_path = settings_map.get(&"input_path".to_string());
    let output_path = settings_map.get(&"output_path".to_string());
   
    index_builder::create_column_store(&[&input_path.unwrap(), "sample.csv"].concat(), &[&output_path.unwrap(), "output_col"].concat(), 2);
    //Set the column for which you need to build the bitgroup
    let index_column = "output_col1";
    let input_filename = &[&output_path.unwrap(), index_column].concat();
    let mut bit_group = BitGroup::new(0, 0, 0, 0, Vec::new(), Box::new([1, 1]));
    let bg_filename = format!("{}{}", output_path.unwrap(), &String::from(format!("{}_index", index_column)));
    index_builder::create_bg_file(&mut bit_group, &input_filename, &bg_filename)?;
    bit_group.read_file(&bg_filename)?;

    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;
    let core_ids = core_affinity::get_core_ids().unwrap();
    let handle =  thread::spawn(move || {
        // Pinning the below execution to the first core in the list.
        core_affinity::set_for_current(core_ids[0]);
        // Need to do bitgroup read again as it should be accessible in the scope

        //TODO: Need to check on sharing the variable across multiple threads
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name("Settings")).unwrap();
        let mut settings_map = settings.try_into::<HashMap<String, String>>().unwrap();
        let input_path = settings_map.get(&"input_path".to_string()); // can this be done better?
        let output_path = settings_map.get(&"output_path".to_string());

        let mut bit_group = BitGroup::new(0, 0, 0, 0, Vec::new(), Box::new([1, 1]));

        let index_column = "output_col1";
        format!("{}{}", output_path.unwrap(), &String::from(format!("{}_index", index_column)));
        let input_filename = &[&output_path.unwrap(), index_column].concat();

        index_builder::create_bg_file(&mut bit_group, &input_filename, &bg_filename);
        bit_group.read_file(&bg_filename);

        unsafe {
            asm!("
                        rdtscp\n
                        shl rdx, 32\n
                        or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
    
            scanner::scan_between(bit_group, 2, 10);

            asm!("
                    rdtscp\n
                    shl rdx, 32\n
                    or rax, rdx\n
                    ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");    
        }

        println!("Bitweaving scan - cpu cycles: {}", diff_late - diff_early);
    });

    handle.join().unwrap();
 
    unsafe {
        asm!("
                    rdtscp\n
                    shl rdx, 32\n
                    or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

        simd_scanner_128::scan_between(&bit_group, 2, 10);

        asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");    
    }

    println!("Bitweaving SIMD scan - cpu cycles: {}", diff_late - diff_early);
    
    diff_early = 0;
    diff_late = 0;

    unsafe {
        asm!("
                    rdtscp\n
                    shl rdx, 32\n
                    or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

        simd_scanner_256::scan_between(&bit_group, 2, 10);

        asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");    
    }

    println!("Bitweaving SIMD256 scan - cpu cycles: {}", diff_late - diff_early);

    let mut arr: Vec<u32> = Vec::new();
    let input_file = File::open(&input_filename)?;
    let mut buf_reader = BufReader::new(input_file);
    for line in buf_reader.lines() {       
        let string_line: String = line.unwrap();
        arr.push(string_line.parse::<u32>().unwrap());
    }
    
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
