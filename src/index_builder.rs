use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::prelude::*;


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
