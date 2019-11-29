use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::cmp;
use bit_vec::BitVec;


pub fn scan_between (arr: &[u32], c1: u32, c2: u32) -> BitVec {
    let mut result_bv = BitVec::new();

    for i in 0..arr.len() {
        if arr[i] > c1 && arr[i] < c2 {
            result_bv.push(true);
        } else {
            result_bv.push(false);
        }
    }

    return result_bv;
}