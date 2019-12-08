use std::fs::File;
use std::io::{BufReader, Error, BufWriter};
use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::error::Error as Err;

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use super::BitGroup;
//K is number of bits to encode a columnar value. (As in paper)
const K: usize = 32;

//B is size of each Bit Group. (As in paper)
const B: usize = 4;

//Processor word length and number of words in a segment. (As in paper)
const W: usize = 32;

//NOTE: ENSURE K is divisible by B


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

fn process_segment(segment: &[u32], bit_groups: &mut Vec<Vec<u32>>) {

    let mut i:usize = K;

    while i > 0 {
        //i is the index for iterating through each bit from MSB to LSB.
        i -= 1;

        let mut val:u32 = 0;
        let mut j:usize = 0;
        while j < W {
            /*
                j is the index for iterating through each word of the segment.
                Here, we accumulate the i'th bit word in 'val'
                by taking i'th bit from each word of the segment.
            */
            val = (val << 1) | ((segment[j] >> i) & 1);
            j += 1;
        }

        let bg_index: usize = (K/B) - 1 - (i/B);

        //If the index isn't found, creating an empty vector.
        if bg_index >= bit_groups.len() {
            bit_groups.push(Vec::new());
        }
        bit_groups[bg_index].push(val);
    }
}

pub fn create_bg_file(bit_group: &mut BitGroup, inp_filename: &String, bg_filename: &String) -> Result<(), Error> {
    println!("Creating bytecode for the file: {}", inp_filename);

    let input = File::open(inp_filename)?;
    let buffered = BufReader::new(input);

    let mut lines_read: usize = 0;
    let mut current_segment: [u32; W] = [0; W];
    let mut segment_counter: usize = 0;
    let mut bit_groups: Vec<Vec<u32>> = Vec::new();

    //Read input file line by line
    for line in buffered.lines() {
       
        let string_line: String = line.unwrap();
        current_segment[lines_read % W] = string_line.parse::<u32>().unwrap();
        lines_read = lines_read + 1;

        //If a segment is filled up, then process it.
        if lines_read % W == 0 {
            process_segment(&current_segment, &mut bit_groups);
            segment_counter += 1;
        }
    }

    //Some partially filled segment found in the end.
    //TODO Handle this.
    /*if lines_read % WORDS_PER_SEGMENT != 0 {
        process_segment(&current_segment, &mut bit_groups);
    }*/
    bit_group.k = K;
    bit_group.b = B;
    bit_group.w = W;
    bit_group.segment_size = segment_counter;
    bit_group.bit_groups = bit_groups;

    bit_group.write_file(bg_filename);
    Ok(())
}
