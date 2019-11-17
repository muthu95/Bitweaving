use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::cmp;
use bit_vec::BitVec;
//use std::collections::BitVec;

//pub fn scanBetween (input: Vec<Vec<u32>>, C1: u64, C2: u64) -> BitVec {
pub fn scanBetween (C1: u64, C2: u64) -> BitVec {
    // number of words per segment
    let k:usize =  32;

    // number of words per group
    let B:usize = 8;
    let number_of_segments:usize = 4;
    let segment_size = 3; // dummy

    let mut filterBv = BitVec::new();
    let mut resultBv = BitVec::new();

    let mut C1Vec: Vec<u32> = Vec::new();
    let mut C2Vec: Vec<u32> = Vec::new();

    for i in 0..k {
       if (C1 & (1 << (i + 1))) == 1 {
           C1Vec.push(!(0));
       } else {
           C1Vec.push(0);
       }
    }

    for i in 0..k {
       if (C2 & (1 << (i + 1))) == 1 {
           C2Vec.push(!(0));
       } else {
           C2Vec.push(0);
       }
    }

    let mut kB = k/B;

    println!("Inside scanbetween");
    for s in 0..number_of_segments {
        
        let mut mlt = 0;
        let mut mgt = !(0);
        let mut meq1 = 1;
        let mut meq2 = 0;

        println!("s {}", s);
        for g in 0..kB {

            println!("g {}", g);
            if meq1 == 0 && meq2 == 0 {
                break;
            }

            let mut start = g * B;
            let mut end = cmp::min(g * B + B, k);

            for i in start..end {
                println!("word number: {}", i);
                /*
                mgt = mgt | (meq1 & (!C1Vec[i]) & input[s][i]);
                mlt = mlt | (meq2 & (!C2Vec[i]) & !input[s][i]);
                meq1 = meq1 & !(input[s][i] ^ C1Vec[i]);
                meq2 = meq2 & !(input[s][i] ^ C2Vec[i]);
                */
            }
        }
        let mut mResult:u32 = mgt & mlt;
        resultBv.append(&mut BitVec::from_bytes(&mResult.to_be_bytes()));
    }


    return resultBv;
}