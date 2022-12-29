use crate::constants::*;
use bit_vec::BitVec;

pub fn ending_layers (mcount: &usize, unique: &Vec<[i16; STATES as usize]>) -> Vec<usize> {
    let mut target_outputs = Vec::new();
    for i in 0..STATES {
        if TARGET.contains(&i) {
            target_outputs.push(i);
        }
    }

    let mut endings = Vec::new();
    for i in 0..*mcount {
        endings.push(i);
        for j in &target_outputs {
            if !unique[i].contains(&j) {
                target_outputs.pop();
                break;
            }
        }
    }

    return endings;
}

/*
Generates a union for all unique layers which is used to check if a given function can reach the target solution with one additional layer
*/
pub fn generate_union (endings: &Vec<usize>, unique: &Vec<[i16; STATES as usize]>) -> [[BitVec; STATES as usize]; STATES as usize] {
    let mut union: [[BitVec; STATES as usize]; STATES as usize] = Default::default();

    for input in 0..STATES as usize {
        for output in 0..STATES as usize {
            for e in endings {
                union[input][output].push(unique[*e][output] == TARGET[input]);
            }
        }
    }

    return union;
}

pub fn intersect_check (output: [i16; STATES as usize], union: &[[BitVec; STATES as usize]; STATES as usize]) -> bool {
    let mut inter: BitVec = BitVec::from_elem(union[0][0].len(), true);

    for i in 0..STATES as usize{
        inter.and(&union[i][output[i] as usize]);
    }
    
    return !inter.none();
}