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
pub fn scan_between (input_bit_group : BitGroup, c1: u32, c2: u32) -> BitVec {
    // number of words per segment
    let k:usize =  input_bit_group.k;

    // number of words per group
    let b:usize = input_bit_group.b;
    let segment_size:usize = input_bit_group.segment_size;
    let input: Vec<Vec<u32>> = input_bit_group.bit_groups;

    let mut filter_bv = BitVec::new();
    let mut result_bv = BitVec::new();

    //let mut c1_vec: Vec<u32> = vec![0; 32];
    //let mut c2_vec: Vec<u32> = vec![0; 32];

    let mut c1_arr = [0;32];
    let mut c2_arr = [0;32];

    for i in 0..k {
       if (c1 & (1 << (i))) > 0 {
           c1_arr[k - i - 1] =  !(0);
       } else {
           c1_arr[k - i - 1] = 0;
       }
    }

    for i in 0..k {
       if (c2 & (1 << (i))) > 0 {
           c2_arr[k - i - 1] = !(0);
       } else {
           c2_arr[k - i - 1] = 0;
       }
    }

    let mut k_b = k/b;
    let mut mlt = 0;
    let mut mgt = 0;
    let mut meq1 = 0;
    let mut meq2 = 0;
    let mut start = 0;
    let mut end = 0;
    let mut index = 0;

    for s in 0..segment_size {
        
        mlt = 0;
        mgt = 0;
        meq1 = !(0);
        meq2 = !(0);

        index = 0;
        for g in 0..k_b {

            if meq1 == 0 && meq2 == 0 {
                break;
            }

            start = s * b;
            end = cmp::min(s * b + b, s * b + k);
            
            for i in start..end {

                mgt = mgt | (meq1 & (!c1_arr[index]) & input[g][i]);
                mlt = mlt | (meq2 & (c2_arr[index]) & (!input[g][i]));
                meq1 = meq1 & !(input[g][i] ^ c1_arr[index]);
                meq2 = meq2 & !(input[g][i] ^ c2_arr[index]);
                index = index + 1;
            }
        }
        let mut m_result:u32 = mgt & mlt;
        result_bv.append(&mut BitVec::from_bytes(&m_result.to_be_bytes()));
        
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
    //println!("{:?}", resultBv);
    return result_bv;
}