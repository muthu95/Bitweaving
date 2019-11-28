use std::cmp;
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

    for s in 0..segment_size {
        let mut mlt = 0;
        let mut mgt = 0;
        let mut meq1 = !(0);
        let mut meq2 = !(0);

        let mut index = 0;
        for g in 0..k_b {

            if meq1 == 0 && meq2 == 0 {
                break;
            }

            let start = s * b;
            let end = cmp::min(s * b + b, k);

            println!("start: {} to end: {}", start, end);
            let mut P = _mm256_set_epi32(input[g][start] as i32, input[g][start] as i32, 
                input[g][start] as i32, input[g][start] as i32, input[g][start] as i32,
                input[g][start] as i32, input[g][start] as i32, input[g][start] as i32);
            let mut P_C = _mm256_xor_si256(P, _mm256_set1_epi64x(-1));
            let mut b_meq1 = _mm256_set1_epi32(meq1);
            let mut b_meq2 = _mm256_set1_epi32(meq2);
            let mut b_mgt = _mm256_set1_epi32(mgt);
            let mut b_mlt = _mm256_set1_epi32(mlt);
            let U = _mm256_set1_epi32(c1_vec[index] as i32);
            let V = _mm256_set1_epi32(c2_vec[index] as i32);
            let U_C = _mm256_xor_si256(U, _mm256_set1_epi64x(-1));

            let mut C = _mm256_and_si256(U_C, P);
            C = _mm256_and_si256(b_meq1, C);
            C = _mm256_or_si256(b_mgt, C);
            //mask and reduce

            let mut D = _mm256_and_si256(V, P_C);
            D = _mm256_and_si256(b_meq2, D);
            D = _mm256_or_si256(b_mlt, D);
            //mask and reduce

            let mut X = _mm256_xor_si256(P, U);
            X = _mm256_xor_si256(X, _mm256_set1_epi64x(-1));
            X = _mm256_and_si256(b_meq1, X);

            let mut Y = _mm256_xor_si256(P, V);
            Y = _mm256_xor_si256(Y, _mm256_set1_epi64x(-1));
            Y = _mm256_and_si256(b_meq1, Y);

            /*for i in start..end {
                mgt = mgt | (meq1 & (!c1_vec[index]) & input[g][i]);
                mlt = mlt | (meq2 & (c2_vec[index]) & (!input[g][i]));
                meq1 = meq1 & !(input[g][i] ^ c1_vec[index]);
                meq2 = meq2 & !(input[g][i] ^ c2_vec[index]);
                index = index + 1;
            }*/
        }
        let m_result = mgt & mlt;
        result_bv.append(&mut BitVec::from_bytes(&m_result.to_be_bytes()));
        
        // TODO: For Testing purpose. Remove it in the final version
        /*let mut count = 0;
        for i in 0..result_bv.len() {
            if result_bv[i] == true {
                count += 1;
            }
        } 
        println!("count {}", count);
        */
    }
    println!("{:?}", result_bv);
    return result_bv;
}
