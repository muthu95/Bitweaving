use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::cmp;
use bit_vec::BitVec;


pub fn scan_between (arr: &[u32], C1: u32, C2: u32) -> BitVec {
    let mut resultBv = BitVec::new();

    for i in 0..arr.len() {
        if arr[i] > C1 && arr[i] < C2 {
            resultBv.push(true);
        } else {
            resultBv.push(false);
        }
    }

    return resultBv;
}