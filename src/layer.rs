use crate::constants::*;
use std::cmp::max;

// Implementation of a Minecraft Redstone Comparator
pub fn comparator (back: i16, side: i16, mode: bool) -> i16 {
    if side > back {
        return 0;
    } else if mode {
        return back - side;
    } else {
        return back;
    }
}

// Passes the input through the provided layer
pub fn through (layer: [i16; STATES as usize], input: [i16; STATES as usize]) -> [i16; STATES as usize] {
    let mut output = [0; STATES as usize];

    for i in 0..STATES {
        output[i as usize] = layer[input[i as usize] as usize];
    }

    return output;
}

pub fn thousands (n: usize) -> String {
    let mut output = String::new();
    let mut count = n.to_string().len();
    for c in n.to_string().chars() {
        if count % 3 == 0 && count != n.to_string().len(){
            output += ",";
        }
        output += &c.to_string();
        count -= 1;
    }
    return output;
}

pub fn find_next (count: &Vec<[usize; 2]>, unique: &Vec<[i16; STATES as usize]>, endings: &Vec<usize>) -> Vec<[usize; 2]> {
    let mut current: [i16; STATES as usize] = [0; STATES as usize];
    for i in 0..STATES as usize {
        current[i] = i as i16;
    }

    for c in count.to_vec() {
        current = through(unique[c[0]], current);
    }

    let mut ncount = Vec::new();
    for c in count {
        ncount.push(*c);
    }

    for e in endings {
        if through(unique[*e], current) == TARGET {
            ncount.push([*e, *e]);
            break;
        }
    }
    return ncount;
}

pub fn print_notation (count: Vec<[usize; 2]>, unique: &Vec<[i16; STATES as usize]>) {
    let mut output = String::new();
    for c in count {
        for i in 0..(4 * STATES.pow(2)) {

            let ma = i / (2 * STATES.pow(2)) == 1;
            let mb = (i / STATES.pow(2)) % 2 == 1;
            let va = (i / STATES) % STATES;
            let vb = i % STATES;

            let mut current = [0i16; STATES as usize];

            for i in 0..STATES {
                current[i as usize] = max(comparator(i, va, ma), comparator(vb, i, mb));
            }

            if current == unique[c[0] as usize] {
                output += &if ma {"*"} else {""}.to_owned();
                output += &va.to_string();
                output += ",";
                output += &if mb {"*"} else {""}.to_owned();
                output += &vb.to_string();
                output += "; ";
                break;
            }
        }
    }
    println!("\n{}", output);
}