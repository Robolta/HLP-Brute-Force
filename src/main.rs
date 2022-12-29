use std::io::stdin;
use std::time::Instant;
use std::cmp::max;

mod constants;
mod optimizations;
mod comp;

use constants::*;
use optimizations::{legality::*, unique_vec::*, union_intersect::*};
use comp::*;

fn main() {

    let start = Instant::now();

    // Generate vector used to verify legality of intermediate outputs
    print!("Generating legality vector...");
    let legality: Vec<[usize; 2]> = generate_legal();
    println!("Done! ({} different pairs)", legality.len());

    // Generate vector of unique layers
    print!("Generating unique...");
    let unique: Vec<[i16; STATES as usize]> = generate_unique();
    let mcount = unique.len();
    println!("Done! ({} unique layers)", mcount);

    // Generate a 2D array of vectors which contains data on what function needs to come before each of the unique layers in order to reach the output
    print!("Generating union...");
    let endings = ending_layers(&mcount, &unique);
    let union = generate_union(&endings, &unique);
    println!("Done! ({:?} end layers)", endings.len());

    // Generate pairs of layers which can come one after another
    print!("Generating pairs...");

    let pairs: Vec<Vec<[usize; 2]>> = generate_pairs(&unique, mcount);

    let mut pair_length = 0;
    for p in &pairs {
        pair_length += p.len();
    }

    println!("Done! ({} pairs)", thousands(pair_length));
    println!("Precalculations done (Took {:?})\n", start.elapsed());

    let mut count: Vec<[usize; 2]> = Vec::new();
    count.push([0, 0]);

    let mut current: Vec<[i16; STATES as usize]> = Vec::new();
    current.push(unique[0]);

    let mut depth = 1;

    let start = Instant::now();

    let mut candidate: Vec<[usize; 2]> = Default::default();
    let mut next_layer = false; // true if Union-Intersection Optimization finds a candidate

    println!("Searching for {:?}...", TARGET);
    print!("Depth 1 (and 2)");
    
    loop {
        let pdepth = depth;
        (count, current, depth) = next(count, current, depth, &unique, mcount, &pairs, &legality, start);
        if DEBUG & 1 == 1 {
            println!("\t[TESTING] {:?}", current[depth - 1]);
        }

        if depth != pdepth {
            //println!("{}, {:?}", depth, count);
            if next_layer {
                break;
            } else {
                print!("Depth {} (and {})", depth, depth + 1);
            }
        }

        let output = current[depth - 1];
        if output == TARGET {
            candidate = Vec::new();
            for i in &count {
                candidate.push(*i);
            }
            next_layer = false;
            break;
        } else if !next_layer {
            next_layer = intersect_check(output, &union);
            if next_layer {
                for i in &count {
                    candidate.push(*i);
                }
            }
        }
    }

    println!(" Solution Found! ({:?})", start.elapsed());
    if next_layer {
        print_notation(find_next(&candidate, &unique, &endings).to_vec(), &unique);
        //println!("{:?} (n+1)", find_next(&candidate, &unique, &endings).to_vec());

        let x = stdin().read_line(&mut String::new()); // Pauses so the output is readable when run as a .exe
        println!("{:?}", x);
    } else {
        print_notation(candidate.to_vec(), &unique);
        //println!("{:?} (n)", candidate.to_vec());

        let x = stdin().read_line(&mut String::new()); // Pauses so the output is readable when run as a .exe
        println!("{:?}", x);
    }

}

