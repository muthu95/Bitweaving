#![feature(asm)]

use std::fs::File;
use std::io::{Write, Error};

mod bitgroup;
mod naivescan;
mod simd_scanner;
mod simd_scanner2;

use self::bitgroup::BitGroup;
use self::bitgroup::index_builder;
use self::bitgroup::scanner;
use self::naivescan::naive_scanner;

use std::collections::HashMap;

extern crate core_affinity;
 
use std::thread;

extern crate config;
extern crate bit_vec;

fn fill_input_file(arr: &[u32], inp_filename: &String) -> Result<(), Error> {
    let mut output = File::create(&inp_filename)?;
    for k in 0..1280000 {
        write!(output, "{}\n", arr[k])?;
    }
    Ok(())
}


fn main() -> Result<(), Error> {

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let mut settings_map = settings.try_into::<HashMap<String, String>>().unwrap();
    let input_path = settings_map.get(&"input_path".to_string()); // can this be done better?
    let output_path = settings_map.get(&"output_path".to_string());
   
    index_builder::create_column_store(&[&input_path.unwrap(), "sample.csv"].concat(), &[&input_path.unwrap(), "output_col"].concat(), 3);
    let mut arr: [u32; 1280000] = [0; 1280000];
    for i in 0..640000 {
        arr[i as usize] = i;
    }
    let mut j:usize = 640000;
    for i in 0..640000 {
        arr[j] = i;
        j += 1;
    }

    let input_filename = &[&input_path.unwrap(), "int_column"].concat();
    fill_input_file(&arr, &input_filename)?;


    let mut bit_group = BitGroup {k: 0, b: 0, segment_size: 0, bit_groups: Vec::new()};

    let bg_filename = &[&input_path.unwrap(), "int_column_index"].concat();
    index_builder::create_bg_file(&mut bit_group, &input_filename, &bg_filename)?;
    bit_group.read_file(&bg_filename);

    
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    let mut core_ids = core_affinity::get_core_ids().unwrap();
 
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
   
            let mut bit_group = BitGroup {k: 0, b: 0, segment_size: 0, bit_groups: Vec::new()};

            let bg_filename = &[&input_path.unwrap(), "int_column_index"].concat();
            let input_filename = &[&input_path.unwrap(), "int_column"].concat();

            index_builder::create_bg_file(&mut bit_group, &input_filename, &bg_filename);
            bit_group.read_file(&bg_filename);

            unsafe {
                asm!("
                            rdtscp\n
                            shl rdx, 32\n
                            or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
        
                //scanner::scan_between(bit_group, 30, 40);
                simd_scanner2::scan_between(bit_group, 30, 40);
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

        scanner::scan_between(bit_group, 30, 40);
        //simd_scanner2::scan_between(bit_group_final, 30, 40);
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
