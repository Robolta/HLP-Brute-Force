use crate::constants::*;
use crate::layers::*;
use std::collections::HashSet;

/*
Generates a vector of valid layers to follow each layer in unique
key: layer ([i16; STATES as usize]), value: unique indexes, index in current table (Vec<[usize; 2]>)*/
pub fn generate_pairs (unique: &Vec<[i16; STATES as usize]>) -> Vec<Vec<[usize; 2]>> { // Consider some form of handling for layers which have no valid layer after
    let mut target_groups: Vec<i16> = Vec::new();
    for i in TARGET {
        if !target_groups.contains(&(i as i16)) {
            target_groups.push(i as i16);
        }
    }
    let target_groups = target_groups.len();

    //let mut used: Vec<[i16; STATES as usize]> = Vec::new();
    let mut used: HashSet<[i16; STATES as usize]> = HashSet::new();
    let mut identity: [i16; STATES as usize] = [0; STATES as usize];
    for i in 0..STATES {
        identity[i as usize] = i;
    }
    used.insert(identity);
    //used.push(identity); // Don't include identity outputs

    for i in unique { // Don't include cases which give the same output as single layers
        //used.push(*i);
        used.insert(*i);
    }

    let mut pairs: Vec<Vec<[usize; 2]>> = Vec::new();
    for first in unique.iter() {

        let mut cpairs: Vec<[usize; 2]> = Vec::new();
        for (index, second) in unique.iter().enumerate() {
            
            let pass: [i16; STATES as usize] = through(*second, *first);

            if groups(pass) >= target_groups && !used.contains(&pass) {
                cpairs.push([index, cpairs.len()]); // [Unique Index, Pairs Element Index]
                //used.push(pass);
                used.insert(pass);
            }
        }

        pairs.push(cpairs);
    }

    return pairs;
}