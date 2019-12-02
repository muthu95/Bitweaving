use std::cmp;
use std::mem;
use bit_vec::BitVec;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
//use std::collections::BitVec;
use super::BitGroup;

//pub fn scanBetween (input: Vec<Vec<u32>>, C1: u64, C2: u64) -> BitVec {
pub unsafe fn scan_between (input_bit_group : BitGroup, c1: u32, c2: u32) -> BitVec {
    // number of words per segment
    let k:usize =  input_bit_group.k; 

    // number of words per group
    let b:usize = input_bit_group.b;
    let segment_size:usize = input_bit_group.segment_size;
    let input: Vec<Vec<u32>> = input_bit_group.bit_groups;

    let mut result_bv = BitVec::new();

    let all_zeros = _mm_set1_epi32(0);
    let all_ones = _mm_set1_epi32(!0);

    let mut c1_vec: Vec<__m128i> = vec![all_zeros; 32];
    let mut c2_vec: Vec<__m128i> = vec![all_zeros; 32];

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
            if _mm_movemask_epi8(_mm_cmpeq_epi32(big_meq1, all_zeros)) == 0xFFFF && 
                _mm_movemask_epi8(_mm_cmpeq_epi32(big_meq2, all_zeros)) == 0xFFFF {
                break;
            }
            let start = s * b;
            let end = cmp::min(s * b + b, s * b + k);
            for i in start..end {
                //Condition to avoid overflow
                if (i + (3*b)) >= input[g].len() {
                    break;
                }
                
                let inp = _mm_set_epi32(input[g][i] as i32, input[g][i + b] as i32, input[g][i + (2*b)] as i32, input[g][i + (3*b)] as i32);
                let c1i = c1_vec[index];
                let c2i = c2_vec[index];
                
                big_mgt = _mm_or_si128(big_mgt,_mm_and_si128(big_meq1, _mm_and_si128(_mm_xor_si128(c1i, all_ones), inp)));
                big_mlt = _mm_or_si128(big_mlt,_mm_and_si128(big_meq2, _mm_and_si128(c2i, _mm_xor_si128(inp, all_ones))));
                big_meq1 = _mm_and_si128(big_meq1, _mm_xor_si128(_mm_xor_si128(inp, c1i), all_ones));
                big_meq2 = _mm_and_si128(big_meq2, _mm_xor_si128(_mm_xor_si128(inp, c2i), all_ones));
                index = index + 1;
            }
        }
        let m_result = _mm_and_si128(big_mgt, big_mlt);
        let unpacked: [u32; 4] = mem::transmute(m_result);
        result_bv.append(&mut BitVec::from_bytes(&unpacked[3].to_be_bytes()));
        result_bv.append(&mut BitVec::from_bytes(&unpacked[2].to_be_bytes()));
        result_bv.append(&mut BitVec::from_bytes(&unpacked[1].to_be_bytes()));
        result_bv.append(&mut BitVec::from_bytes(&unpacked[0].to_be_bytes()));

        s += 4;
    }
    //println!("{:?}", result_bv);
    return result_bv;
}
