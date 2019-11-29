use std::cmp;
use std::mem;
use packed_simd::u32x4;
use bit_vec::BitVec;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
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

    let mut result_bv = BitVec::new();

    let all_zeros = u32x4::splat(0);
    let all_ones = u32x4::splat(!0);

    let mut c1_vec: Vec<u32x4> = vec![all_zeros; 32];
    let mut c2_vec: Vec<u32x4> = vec![all_zeros; 32];

    for i in 0..k {
       if (c1 & (1 << (i))) > 0 {
           c1_vec[k - i - 1] =  all_ones;
       } else {
           c1_vec[k - i - 1] = all_zeros;
       }
    }

    for i in 0..k {
       if (c2 & (1 << (i))) > 0 {
           c2_vec[k - i - 1] = all_ones;
       } else {
           c2_vec[k - i - 1] = all_zeros;
       }
    }

    let k_b = k/b;
    let mut s = 0;
    while s < segment_size {
        let mut big_mlt = all_zeros;
        let mut big_mgt = all_zeros;
        let mut big_meq1 = all_ones;
        let mut big_meq2 = all_ones;
        let mut index = 0;
        for g in 0..k_b {
            if big_meq1.eq(all_zeros).all() && big_meq2.eq(all_zeros).all() {
                break;
            }
            let start = s * b;
            let end = cmp::min(s * b + b, s * b + k);
            for i in start..end {
                //Condition to avoid overflow
                if (i + (3*b)) >= input[g].len() {
                    break;
                }
                let inp = u32x4::new(input[g][i], input[g][i + b], input[g][i + (2*b)], input[g][i + (3*b)]);
                let c1i = c1_vec[index];
                let c2i = c2_vec[index];
                
                big_mgt = big_mgt | (big_meq1 & (!c1i & inp));
                big_mlt = big_mlt | (big_meq2 & (c2i & !inp));
                big_meq1 = big_meq1 & !(inp ^ c1i);
                big_meq2 = big_meq2 & !(inp ^ c2i);
                index = index + 1;
            }
        }
        let m_result = big_mgt & big_mlt;
        //println!("{:?}", big_mgt);
        //println!("{:?}", big_mlt);
        // Need to find an optimized way
        result_bv.append(&mut BitVec::from_bytes(&m_result.extract(0).to_be_bytes()));
        result_bv.append(&mut BitVec::from_bytes(&m_result.extract(1).to_be_bytes()));
        result_bv.append(&mut BitVec::from_bytes(&m_result.extract(2).to_be_bytes()));
        result_bv.append(&mut BitVec::from_bytes(&m_result.extract(2).to_be_bytes()));
        s += 4;
    }
    println!("{:?}", result_bv);
    return result_bv;
}
