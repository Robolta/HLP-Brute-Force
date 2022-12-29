use crate::constants::*;

pub fn generate_legal () -> Vec<[usize; 2]> {

    let mut legality: Vec<[usize; 2]> = Vec::new();
    for a in 0..STATES as usize {
        for b in (a + 1)..STATES as usize {
            if TARGET[a] != TARGET[b] {
                legality.push([a, b]);
            }
        }
    }

    return legality;
}

/*
If any inputs are incorrectly mapped together, it is not legal.
This should also end up catching anything with less unique outputs than the target output.*/
pub fn legal (current: [i16; STATES as usize], legality: &Vec<[usize; 2]>) -> bool {
    for [a, b] in legality { // Uses existing vector for iteration rather than going over values which should be equal
        if current[*a] == current[*b] {
            return false;
        }
    }
    return true;
}