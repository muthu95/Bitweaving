use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::prelude::*;

const BITS_PER_WORD:usize = 32;
const WORDS_PER_SEGMENT:usize = 32;
const WORDS_PER_BIT_GROUP:usize = 8;
const SEGMENTS_PER_BIT_GROUP:usize = 2;
const WORDS_OF_SEGMENT_PER_BIT_GROUP:usize = (WORDS_PER_BIT_GROUP/SEGMENTS_PER_BIT_GROUP);

pub fn create_column_store(input_file: &str, output_file: &str, num_cols: u64) {

    //TODO -> Try to detect the number of columns in the file efficiently

    let path = Path::new(input_file);
    let display = path.display();

    let read_file = File::open(&path).expect("Unable to open");
    let mut input_file_reader = io::BufReader::new(read_file);
    let mut buffer = String::new();


    let mut output_writers = Vec::new();
    for i in 0..num_cols {
        let output_file_name = format!("{}{}", output_file, i);
        let mut write_file = match File::create(&output_file_name) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut writer = io::BufWriter::new(write_file);
        output_writers.push(writer);
    }

    let mut is_first_line = true;
    while input_file_reader.read_line(&mut buffer).expect("Unable to read") > 0 {
        {
            let items = buffer.split(",");
            let mut writer_index = 0;
            for item in items {
                if !is_first_line {
                    output_writers[writer_index].write("\n".as_bytes());
                }
                output_writers[writer_index].write(item.trim().as_bytes());
                writer_index += 1;
            }
            is_first_line = false;
        }
        buffer.clear();
    }

}

fn process_segment(segment_number: &usize, segment: &[u32], bit_groups: &mut Vec<[u32; WORDS_PER_BIT_GROUP]>) {
    println!("Segment Number: {}", segment_number);
    println!("Elements in segment: {:?}", segment);
    let mut i:usize = BITS_PER_WORD;
    let mut k:usize = 0;
    while i > 0 {
        i = i-1;
        let mut val:u32 = 0;
        let mut j:usize = 0;
        while j < segment.len() {
            //println!("In {}th position, number = {} & currentBit = {}", i, segment[j], (segment[j]>>i)&1);
            val = (val<<1)|((segment[j]>>i)&1);
            j = j+1;
        }
        //println!("{}th position val is {}", i, val);
        let bit_group_number:usize = (BITS_PER_WORD/WORDS_OF_SEGMENT_PER_BIT_GROUP) - 1 - (i/WORDS_OF_SEGMENT_PER_BIT_GROUP);
        let index_within_bit_group:usize = (segment_number*WORDS_OF_SEGMENT_PER_BIT_GROUP) + (k%WORDS_OF_SEGMENT_PER_BIT_GROUP);
        //println!("bit_group_number: {}, index_within_bit_group: {}", bit_group_number, index_within_bit_group);
        if bit_group_number >= bit_groups.len() {
            //Create a bit groups
            bit_groups.push([0; WORDS_PER_BIT_GROUP]);
        }
        bit_groups[bit_group_number][index_within_bit_group] = val;
        k = k+1;
    }
}

pub fn create_byte_code(arr: &[u32]) {
    println!("Received {} elements", arr.len());

    let number_of_segments:usize = arr.len()/WORDS_PER_SEGMENT;
    let number_of_bit_groups:usize = arr.len()/WORDS_PER_BIT_GROUP;
    println!("No of Segments: {}", number_of_segments);
    println!("No of Bit Groups: {}", number_of_bit_groups);

    //TODO: Handle a case where the number of values is not a exact multiple

    /* Vectors because we dont know the number of bit groups during compile time.
    Each bit group is array because they should be stored continuously in memory.*/
    let mut bit_groups: Vec<[u32; WORDS_PER_BIT_GROUP]> = Vec::new();

    let mut i:usize = 0;
    while i < arr.len() {
        process_segment(&(i/WORDS_PER_SEGMENT), &arr[i .. i+WORDS_PER_SEGMENT], &mut bit_groups);
        i += WORDS_PER_SEGMENT;
    }
    println!("BIT GROUP: {:?}", bit_groups);
}
