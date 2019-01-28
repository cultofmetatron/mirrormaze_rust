/*

this mirror maze consists of an array of values in an m x n matrix

0 0 / 0 \ 0
0 0 / 0 0 0
0 0 0 0 0 0
0 E 0 0 / 0
0 0 0 0 0 0

(0, 0) ----------> (0, m)
        \
             \
(n, 0) ---------> (n, m)

given such a matrix, return the path on its way to the exit or an error if there is a cyclic loop



*/

// use bloom::{ASMS,BloomFilter};
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

// type for a room's contents
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum RoomContents {
    DESC, // \
    ASC, // /
    EMPTY // 0
}

// marker struct for a marker
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Room {
    m: usize,
    n: usize,
    content: RoomContents,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct BreadCrumb {
    m: usize,
    n: usize,
    direction: Direction,
}

impl Hash for BreadCrumb {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.m.hash(state);
        self.n.hash(state);
        self.direction.hash(state);
    }
}

impl PartialEq for BreadCrumb {
    fn eq(&self, other: &BreadCrumb) -> bool {
        self.m == other.m && self.n == other.n && self.direction == other.direction
    }
}
impl Eq for BreadCrumb {}





// maze
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Maze {
    m_size: usize,
    n_size: usize,
    layout: Box<HashMap<(usize, usize), RoomContents>>,
    start_position: (usize, usize),
    start_direction: Direction
}

/*
  returns option for a maze
*/
impl Maze {

    #[allow(dead_code)]
    fn get_room(&self, n: usize, m: usize) -> Option<Room> {
        if m >= self.m_size || n >= self.n_size {
            return Option::None;
        } else {
            return match self.layout.get(&(n, m)) {
                Option::Some(content) => Option::Some(Room{
                    m,
                    n,
                    content: *content
                }),
                Option::None => None
            }
        }
    }

    #[allow(dead_code)]
    fn new((m, n): (usize, usize), start_position: (usize, usize), start_direction: Direction, contents: Vec<Room>) -> Maze {
        // setthe size of the room
        let mut layout : HashMap<(usize, usize), RoomContents> = HashMap::new();
        for room in contents.iter() {
            layout.insert((room.n, room.m), room.content);
        };
        // Err(String::from("invalid"))
        Maze{
            m_size: m,
            n_size: n,
            layout: Box::new(layout),
            start_direction,
            start_position
        }
    }

}

#[allow(dead_code)]
struct Cursor {
    cache: Box<HashSet<BreadCrumb>>,
    path: Box<Vec<BreadCrumb>>,
    direction: Direction,
    current_room: Room,
    maze: Box<Maze>,
}

impl Cursor {

    #[allow(dead_code)]
    fn check_for_cycle(&self) -> bool {
        // check if the current room and direction already exsit in the cache
        self.cache.contains(&BreadCrumb{
            m: self.current_room.m,
            n: self.current_room.n,
            direction: self.direction
        })
    }
    #[allow(dead_code)]
    fn next_coordinates(&self) -> (usize, usize) {
        match self.direction {
            Direction::North => (self.current_room.n - 1, self.current_room.m),
            Direction::South => (self.current_room.n + 1, self.current_room.m),
            Direction::East  => (self.current_room.n, self.current_room.m + 1),
            Direction::West  => (self.current_room.n, self.current_room.m - 1)
        }
    }
    #[allow(dead_code)]
    fn out_of_bounds(&self, (n, m): (usize, usize)) -> bool {
        // let (n, m): (usize, usize) = next;
        if m >= self.maze.m_size || n >= self.maze.n_size {
            return true;
        } else {
            return false;
        }
    }

    #[allow(dead_code)]
    fn has_next(&self) -> bool {
        // returns true if the next square is outside the bounds of the room
        let next_coordinates = self.next_coordinates();
        self.out_of_bounds(next_coordinates)
    }

    #[allow(dead_code)]
    fn iterate(&mut self) -> &Cursor {
        if self.has_next() {
            let (n, m) = self.next_coordinates();
            let maze = self.maze.clone();
            match maze.get_room(n, m) {
                Option::None => {

                },
                Option::Some(room) => {
                    self.current_room = room;
                }
            }
        };
        self
    }

    #[allow(dead_code)]
    fn new(maze: &Maze, direction: Direction, (n, m): (usize, usize)) -> Cursor {
        let new_maze = Box::new(maze.clone());
        Cursor {
            cache: Box::new(HashSet::new()),
            path: Box::new(vec![]),
            current_room: new_maze.get_room(n, m).unwrap().clone(),
            direction: direction,
            maze: new_maze.clone()
        }
    }

    fn adjust_heading(&self, direction: Direction, content: RoomContents) -> Direction {
        match (direction, content) {
            (direction, RoomContents::EMPTY) => direction,
            (Direction::North, RoomContents::ASC) => Direction::West,
            (Direction::South, RoomContents::ASC) => Direction::East,
            (Direction::East, RoomContents::ASC) => Direction::North,
            (Direction::West, RoomContents::ASC) => Direction::South,

            (Direction::North, RoomContents::DESC) => Direction::West,
            (Direction::South, RoomContents::DESC) => Direction::East,
            (Direction::East, RoomContents::DESC) => Direction::South,
            (Direction::West, RoomContents::DESC) => Direction::North
        }
    }

    // updates the system
    #[allow(dead_code)]
    fn next(&mut self) -> () {
        if self.has_next() {
            let (n, m) = self.next_coordinates();
            let room: Room = self.maze.get_room(n, m).unwrap();
            let direction: Direction = self.adjust_heading(self.direction, room.content);
            
            // set the current room in the path
            let bread_crumb: BreadCrumb = BreadCrumb{
                n: self.current_room.n,
                m: self.current_room.m,
                direction: self.direction
            };
            // save it into the paths and cache
            self.path.push(bread_crumb.clone());
            self.cache.insert(bread_crumb.clone());

            // update the state
            self.direction = direction;
            self.current_room = room;
        }
    }

    #[allow(dead_code)]
    fn solve(&mut self) -> Option<(usize, Room)> {
        // iterates until we get to the last square, at which point we push the area ofexit and the number of steps. 
        // if there is a cycle, return None

        loop {
            if self.check_for_cycle() {
                return Option::None;
            }
            if self.has_next() {
                self.next();
            } else {
                break;
            }
        }
        
        Some((self.path.len(), self.current_room))
    }  
}



fn main() {
    println!("Hello, world!");

}
