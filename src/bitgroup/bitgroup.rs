use std::fs::File;
use std::io::{BufReader, Error, BufWriter};
use std::io::prelude::*;

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub struct BitGroup {

    pub k: usize,
    pub b: usize,
    pub w: usize,
    pub segment_size: usize,
    pub bit_groups: Vec<Vec<u32>>,
    pub bit_group_box: Box<[u32]>,
}

impl BitGroup {

    pub fn new(k: usize, b: usize, w: usize, segment_size: usize, bit_groups: Vec<Vec<u32>>, bit_group_box: Box::<[u32]>) -> BitGroup {
        BitGroup { k: k, b: b, w: w, segment_size: segment_size, bit_groups: bit_groups, bit_group_box: bit_group_box}
    }

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

    pub fn read_file(&mut self, file_name: &String) -> Result<(), Error> {

        let input_file = File::open(&file_name)?;
        let mut buf_reader = BufReader::new(input_file);
        
        self.k = BitGroup::read_usize(&mut buf_reader).unwrap();
        self.b = BitGroup::read_usize(&mut buf_reader).unwrap();
        self.w = BitGroup::read_usize(&mut buf_reader).unwrap();
        self.segment_size = BitGroup::read_usize(&mut buf_reader).unwrap();
        self.bit_groups = Vec::new();
        let mut X: Vec<u32> = Vec::new();

        loop {
            let mut bg_size = BitGroup::read_usize(&mut buf_reader).unwrap();
            if bg_size == 0 {
                break;
            }
            let mut bg: Vec<u32> = Vec::new();
            while bg_size != 0 {
                let data = buf_reader.read_u32::<BigEndian>().unwrap();
                bg.push(data);
                X.push(data);
                bg_size -= 1;
            }
            self.bit_groups.push(bg);
        }
        self.bit_group_box = X.into_boxed_slice();
        Ok(())
    }

    pub fn write_file(&self, file_name: &String) -> Result<(), Error> {

        let output_file = File::create(&file_name)?;
        let mut buf_writer = BufWriter::new(output_file);

        BitGroup::write_usize(&self.k, &mut buf_writer)?;
        BitGroup::write_usize(&self.b, &mut buf_writer)?;
        BitGroup::write_usize(&self.w, &mut buf_writer)?;
        BitGroup::write_usize(&self.segment_size, &mut buf_writer)?;

        for i in 0..self.bit_groups.len() {
            BitGroup::write_usize(&self.bit_groups[i].len(), &mut buf_writer)?;
            for j in 0..self.bit_groups[i].len() {
                buf_writer.write_u32::<BigEndian>(self.bit_groups[i][j])?;
            }
        }
        Ok(())
    }

}
