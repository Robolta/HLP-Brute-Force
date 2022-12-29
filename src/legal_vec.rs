use crate::constants::{STATES, TARGET};

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