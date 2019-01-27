/*

this mirror maze consists of an array of values in an m x n matrix

0 0 / 0 \ 0
0 0 / 0 0 0
0 0 0 0 0 0
0 E 0 0 / 0
0 0 0 0 0 0

given such a matrix, return the path on its way to the exit or an error if there is a cyclic loop



*/

// use bloom::{ASMS,BloomFilter};
// use std::collections::HashSet;
use std::collections::HashMap;

enum Direction {
    North,
    South,
    East,
    West,
}
// type for a room's contents
enum RoomContents {
    DESC, // \
    ASC, // /
    EMPTY // 0
}

// marker struct for a marker
struct Marker {
    m: usize,
    n: usize,
    content: RoomContents,
}

struct BreadCrumb {
    m: usize,
    n: usize,
    direction: Direction,
}

// maze
struct Maze {
    layout: Vec<Vec<Marker>>,
    start_position: (usize, usize),
    start_direction: Direction
}

struct Cursor {
    path: HashMap<(usize, usize), BreadCrumb>,
    current_node: Marker,
    maze: Maze,
}

fn main() {
    println!("Hello, world!");

}
