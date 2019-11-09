use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error as Err;
use std::io::{BufReader, Error};
use std::io::prelude::*;

//Following are configuration variables. Change and see the results.
// processor word size
const BITS_PER_WORD:usize = 32;

/* 
have the value same as processor word  -> k 
(number of words in the column grouped as segments)
*/
const WORDS_PER_SEGMENT:usize = 32;

// maximum limit on the number of words in the bit group
const WORDS_PER_BIT_GROUP:usize = 8;

// To put a limit on number of segments that can be placed in the bit group
const SEGMENTS_PER_BIT_GROUP:usize = 4;

//Number of words of a segment that belong to 1 bit group.
const WORDS_OF_SEGMENT_PER_BIT_GROUP:usize = (WORDS_PER_BIT_GROUP / SEGMENTS_PER_BIT_GROUP);

//Number of bit groups than a segment spans.
const BIT_GROUP_SPAN_OF_A_SEGMENT:usize = (WORDS_PER_SEGMENT / WORDS_OF_SEGMENT_PER_BIT_GROUP);

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

fn process_segment(segment_number: usize, segment: &[u32], segment_len: usize, bit_groups: &mut Vec<[u32; WORDS_PER_BIT_GROUP]>) {
    println!("Segment Number: {}", segment_number);
    println!("Elements in segment: {:?}", segment);

    let mut i:usize = BITS_PER_WORD;
    let mut k:usize = 0;
    while i > 0 {
        //i is the index for iterating through each bit from MSB to LSB.
        //It effectively runs from BITS_PER_WORD-1 to 0.
        i -= 1;

        let mut val:u32 = 0;
        let mut j:usize = 0;
        while j < segment_len {
            //j is the index for iterating through each word of the segment.

            //println!("In {}th position, number = {} & currentBit = {}", i, segment[j], (segment[j]>>i)&1);

            //Here, we accumulate the i'th bit word in 'val'
            // by taking i'th bit from each word of the segment.
            val = (val << 1) | ((segment[j] >> i) & 1);
            j += 1;
        }
        //Now 'val' will have i'th bits of all words in the segment.

        //println!("{}th position val is {}", i, val);

        //See the LAYOUT in Line 131 to understand how I arrived at these 2 formulas.
        //Also uncomment Line 95 for easy understanding.
        let bit_group_number:usize = ((segment_number / SEGMENTS_PER_BIT_GROUP) * BIT_GROUP_SPAN_OF_A_SEGMENT) + (k / WORDS_OF_SEGMENT_PER_BIT_GROUP);

        let index_within_bit_group:usize = ((segment_number * WORDS_OF_SEGMENT_PER_BIT_GROUP) +
        (k % WORDS_OF_SEGMENT_PER_BIT_GROUP)) % WORDS_PER_BIT_GROUP;

        //println!("bit_group_number: {}, index_within_bit_group: {}", bit_group_number, index_within_bit_group);

        if bit_group_number >= bit_groups.len() {
            //Create a bit group.
            bit_groups.push([0; WORDS_PER_BIT_GROUP]);
        }
        bit_groups[bit_group_number][index_within_bit_group] = val;

        //k is used to calculate bit_group_number & index_within_bit_group.
        //It runs from [0, BITS_PER_WORD].
        k += 1;
    }
}

pub fn create_byte_code_from_array(arr: &[u32]) {
    println!("Received {} elements", arr.len());

    let number_of_segments:usize = arr.len() / WORDS_PER_SEGMENT;
    let number_of_bit_groups:usize = arr.len() / WORDS_PER_BIT_GROUP;
    println!("No of Segments: {}", number_of_segments);
    println!("No of Bit Groups: {}", number_of_bit_groups);

    //TODO: Handle a case where the number of values is not a exact multiple

    /* Vectors because we dont know the number of bit groups during compile time.
    Each bit group is array because they should be stored continuously in memory.*/
    //Vector of arrays
    let mut bit_groups: Vec<[u32; WORDS_PER_BIT_GROUP]> = Vec::new();

    let mut i:usize = 0;
    while i < arr.len() {
        //Take each segment and put all the words inside it, into corressponding bit groups.
        process_segment(i / WORDS_PER_SEGMENT, &arr[i .. i+WORDS_PER_SEGMENT], WORDS_PER_SEGMENT, &mut bit_groups);
        i += WORDS_PER_SEGMENT;
    }

    // LAYOUT
    //For the given 64 values, Now each bit group will have
    //BG0 -> [seg0W31, seg0W30, seg0W29, seg0W28, seg1W31, seg1W30, seg1W29, seg1W28]
    //BG1 -> [seg0W27, seg0W26, seg0W25, seg0W24, seg1W27, seg1W26, seg1W25, seg1W24]
    //.
    //.
    //.
    //BG7 -> [seg0W3, seg0W2, seg0W1, seg0W0, seg1W3, seg1W2, seg1W1, seg1W0]
    //BG8 -> [seg2W31, seg2W30, seg2W29, seg2W28, seg3W31, seg3W30, seg3W29, seg3W28]
    //BG9 -> [seg2W27, seg2W26, seg2W25, seg2W24, seg3W27, seg3W26, seg3W25, seg3W24]
    //.
    //.
    //.
    //BG15 -> [seg2W3, seg2W2, seg2W1, seg2W0, seg3W3, seg3W2, seg3W1, seg3W0]
    println!("BIT GROUP: {:?}", bit_groups);
}

pub fn create_byte_code_from_file(filename: String) -> Result<(), Error> {
    println!("Creating bytecode for the file: {}", filename);

    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    let mut bit_groups: Vec<[u32; WORDS_PER_BIT_GROUP]> = Vec::new();
    let mut lines_read: usize = 0;
    let mut current_segment: [u32; WORDS_PER_SEGMENT] = [0; WORDS_PER_SEGMENT];

    //Read input file line by line
    for line in buffered.lines() {
        //println!("{}", line?);
        //Unwrapping Result gives the line as String.
        let string_line: String = line.unwrap();
        //Parsing it to u32
        current_segment[lines_read % WORDS_PER_SEGMENT] = string_line.parse::<u32>().unwrap();
        lines_read = lines_read + 1;
        //If a segment is filled up, then process it.
        if lines_read % WORDS_PER_SEGMENT == 0 {
            process_segment((lines_read/WORDS_PER_SEGMENT)-1, &current_segment, WORDS_PER_SEGMENT, &mut bit_groups)
        }
    }

    //Some partially filled segment found in the end.
    //TODO Handle this.
    /*if lines_read % WORDS_PER_SEGMENT != 0 {
        process_segment(lines_read/WORDS_PER_SEGMENT, &current_segment, lines_read % WORDS_PER_SEGMENT, &mut bit_groups);
    }*/

    println!("BIT GROUP: {:?}", bit_groups);
    Ok(())
}
