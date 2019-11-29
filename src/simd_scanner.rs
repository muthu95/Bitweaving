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
pub unsafe fn scan_between (input_bit_group : BitGroup, C1: u32, C2: u32) -> BitVec {
    // number of words per segment
    let k:usize =  input_bit_group.k; 

    // number of words per group
    let b:usize = input_bit_group.b;
    let segment_size:usize = input_bit_group.segment_size;
    let input: Vec<Vec<u32>> = input_bit_group.bit_groups;

    let mut result_bv = BitVec::new();

    let mut c1_vec: Vec<u32> = vec![0; 32];
    let mut c2_vec: Vec<u32> = vec![0; 32];

    for i in 0..k {
       if (C1 & (1 << (i))) > 0 {
           c1_vec[k - i - 1] =  !(0);
       } else {
           c1_vec[k - i - 1] = 0;
       }
    }

    for i in 0..k {
       if (C2 & (1 << (i))) > 0 {
           c2_vec[k - i - 1] = !(0);
       } else {
           c2_vec[k - i - 1] = 0;
       }
    }

    let k_b = k/b;
    let all_zeros = u32x4::splat(0);
    let mut s = 0;
    while s < segment_size {
        let mut big_mlt = u32x4::splat(0);
        let mut big_mgt = u32x4::splat(0);
        let mut big_meq1 = u32x4::splat(!0);
        let mut big_meq2 = u32x4::splat(!0);
        let mut index = 0;
        for g in 0..k_b {
            println!("@@@@@@@ {}", g);
            if big_meq1.eq(all_zeros).all() && big_meq2.eq(all_zeros).all() {
                break;
            }
            let start = s * b;
            let end = cmp::min(s * b + b, s * b + k);

            println!("start: {} to end: {}", start, end);
            for i in start..end {
                println!("$$$$$$$ {}", i);
                //Condition to avoid overflow
                if (i + (3*b)) >= input[g].len() {
                    break;
                }
                let inp = u32x4::new(input[g][i], input[g][i + b], input[g][i + (2*b)], input[g][i + (3*b)]);
                //c1i and c2i can be computed outside and reused.
                let c1i = u32x4::splat(c1_vec[index]);
                let c2i = u32x4::splat(c2_vec[index]);
                //println!("{:?}, {:?}, {:?}", inp, c1i, c2i);
                big_mgt = big_mgt | (big_meq1 & (!c1i & inp));
                //mgt = mgt | (meq1 & (!c1_vec[index]) & input[g][i]);
                big_mlt = big_mlt | (big_meq2 & (c2i & !inp));
                //mlt = mlt | (meq2 & (c2_vec[index]) & (!input[g][i]));
                big_meq1 = big_meq1 & !(inp ^ c1i);
                //meq1 = meq1 & !(input[g][i] ^ c1_vec[index]);
                big_meq2 = big_meq2 & !(inp ^ c2i);
                //meq2 = meq2 & !(input[g][i] ^ c2_vec[index]);
                index = index + 1;
            }
        }
        let m_result = big_mgt & big_mlt;
        //println!("{:?}", m_result);
        //println!("============");
        //Should convert to m_result to bytes. Runtime Error here
        //result_bv.append(&mut BitVec::from_bytes(m_result));

        // TODO: For Testing purpose. Remove it in the final version
        /*let mut count = 0;
        for i in 0..result_bv.len() {
            if result_bv[i] == true {
                count += 1;
            }
        } 
        println!("count {}", count);
        */
        
        s += 4;
    }
    println!("{:?}", result_bv);
    return result_bv;
}