fn thousands (n: usize) -> String {
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

fn find_next (count: &Vec<[usize; 2]>, unique: &Vec<[i16; STATES as usize]>, endings: &Vec<usize>) -> Vec<[usize; 2]> {
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

fn print_notation (count: Vec<[usize; 2]>, unique: &Vec<[i16; STATES as usize]>) {
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

/*
Iterates the current function once
mut count: Vec<[usize; 2]>      - A vector of index pairs.  First value is the layer's index within the unique vector, second is the layer's index within the previous layer's pair vector.
mut change: usize               - The index to be changed within count.
mut last: usize                 - The last used index within count.
mcount: usize                   - The number of layers in unique.
mut depth: usize                - The current search depth.
pairs: &Vec<Vec<[usize; 2]>>    - Stores which layers can follow each of the unique layers.
start: Instant                  - Used for timing.

The entire function contains a main loop which iterates until all changes have successfully resulted in a valid iteration.
The inner loop at the top handles changes which are then validated by the remaining code.
*/
fn iter8 (mut count: Vec<[usize; 2]>, mut change: usize, mut last: usize, mcount: usize, mut depth: usize, pairs: &Vec<Vec<[usize; 2]>>, start: Instant) -> (Vec<[usize; 2]>, usize, usize, usize) {
    loop {
        loop { // count has [i16; 2] where [unique index, pairs element index]
            if change == 0 {
                if count[0][0] == mcount - 1 { // Special case for input layer reaching maximum value
                    count[0] = [0, 0];
                    count.push([0, 0]);
                    count.push([0, 0]);
                    depth += 2;
                    last += 2;
                    println!("\t({:?})", start.elapsed());
                    break;
                } else { // Input layer iterate
                    count[0] = [count[0][0] + 1, count[0][1] + 1];
                    break;
                }
            } else if count[change][1] + 1 == pairs[count[change - 1][0] as usize].len() { // Check if layer would overflow
                change -= 1;
            } else {
                count[change] = pairs[count[change - 1][0] as usize][(count[change][1] + 1) as usize]; // This mess means it's iterating to the next element of the pair vector based on the previous element in count AND the current element in count
                break;
            }
        }

        let mut zero = false;

        for i in change + 1..last + 1 { // Set appropriate count elements to "0"
            let pair = &pairs[count[i - 1][0] as usize];
            if pair.len() == 0 { // If no valid layers to follow the previous one, go back and change the previous layer (consider some form of handling when creating pairs to avoid checking this condition in the first place)
                zero = true;
                change = i - 1;
                break;
            }
            count[i] = pair[0]; // Similar to incrementing a few lines back, but simply uses the first element in the vector based on the previous element in count
        }

        if !zero {
            break;
        }
    }

    return (count, change, last, depth);
}

/*
Returns the next valid function to check.

mut count: Vec<[usize; 2]>                  - A vector of index pairs.  First value is the layer's index within the unique vector, second is the layer's index within the previous layer's pair vector.
mut current: Vec<[i16; STATES as usize]>    - The immediate outputs of the current function.  Used to avoid recalculating the entire function whenever it is changed.
mut depth: usize                            - The current search depth.
unique: &Vec<[i16; STATES as usize]>        - The vector of unique layers for the TARGET.
mcount: usize                               - The number of layers in unique.
pairs: &Vec<Vec<[usize; 2]>>                - Stores which layers can follow each of the unique layers.
legality: &Vec<[usize; 2]>                  - A vector specifying what input-output pairs are illegal.
start: Instant                              - Used for timing.
*/
fn next (mut count: Vec<[usize; 2]>, mut current: Vec<[i16; STATES as usize]>, mut depth: usize, unique: &Vec<[i16; STATES as usize]>, mcount: usize, pairs: &Vec<Vec<[usize; 2]>>, legality: &Vec<[usize; 2]>, start: Instant) -> (Vec<[usize; 2]>, Vec<[i16; STATES as usize]>, usize) {
    let mut last = depth - 1;
    let mut change = last;
    let mut asc: [i16; STATES as usize] = [0i16; STATES as usize];
    for i in 1..STATES {
        asc[i as usize] = i;
    }

    loop { // Repeats until legal (doesn't check very end layer)
        
        (count, change, last, depth) = iter8(count, change, last, mcount, depth, &pairs, start); // Iterate

        for i in change..last + 1 { // Iterate over all indexes affected by the change
            
            // Update the layers for the affected indexes
            if i == 0 {
                current[i] = through(unique[count[i][0] as usize], asc);
            } else if i > current.len() - 1 {
                current.push(through(unique[count[i][0] as usize], current[i - 1]));
            } else {
                current[i] = through(unique[count[i][0] as usize], current[i - 1]);
            }
            
            if i == last { // If the last index is reached without any illegal intermediate outputs
                return (count, current, depth); // Return the next valid states
            } else if !legal(current[i], legality) { // If an index isn't the last and isn't legal
                change = i; // Set the change index to i
                if DEBUG & 2 == 2 {
                    println!("\t\t[CHANGING] {:?}", count);
                }

                break; // Start from the beginning
            }
        }
    }
}

/*
Generates a vector of valid layers to follow each layer in unique
key: layer ([i16; STATES as usize]), value: unique indexes, index in current table (Vec<[usize; 2]>)*/
fn generate_pairs (unique: &Vec<[i16; STATES as usize]>, mcount: usize) -> Vec<Vec<[usize; 2]>> { // Consider some form of handling for layers which have no valid layer after
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

        let first = unique[i];
        //let mut cpairs: Vec<[i16; STATES as usize]> = Vec::new();
        let mut cpairs: Vec<[usize; 2]> = Vec::new();
        for j in 0..mcount {
            
            let second = unique[j];
            let pass = through(second, first);

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

// Passes the input through the provided layer
fn through (layer: [i16; STATES as usize], input: [i16; STATES as usize]) -> [i16; STATES as usize] {
    let mut output = [0; STATES as usize];

    for i in 0..STATES {
        output[i as usize] = layer[input[i as usize] as usize];
    }

    return output;
}