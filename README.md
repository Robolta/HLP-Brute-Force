# HLP Brute Force
An optimized brute force program created to find solutions for the Hex Layer Problem.

## Table of Contents

- The Hex Layer Problem
    - Short Explanation
    - Long Explanation
    - Layer Notation
- HLP Optimized Brute Force
    - Storing Layers
    - Unique Layers
        - Unique Functions
    - Intermediate Outputs
    - Group Check
    - Function Legality
    - Pairwise Iteration
    - Union-Intersection

## The Hex Layer Problem
*What is the Hex Layer Problem anyway?*

### Short Explanation

The short explanation is that the problem asks the following:

Given a target sequence, find the optimal (shortest) function of layers which reaches it.  
"Reaching" a target sequence just means treating the sequence as a function (index = input) would have the same funcitonality as the layer-composed function.

### Long Explanation

First, what is a layer?

A layer is a redstone circuit within the game Minecraft.  
It was first designed by Aminotreal and Powsi.
It has 1024 possible static states which affect the main input-output relationship.

![image](https://media.discordapp.net/attachments/721120731974598726/1017443702332993616/unknown.png)

Each of the barrels (brown) can produce a signal from 0 to 15.  
Each of the right two comparators (white) can be in either a "compare" or "subtract" state.

16 * 16 * 2 * 2 = 1024 states

We can reach more than 1024 possible functions by simply chaining together the inputs and outputs of multiple layers.

![image](https://user-images.githubusercontent.com/58904726/209972849-b7c6413a-4b86-40d7-a08e-642f22cd952b.png)

It has been proven that a subset of all layers can reach any arbitrary function (with 1 input and 1 output, both 0-15).  
This means that using the entire set of layers can certainly do the same.

So how does a layer work?  
Let's start with how a comparator works.  
Within my program, comparators are represented by the following function.

```rust
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
```

Compare mode means the back outputs unless the side is bigger, in which case the output is 0.  
Subtract mode means the side is subtracted from the back.  
The output can never be less than 0, so subtract mode is at least 0. (Negative subtractions would give 0)

After the two comparators produce their outputs, the larger of the two reaches the final output.

So, again, the problem boils down to finding a configuration of layers which uses as few as possible to reach a target function.  
[Sorting by Prefix Reversals is NP-Hard](https://arxiv.org/abs/1111.0434v1) and they are the only layers we have to sort while maintaining the full 16 output values.  
This means that our problem only proves to be doable optimally with brute force.

### Layer Notation

Currently the common notation is called "asterisk notation" and is relatively straightforward.  
The leftmost layer is always the comparator with a barrel as its side input.  
Sides are denoted with an asterisk if in subtract mode (nothing in compare mode) along with the value of the correspoding barrel.  
Sides are separated by a comma, layers are separated by a semicolon and go from input to output.  
Whitespace is ignored and using either base ten or base sixteen is entirely up to you.

Here are a few reference examples with their correspoding outputs.

`1, *2; *3, 4;`         = `[4, 4, 4, 4, 4, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]`  
`1, *F; C, 3; A, *F;`   = `[15, 14, 13, 12, 15, 15, 15, 15, 15, 15, 15, 15, 12, 13, 14, 15]`

## HLP Optimized Brute Force
*How exactly do you find optimal solutions?*

The number of possible functions grows exponentially with depth.  
Just going 3 layers deep means you've already got 1,073,741,824 possible functions.  
Currently our programs can reach around 6 layer deep relatively quickly.  
Here are the optimizations used to achieve that.

### Storing Layers

Likely the most trivial optimization: storing the outputs for each layer rather than calculating it each time.

### Unique Layers

Unique layers are how we refer to the set of layers where there is at most one layer for any given function.  
This set has 739 layers in it, meaning it's a very trivial reduction of our original 1024 layers.

#### Unique Functions

Along with unique layers, we can extend things to unique functions.  
Currently it's only really feasible to store 2 or 3 deep at most, given the space and time requirements to do any more.

### Intermediate Outputs

Storing outputs between each pair of layers avoids having to recompute them.  
It's also significant in the Legality optimization.

### Group Check

An intermediate output can't reach the target if it has less output values than the target does.  
This is a simple check that can be applied even on the unique layers, since it isn't dependant on location within the function.

### Function Legality

Functions which map two inputs to the same output which are different outputs in the target, are not valid as intermediate outputs.  
There is no way to separate the two values once they've reached the same value.  
This works along with the Intermediate Output optimization to do more than verify the final output (which generally isn't worth doing over simply checking if it's the solution).
This enables us to check intermediate outputs when they are first generated and easily skip large sections rather than simply iterating the very end.
Unfortunately, this optimization is dependant on starting from the input, meaning it cannot be applied to unique layers the same as Group Check.

### Pairwise Iteration

Each layer has a set of layers which can follow it, this is generally smaller than the set of all unique layers.  
Similar to Unique Layers, we can apply a Group Check but not Function Legality since their location isn't defined.

### Union-Intersection

A more complicated optimization where the idea is to check if the current function can reach the output in 1 more layer.  
This is done by pre-computing a 2D array of vectors where every possible input-output pair is represented.  
For a given input-output pair, the layers within the vector are ones which would bring the provided input the rest of the way to the correct output.  
The check is performed by using the current function's input-output pairs to get 16 corresponding vectors, then checking if the vectors share at least 1 common element.  
If this check succeeds, the check can be ignored for the rest of the depth.  
However, the current depth still needs to be searched entirely in order to avoid missing a potentially shorter solution.
