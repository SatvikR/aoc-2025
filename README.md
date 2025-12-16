# Advent Of Code 2025

Practicing some rust. Didn't bother with the most optimal solutions, just implemented whatever came to mind.

## Solutions

### Day 9

I am pretty much a computational geometry beginner. Part 1 I brute forced by checking every possible rectangle. Part 2, I used PIP on each the
corners of each candidate rectangle. If at least one of the corners was not in the polygon, skip that rectangle.
Then, check all the polygon edges that intersect the candidate rectangle. Luckily, the problem set does not have
any rectangles where two edges intersect from either end at the same x coordinate, which my code hinges on. If 
there are two edges that pass into your rectangle that are more than 1 space apart, then they create a gap in the 
rectangle so it also can be skipped.

### Day 8

Relatively simple graph problem. Create a min heap of all connections similar to what you'd do when computing
an MST. Maintain both an adjacency list and a reverse adjacency list to build up the connected components. For
part 1, exit after ingesting the top 10 items off the min heap. For part 2, ingest the entire min heap and keep
track of the extension chord distance of the most recent connection you've made.

### Day 7

Really wanted to use a bitset on this but rust moved it to an external crate and I didn't want to implement my own.
For part 1, just loop through each line, keeping track of where the beam is after each split. For part 2, do the same,
but keep track of how many predecessors each beam has. When a split happens, add the predecessors of the incoming beam
to each of the two beams it creates. The sum of the predecessors of all the beams on the last row is equivalent to the
"number of timelines".

### Day 6

This is purely a problem in parsing. For part 1, parse the input conventionally and compute the homework problems conventionally.
For part 2, you can reuse the computing routine from part 1, but reimplement the parser to parse numbers column wise instead of 
row-wise. You can use the spaces between operators on the last line to figure out how many numbers each homework problem has.

### Day 5

Part 1 is a for loop. Part 2 is more interesting, and ideally this is implemented using Rust's LinkedList on nightly builds
which has cursor functionality. Instead, I had to use a VecDeque because I don't have nightly builds installed. Part 2 reminded
me of a leetcode problem (I think called "merging intervals" or something like that) so basically just implemented that here. There's
definitely a more efficient way to solve part 2 since we don't actually need to maintain a list of all the intervals, so we can sort
the intervals first before merging them, which would make the merging much more simple and would not require as much memory.

### Day 4

Part 1 is a for loop that iterates over the board. Part 2 is just part 1 in a loop, removing removable nodes, and repeating until no changes 
are made.

### Day 3

This is a very good problem. Part 1 is just a matter of comparing all the possible largest 2 digit numbers going back to front to make it 
faster. Part 2 is full on DP, but a similar concept to part 1. Start from the back of the number, and maintain a list of the largest numbers
of each digit length as you iterate.

### Day 2

My solution here was a little messy, and I'd wager there are much faster ways to do this. For both part 1 and 2, my strategy was to iterate
over every possible repeated number (invalid ID in the context of the problem) in each range. For part 1, the algebra to compute the smallest
repeated number is quite easy, for part 2, I just guess a number and iterate from there. Lots of multiplying by 10 and adding, but nothing too
crazy.

### Day 1

Part 1 is just adding numbers and modulo'ing, incrementing a counter whenever your position is zero. Part 2 is the same structure, but you need to
do some math to figure out whether each move will cross zero or not (if the delta is greater than the distance to zero, and you're not currently
on zero, then you need to increment your counter).



