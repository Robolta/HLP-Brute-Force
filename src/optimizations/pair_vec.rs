use crate::constants::*;
use crate::layer::*;

/*
Generates a vector of valid layers to follow each layer in unique
key: layer ([i16; STATES as usize]), value: unique indexes, index in current table (Vec<[usize; 2]>)*/
pub fn generate_pairs (unique: &Vec<[i16; STATES as usize]>, mcount: usize) -> Vec<Vec<[usize; 2]>> { // Consider some form of handling for layers which have no valid layer after
    let mut groups: Vec<i16> = Vec::new();
    for i in TARGET {
        if !groups.contains(&(i as i16)) {
            groups.push(i as i16);
        }
    }

    let mut unique2: Vec<[i16; STATES as usize]> = Vec::new();
    let mut identity: [i16; STATES as usize] = [0; STATES as usize];
    for i in 0..STATES {
        identity[i as usize] = i;
    }
    unique2.push(identity); // Don't include identity outputs

    for i in unique { // Don't include cases which give the same output as single layers
        unique2.push(*i);
    }

    let mut pairs: Vec<Vec<[usize; 2]>> = Vec::new();
    for i in 0..mcount {

        let first: [i16; STATES as usize] = unique[i];
        //let mut cpairs: Vec<[i16; STATES as usize]> = Vec::new();
        let mut cpairs: Vec<[usize; 2]> = Vec::new();
        for j in 0..mcount {
            
            let second: [i16; STATES as usize] = unique[j];
            let pass: [i16; STATES as usize] = through(second, first);

            let mut groups2: Vec<i16> = Vec::new();
            for i in pass {
                if !groups2.contains(&(i as i16)) {
                    groups2.push(i as i16);
                }
            }

            if groups2.len() >= groups.len() && !unique2.contains(&pass) {
                cpairs.push([j, cpairs.len()]); // [Unique Index, Pairs Element Index]
                unique2.push(pass);
            }
        }

        pairs.push(cpairs);
    }

    return pairs;
}