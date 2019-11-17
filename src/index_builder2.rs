use std::io;
use std::fs::File;
use std::io::{BufReader, Error, Write};
use std::io::prelude::*;

//k is number of words in a segment. (As in paper)
const k: usize =  8;

//B is size of each Bit Group. (As in paper)
const B: usize = 2;

fn write_bg_to_file(bit_groups: &Vec<Vec<u32>>) -> Result<(), Error> {
    let mut outputF = File::create("int_column_index")?;

    //First line in the output file contains k.
    write!(outputF, "{}\n", k)?;

    //Second line in the output file contains B.
    write!(outputF, "{}\n", B)?;

    for i in 0..bit_groups.len() {
        //Writing the size of i'th bit group.
        write!(outputF, "{}\n", bit_groups[i].len())?;

        //Writing words of i'th bit group.
        for j in 0..bit_groups[i].len() {
            write!(outputF, "{}\n", bit_groups[i][j])?;
        }
    }
    println!("Successfully wrote the index into file.");
    Ok(())
}

fn process_segment(segment: &[u32], bit_groups: &mut Vec<Vec<u32>>) {
    println!("Elements in segment: {:?}", segment);
    
    let mut i:usize = k;

    while i > 0 {
        //i is the index for iterating through each bit from MSB to LSB.
        i -= 1;

        let mut val:u32 = 0;
        let mut j:usize = 0;
        while j < k {
            //j is the index for iterating through each word of the segment.

            //println!("In {}th position, number = {} & currentBit = {}", i, segment[j], (segment[j]>>i)&1);

            //Here, we accumulate the i'th bit word in 'val'
            // by taking i'th bit from each word of the segment.
            val = (val << 1) | ((segment[j] >> i) & 1);
            j += 1;
        }

        //Finding the bit group index.
        let bg_index: usize = (k/B) - 1 - (i/B);

        //If the index isn't found, creating an empty vector.
        if bg_index >= bit_groups.len() {
            bit_groups.push(Vec::new());
        }

        //Pushing word in the corressponding bit group.
        bit_groups[bg_index].push(val);
    }
}

pub fn create_byte_code(filename: String) -> Result<(), Error> {
    println!("Creating bytecode for the file: {}", filename);

    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    let mut bit_groups: Vec<Vec<u32>> = Vec::new();
    let mut lines_read: usize = 0;
    let mut current_segment: [u32; k] = [0; k];

    //Read input file line by line
    for line in buffered.lines() {
        //println!("{}", line?);

        //Unwrapping Result gives the line as String.
        let string_line: String = line.unwrap();

        //Parsing it to u32
        current_segment[lines_read % k] = string_line.parse::<u32>().unwrap();

        lines_read = lines_read + 1;

        //If a segment is filled up, then process it.
        if lines_read % k == 0 {
            process_segment(&current_segment, &mut bit_groups);
        }
    }

    //Some partially filled segment found in the end.
    //TODO Handle this.
    /*if lines_read % WORDS_PER_SEGMENT != 0 {
        process_segment(&current_segment, &mut bit_groups);
    }*/

    println!("BIT GROUP: {:?}", bit_groups);
    write_bg_to_file(&bit_groups);
    Ok(())
}
