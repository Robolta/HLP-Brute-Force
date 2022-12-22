use std::io::stdin;
use std::time::Instant;
use std::cmp::max;

const STATES: i16 = 16;
const TARGET: [i16; STATES as usize] = [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0];//[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];
const DEBUG: u16 = 0; // 0, 1, 2, 3 are usable values currently

fn main() {

    let start = Instant::now();

    // Generate vector used to verify legality of intermediate outputs
    print!("Generating legality vector...");

    let mut legality: Vec<[usize; 2]> = Vec::new();
    for a in 0..STATES as usize {
        for b in (a + 1)..STATES as usize {
            if TARGET[a] != TARGET[b] {
                legality.push([a, b]);
            }
        }
    }
    
    println!("Done! ({} different pairs)", legality.len());

    // Generate vector of unique layers
    print!("Generating unique...");

    let mut unique: Vec<[i16; STATES as usize]> = generate_unique();
    unique.remove(0);

    let mcount = unique.len();

    println!("Done! ({} unique layers)", mcount);

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

    println!("Searching for {:?}...", TARGET);
    print!("Depth 1");
    
    loop {
        (count, current, depth) = next(count, current, depth, &unique, mcount, &pairs, &legality, start);
        if DEBUG & 1 == 1 {
            println!("\t[TESTING] {:?}", current[depth - 1]);
        }

        if current[depth - 1] == TARGET {
            break;
        }
    }

    println!(" Solution Found! ({:?})", start.elapsed());
    print_notation(count, unique);

    let x = stdin().read_line(&mut String::new()); // Pauses so the output is readable when run as a .exe
    println!("{:?}", x);

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

fn print_notation (count: Vec<[usize; 2]>, unique: Vec<[i16; STATES as usize]>) {
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

fn iter8 (mut count: Vec<[usize; 2]>, mut change: usize, mut last: usize, mcount: usize, mut depth: usize, pairs:&Vec<Vec<[usize; 2]>>, start: Instant) -> (Vec<[usize; 2]>, usize, usize, usize) {
    loop {
        loop { // count has [i16; 2] where [unique index, pairs element index]
            if change == 0 {
                if count[0][0] == mcount - 1 { // Special case for input layer reaching maximum value
                    count[0] = [0, 0];
                    count.push([0, 0]);
                    depth += 1;
                    last += 1;
                    println!("\t({:?})", start.elapsed());
                    print!("Depth {}", depth);
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
If any inputs are incorrectly mapped together, it is not legal.
This should also end up catching anything with less unique outputs than the target output.*/
fn legal (current: [i16; STATES as usize], legality: &Vec<[usize; 2]>) -> bool {
    for [a, b] in legality { // Uses existing vector for iteration rather than going over values which should be equal
        if current[*a] == current[*b] {
            return false;
        }
    }
    return true;
}

/*
Generates a union for all unique layers which is used to check if a given function can reach the target solution with one additional layer
*/
/*fn generate_union (unique: &Vec<[i16; STATES as usize]>, mcount: usize) -> [[Vec<u64>; STATES as usize]; STATES as usize] {
    let mut union: [[Vec<u64>; STATES as usize]; STATES as usize] = Default::default();

    for i in 0..mcount {
        if i % 64 == 0 {
            for input in 0..STATES as usize {
                for output in 0..STATES as usize {
                    union[input][output].push(0);
                }
            }
        }

        let layer: [i16; STATES as usize] = unique[i];
    }

    return union;
}*/

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

fn generate_unique () -> Vec<[i16; STATES as usize]> {
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
            current[j as usize] = max(comparator(j, va, ma), comparator(vb, j, mb));
            if !groups2.contains(&current[j as usize]) {
                groups2.push(current[j as usize]);
            }
        }

        if !outputs.contains(&current) &&  groups2.len() >= groups.len() {
            outputs.push(current);
        }
    }
    return outputs;
}

fn comparator (back: i16, side: i16, mode: bool) -> i16 {
    if side > back {
        return 0;
    } else if mode {
        return back - side;
    } else {
        return back;
    }
}

fn through (layer: [i16; STATES as usize], input: [i16; STATES as usize]) -> [i16; STATES as usize] {
    let mut output = [0; STATES as usize];

    for i in 0..STATES {
        output[i as usize] = layer[input[i as usize] as usize];
    }

    return output;
}