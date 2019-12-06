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
pub fn scan_between (input_bit_group : BitGroup, num:u32, c1: u32, c2: u32) -> Box<[u32]> {
    // number of words per segment
    let k:usize =  input_bit_group.k;

    // number of words per group
    let b:usize = input_bit_group.b;
    let segment_size:usize = input_bit_group.segment_size;
    let input_arr = input_bit_group.bit_group_box;
    let input = input_bit_group.bit_groups;

    let mut filter_bv = BitVec::new();

    let mut result_size = num / 32;
    let mut result_vec = Vec::with_capacity(result_size as usize);

    for i in 0..result_size {
        result_vec.push(0);
    }
    let mut result_arr = result_vec.into_boxed_slice();

    println!("result arr size: {}", result_arr.len());
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

    let mut input_index = 0;
    let mut result_index = 0;
    let h = b * input_bit_group.segment_size;


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
            end = cmp::min(start + b, start + k);
            
            let offset = g * h;
            for i in start..end {
                

                input_index = offset + i;
                mgt = mgt | (meq1 & (!c1_arr[index]) & input_arr[input_index]);
                mlt = mlt | (meq2 & (c2_arr[index]) & (!input_arr[input_index]));
                meq1 = meq1 & !(input_arr[input_index] ^ c1_arr[index]);
                meq2 = meq2 & !(input_arr[input_index] ^ c2_arr[index]);
                
                /*mgt = mgt | (meq1 & (!c1_arr[index]) & input[g][i]);
                mlt = mlt | (meq2 & (c2_arr[index]) & (!input[g][i]));
                meq1 = meq1 & !(input[g][i] ^ c1_arr[index]);
                meq2 = meq2 & !(input[g][i] ^ c2_arr[index]);*/
                index = index + 1;
            }
        }

        result_arr[result_index] = result_arr[result_index] | (mgt & mlt);
        result_index = result_index + 1;
        //let mut m_result:u32 = mgt & mlt;
        //result_bv.append(&mut BitVec::from_bytes(&m_result.to_be_bytes()));
        
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
    //println!("{:?}", result_bv);
    return result_arr;
}