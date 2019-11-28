use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::cmp;
use bit_vec::BitVec;
//use std::collections::BitVec;
use super::BitGroup;

//pub fn scanBetween (input: Vec<Vec<u32>>, C1: u64, C2: u64) -> BitVec {
pub fn scan_between (input_bit_group : BitGroup, C1: u32, C2: u32) -> BitVec {
    // number of words per segment
    let k:usize =  input_bit_group.k;

    // number of words per group
    let b:usize = input_bit_group.b;
    let segment_size:usize = input_bit_group.segment_size;
    let input: Vec<Vec<u32>> = input_bit_group.bit_groups;

    let mut filterBv = BitVec::new();
    let mut resultBv = BitVec::new();

    let mut C1Vec: Vec<u32> = vec![0; 32];
    let mut C2Vec: Vec<u32> = vec![0; 32];

    for i in 0..k {
       if (C1 & (1 << (i))) > 0 {
           C1Vec[k - i - 1] =  !(0);
       } else {
           C1Vec[k - i - 1] = 0;
       }
    }

    for i in 0..k {
       if (C2 & (1 << (i))) > 0 {
           C2Vec[k - i - 1] = !(0);
       } else {
           C2Vec[k - i - 1] = 0;
       }
    }

    let mut kB = k/b;

    for s in 0..segment_size {
        
        let mut mlt = 0;
        let mut mgt = 0;
        let mut meq1 = !(0);
        let mut meq2 = !(0);

        let mut index = 0;
        for g in 0..kB {

            if meq1 == 0 && meq2 == 0 {
                break;
            }

            let mut start = s * b;
            let mut end = cmp::min(s * b + b, k);
            
            for i in start..end {
                mgt = mgt | (meq1 & (!C1Vec[index]) & input[g][i]);
                mlt = mlt | (meq2 & (C2Vec[index]) & (!input[g][i]));
                meq1 = meq1 & !(input[g][i] ^ C1Vec[index]);
                meq2 = meq2 & !(input[g][i] ^ C2Vec[index]);
                index = index + 1;
            }
        }
        let mut mResult:u32 = mgt & mlt;
        resultBv.append(&mut BitVec::from_bytes(&mResult.to_be_bytes()));
        
        // TODO: For Testing purpose. Remove it in the final version
        /*let mut count = 0;
        for i in 0..resultBv.len() {
            if resultBv[i] == true {
                count += 1;
            }
        } 
        println!("count {}", count);
        */
    }
    println!("{:?}", resultBv);
    return resultBv;
}