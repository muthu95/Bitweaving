use std::fs::File;
use std::io::{BufReader, Error, BufWriter};
use std::io::prelude::*;

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

//K is number of words in a segment. (As in paper)
const K: usize =  8;

//B is size of each Bit Group. (As in paper)
const B: usize = 2;

fn write_usize(data: &usize, buf_writer: &mut BufWriter<File>) -> Result<(), Error> {
    buf_writer.write(&data.to_be_bytes())?;
    Ok(())
}
fn read_usize(buf_reader: &mut BufReader<File>) -> Result<(usize), Error> {
    //TODO check size of usize. in 32bit machine its 4 bytes only
    let mut buffer: [u8; 8] = [0; 8];
    let bytes_read = buf_reader.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(0);
    }
    let data = usize::from_be_bytes(buffer);
    Ok(data)
}

fn write_bg_to_file(filename: &String, bit_groups: &Vec<Vec<u32>>, segment_counter: &usize) -> Result<(), Error> {
    let output_f = File::create(&filename)?;
    let mut buf_writer = BufWriter::new(output_f);

    //First 8byte sequence in the output file contains K.
    write_usize(&K, &mut buf_writer)?;

    //Second 8byte sequence in the output file contains B.
    write_usize(&B, &mut buf_writer)?;

    //Third 8byte sequence in the output file contains number of segments.
    write_usize(&segment_counter, &mut buf_writer)?;

    for i in 0..bit_groups.len() {
        //8byte sequence containing size of i'th bit group.
        write_usize(&bit_groups[i].len(), &mut buf_writer)?;

        //Writing words of i'th bit group.
        for j in 0..bit_groups[i].len() {
            buf_writer.write_u32::<BigEndian>(bit_groups[i][j])?;
        }
    }
    println!("Successfully wrote the index into file.");
    Ok(())
}

fn read_bg_from_file(filename: &String) -> Result<(), Error> {
    let input_f = File::open(&filename)?;
    let mut buf_reader = BufReader::new(input_f);
    let k_from_file = read_usize(&mut buf_reader).unwrap();
    let b_from_file = read_usize(&mut buf_reader).unwrap();
    let num_segments = read_usize(&mut buf_reader).unwrap();
    println!("Reading from file");
    println!("K: {}", k_from_file);
    println!("B: {}", b_from_file);
    println!("segCount: {}", num_segments);
    let mut bit_groups: Vec<Vec<u32>> = Vec::new();
    loop {
        let mut bg_size = read_usize(&mut buf_reader).unwrap();
        if bg_size == 0 {
            break;
        }
        let mut current_bg: Vec<u32> = Vec::new();
        while bg_size != 0 {
            let data = buf_reader.read_u32::<BigEndian>().unwrap();
            current_bg.push(data);
            bg_size -= 1;
        }
        bit_groups.push(current_bg);
    }

    println!("BIT GROUP: {:?}", bit_groups);
    Ok(())
}

fn process_segment(segment: &[u32], bit_groups: &mut Vec<Vec<u32>>) {
    println!("Elements in segment: {:?}", segment);
    
    let mut i:usize = K;

    while i > 0 {
        //i is the index for iterating through each bit from MSB to LSB.
        i -= 1;

        let mut val:u32 = 0;
        let mut j:usize = 0;
        while j < K {
            //j is the index for iterating through each word of the segment.

            //println!("In {}th position, number = {} & currentBit = {}", i, segment[j], (segment[j]>>i)&1);

            //Here, we accumulate the i'th bit word in 'val'
            // by taking i'th bit from each word of the segment.
            val = (val << 1) | ((segment[j] >> i) & 1);
            j += 1;
        }

        //Finding the bit group index.
        let bg_index: usize = (K/B) - 1 - (i/B);

        //If the index isn't found, creating an empty vector.
        if bg_index >= bit_groups.len() {
            bit_groups.push(Vec::new());
        }

        //Pushing word in the corressponding bit group.
        bit_groups[bg_index].push(val);
    }
}

pub fn create_byte_code(filename: String, mut bit_groups: &mut Vec<Vec<u32>>) -> Result<(), Error> {
    println!("Creating bytecode for the file: {}", filename);

    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    let mut lines_read: usize = 0;
    let mut current_segment: [u32; K] = [0; K];
    let mut segment_counter: usize = 0;

    //Read input file line by line
    for line in buffered.lines() {
        //println!("{}", line?);

        //Unwrapping Result gives the line as String.
        let string_line: String = line.unwrap();

        //Parsing it to u32
        current_segment[lines_read % K] = string_line.parse::<u32>().unwrap();

        lines_read = lines_read + 1;

        //If a segment is filled up, then process it.
        if lines_read % K == 0 {
            process_segment(&current_segment, &mut bit_groups);
            segment_counter += 1;
        }
    }

    //Some partially filled segment found in the end.
    //TODO Handle this.
    /*if lines_read % WORDS_PER_SEGMENT != 0 {
        process_segment(&current_segment, &mut bit_groups);
    }*/

    let idx_filename = String::from("int_column_index");
    write_bg_to_file(&idx_filename, &bit_groups, &segment_counter)?;
    read_bg_from_file(&idx_filename)?;
    Ok(())
}

