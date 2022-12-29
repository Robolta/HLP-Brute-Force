use crate::constants::*;
use crate::layer::*;
use std::cmp::max;
use itertools::Itertools;

// Generates the vector of unique layers
pub fn generate_unique () -> Vec<[i16; STATES as usize]> {
    let mut target_groups: Vec<i16> = Vec::new();
    for i in TARGET {
        if !target_groups.contains(&(i as i16)) {
            target_groups.push(i as i16);
        }
    }

    let mut outputs: Vec<[i16; STATES as usize]> = Vec::new();
    for i in 0..(4 * STATES.pow(2)) {

        let mode_a: bool = i / (2 * STATES.pow(2)) == 1;
        let mode_b: bool = (i / STATES.pow(2)) % 2 == 1;
        let a: i16 = (i / STATES) % STATES;
        let b: i16 = i % STATES;

        let current: [i16; STATES as usize] = layer(a, b, mode_a, mode_b);

        if !outputs.contains(&current) &&  groups(current) >= target_groups.len() {
            outputs.push(current);
        }
    }

    outputs.remove(0);

    return outputs;
}

fn layer (a: i16, b: i16, mode_a: bool, mode_b: bool) -> [i16; STATES as usize] {
    
    let mut output: [i16; STATES as usize] = Default::default();
    for i in 0..STATES {
        let a_side: i16 = comparator(i, a, mode_a);
        let b_side: i16 = comparator(b, i, mode_b);
        output[i as usize] = max(a_side, b_side);
    }
    return output;
}

fn groups (output: [i16; STATES as usize]) -> usize {
    return output.into_iter().unique().collect_vec().len();
}