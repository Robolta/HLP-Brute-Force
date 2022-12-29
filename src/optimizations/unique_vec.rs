use crate::constants::*;
use crate::comp::*;
use std::cmp::max;

// Generates the vector of unique layers
pub fn generate_unique () -> Vec<[i16; STATES as usize]> {
    let mut groups: Vec<i16> = Vec::new();
    for i in TARGET {
        if !groups.contains(&(i as i16)) {
            groups.push(i as i16);
        }
    }

    let mut outputs: Vec<[i16; STATES as usize]> = Vec::new();
    for i in 0..(4 * STATES.pow(2)) {

        let ma = i / (2 * STATES.pow(2)) == 1;
        let mb = (i / STATES.pow(2)) % 2 == 1;
        let va = (i / STATES) % STATES;
        let vb = i % STATES;

        let mut current: [i16; STATES as usize] = [0; STATES as usize];
        let mut groups2: Vec<i16> = Vec::new();

        for j in 0..STATES {
            // A single input-output pair passing through a layer (The maximum of the two comparators)
            current[j as usize] = max(comparator(j, va, ma), comparator(vb, j, mb));
            if !groups2.contains(&current[j as usize]) {
                groups2.push(current[j as usize]);
            }
        }

        if !outputs.contains(&current) &&  groups2.len() >= groups.len() {
            outputs.push(current);
        }
    }

    outputs.remove(0);

    return outputs;
}