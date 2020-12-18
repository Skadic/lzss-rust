use std::string::FromUtf8Error;

use crate::{augmented::AugmentedString, lzfactor::LZFactor};

#[allow(unused)]
pub fn reconstruct<T: IntoIterator<Item=LZFactor>>(iter: T) -> Result<String, FromUtf8Error> {
    let mut buffer = vec![];

    for factor in iter.into_iter() {
        match factor {
            LZFactor::Char(byte, _) => buffer.push(byte),
            LZFactor::Factor(pattern_start, pattern_length, _) => {
                for i in 0..pattern_length {
                    buffer.push(buffer[pattern_start + i]) 
                }
            }
        }
    }

    String::from_utf8(buffer)
}


pub fn lcp_len(s: &AugmentedString, index1: usize, index2: usize) -> usize {

    if index1 == s.string_len() || index2 == s.string_len() {
        return 0;
    }

    if index1 == index2 {
        return s.string_len() - index1 - 1;
    }

    let suffix_index_1 = s.suffix_array_index(index1);
    let suffix_index_2 = s.suffix_array_index(index2);

    let min_index = suffix_index_1.min(suffix_index_2) as isize;
    let max_index = suffix_index_1.max(suffix_index_2) as isize;

    let min = (min_index + 1..=max_index)
        .map(|i| s.lcp(i as usize))
        .fold(isize::MAX, isize::min);

    std::cmp::max(0, min as usize)
}

pub fn lz_factor(s: &AugmentedString, k: usize, prev_suffix_value: usize, next_suffix_value: usize) -> LZFactor {
    let lcp_len_lesser = lcp_len(s, k, prev_suffix_value);
    let lcp_len_greater = lcp_len(s, k, next_suffix_value);

    let factor_position;
    let factor_len;
    if lcp_len_lesser > lcp_len_greater {
        factor_position = prev_suffix_value;
        factor_len = lcp_len_lesser;
    } else {
        factor_position = next_suffix_value;
        factor_len = lcp_len_greater;
    }

    if factor_len == 0 {
        LZFactor::new_char(s.underlying().as_bytes()[k], k)
    } else {
        LZFactor::new_factor(factor_position, factor_len, k)
    }
}

pub fn factorize(s: String) -> Vec<LZFactor> {
    let augmented = AugmentedString::new(s, '\0');
    let n = augmented.string_len();
    let mut vec = vec![];

    let mut k = 0;
    while k < n {
        let suffix_array_index = augmented.suffix_array_index(k);
        let psv = augmented.suffix_index(augmented.psv(suffix_array_index));
        let nsv = augmented.suffix_index(augmented.nsv(suffix_array_index));
        let factor = lz_factor(&augmented, k, psv, nsv);
        k = factor.next_index();
        vec.push(factor);
    }

    // The last would be the sentinel character, and we don't want that to end up in the factorization
    vec.pop();

    vec
}

pub fn lz_factorized_string(s: String) -> String {
    factorize(s)
        .into_iter()
        .map(|factor| factor.to_string())
        .collect::<String>()
}
