use std::cmp;
use std::mem;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
use super::BitGroup;

pub unsafe fn scan_between (input_bit_group : &BitGroup, c1: u32, c2: u32) -> Box<[u32]> {
    // number of words per segment
    let k:usize =  input_bit_group.k; 

    // number of words per group
    let b:usize = input_bit_group.b;

    let segment_size:usize = input_bit_group.segment_size;
    let input_arr = &input_bit_group.bit_group_box;

    let result_size = segment_size;
    let mut result_vec = Vec::with_capacity(result_size as usize);

    for i in 0..result_size {
        result_vec.push(0);
    }
    let mut result_arr = result_vec.into_boxed_slice();

    let all_zeros = _mm256_set1_epi32(0);
    let all_ones = _mm256_set1_epi32(!0);

    let mut c1_arr = [all_zeros; 32];
    let mut c2_arr = [all_zeros; 32];

    for i in 0..k {
       if (c1 & (1 << (i))) > 0 {
            c1_arr[k - i - 1] =  all_ones;
       } else {
            c1_arr[k - i - 1] = all_zeros;
       }
    }

    for i in 0..k {
       if (c2 & (1 << (i))) > 0 {
           c2_arr[k - i - 1] = all_ones;
       } else {
           c2_arr[k - i - 1] = all_zeros;
       }
    }

    let k_b = k/b;
    let mut s = 0;
    let mut big_mlt = all_zeros;
    let mut big_mgt = all_zeros;
    let mut big_meq1 = all_ones;
    let mut big_meq2 = all_ones;
    let mut index = 0;
    let mut result_index = 0;
    let h = b * input_bit_group.segment_size;
    let mut start = 0;
    let mut end = 0;
    let mut offset = 0;
    let mut m_result;
    let mut unpacked: [u32; 8];
    
    while s < segment_size {
        big_mlt = all_zeros;
        big_mgt = all_zeros;
        big_meq1 = all_ones;
        big_meq2 = all_ones;
        index = 0;
        for g in 0..k_b {

            if _mm256_movemask_epi8(_mm256_cmpeq_epi32(big_meq1, all_zeros)) == 0xFFFF && 
                _mm256_movemask_epi8(_mm256_cmpeq_epi32(big_meq2, all_zeros)) == 0xFFFF {
                break;
            }

            start = s * b;
            end = cmp::min(s * b + b, s * b + k);
            
            offset = g * h;
            for i in start..end {
                //Condition to avoid overflow
                if (offset + i + (7*b)) >= input_arr.len() {
                    break;
                }
                
                let inp = _mm256_set_epi32(input_arr[offset + i] as i32, input_arr[offset + i + b] as i32,
                    input_arr[offset + i + (2*b)] as i32, input_arr[offset + i + (3*b)] as i32,
                    input_arr[offset + i + (4*b)] as i32, input_arr[offset + i + (5*b)] as i32,
                    input_arr[offset + i + (6*b)] as i32, input_arr[offset + i + (7*b)] as i32);
                
                big_mgt = _mm256_or_si256(big_mgt,_mm256_and_si256(big_meq1, _mm256_and_si256(_mm256_xor_si256(c1_arr[index], all_ones), inp)));
                big_mlt = _mm256_or_si256(big_mlt,_mm256_and_si256(big_meq2, _mm256_and_si256(c2_arr[index], _mm256_xor_si256(inp, all_ones))));
                big_meq1 = _mm256_and_si256(big_meq1, _mm256_xor_si256(_mm256_xor_si256(inp, c1_arr[index]), all_ones));
                big_meq2 = _mm256_and_si256(big_meq2, _mm256_xor_si256(_mm256_xor_si256(inp, c2_arr[index]), all_ones));
                index = index + 1;
            }
        }
       
        m_result = _mm256_and_si256(big_mgt, big_mlt);
        unpacked = mem::transmute(m_result);
        result_arr[result_index] = result_arr[result_index] | unpacked[7];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[6];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[5];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[4];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[3];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[2];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[1];
        result_index = result_index + 1;
        result_arr[result_index] = result_arr[result_index] | unpacked[0];
        result_index = result_index + 1;
        
        s += 8;
    }
    //println!("Result: {:?}", result_arr);
    return result_arr;
}
