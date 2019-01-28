/*

this mirror maze consists of an array of values in an m x n matrix
n, m, E
0 0 / 0 \ 0
0 0 / 0 0 0
0 0 0 0 0 0
0 0 0 0 / 0
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
        println!("sizes: {:?} on {:?}", (n, m),  (self.n_size, self.m_size));
        if m >= self.m_size || n >= self.n_size {
            return Option::None;
        } else {
            return match self.layout.get(&(n, m)) {
                Option::Some(content) => Option::Some(Room{
                    m,
                    n,
                    content: *content
                }),
                Option::None => Option::Some(Room{
                    m, n, content: RoomContents::EMPTY,
                })
            }
        }
    }

    #[allow(dead_code)]
    fn new((n, m): (usize, usize), start_position: (usize, usize), start_direction: Direction, contents: Vec<Room>) -> Maze {
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
    fn next_coordinates(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::North => {
                if self.current_room.n == 0 {
                    return Option::None;
                } else {
                    return Option::Some((self.current_room.n - 1, self.current_room.m));
                }
            },
            Direction::South => {
                if self.current_room.n >= self.maze.n_size {
                    return Option::None;
                } else {
                    return Option::Some((self.current_room.n + 1, self.current_room.m));
                }
            },
            Direction::East  => {
                if self.current_room.m >= self.maze.m_size {
                    return Option::None;
                } else {
                    return Option::Some((self.current_room.n, self.current_room.m + 1));
                }
                
            },
            Direction::West  => {
                if self.current_room.m == 0 {
                    return Option::None;
                } else {
                    return Option::Some((self.current_room.n, self.current_room.m - 1));
                }
            }
        }
    }
    #[allow(dead_code)]
    fn out_of_bounds(&self, (n, m): (usize, usize)) -> bool {
        // let (n, m): (usize, usize) = next;
        println!("bounds: ({:?}) of ({:?})", (n, m), (self.maze.m_size, self.maze.n_size));
        if m >= self.maze.m_size || n >= self.maze.n_size {
            return true;
        } else {
            return false;
        }
    }

    #[allow(dead_code)]
    fn has_next(&self) -> bool {
        // returns true if the next square is outside the bounds of the room
        match self.next_coordinates() {
            Some(coordinates) => true,
            None => false
        }
    }

    #[allow(dead_code)]
    fn iterate(&mut self) -> &Cursor {
        match self.next_coordinates() {
            Some((n, m)) => {
                let maze = self.maze.clone();
                match maze.get_room(n, m) {
                    Option::None => {

                    },
                    Option::Some(room) => {
                        self.current_room = room;
                    }
                }
            },
            None => {

            }
        };
        self
    }

    #[allow(dead_code)]
    fn new(maze: &Maze) -> Cursor {
        let new_maze = Box::new(maze.clone());
        let (n, m) = new_maze.start_position;
        Cursor {
            cache: Box::new(HashSet::new()),
            path: Box::new(vec![]),
            current_room: new_maze.get_room(n, m).unwrap_or(Room{
                m,
                n,
                content: RoomContents::EMPTY,
            }),
            direction: new_maze.start_direction,
            maze: new_maze.clone()
        }
    }

    fn adjust_heading(&self, direction: Direction, content: RoomContents) -> Direction {
        match (direction, content) {
            (direction, RoomContents::EMPTY) => direction,
            (Direction::North, RoomContents::ASC) => Direction::East,
            (Direction::South, RoomContents::ASC) => Direction::West,
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
        match self.next_coordinates() {
            Some((n, m)) => {
                // let maze = self.maze.clone();
                println!("next code: {:?}", (n, m));
                let room: Room = self.maze.get_room(n, m).unwrap();
                let direction: Direction = self.adjust_heading(self.direction, room.content);
                
                // set the current room in the path
                let bread_crumb: BreadCrumb = BreadCrumb{
                    n: self.current_room.n,
                    m: self.current_room.m,
                    direction: self.direction
                };
                // save it into the paths and cache
                println!("pushing breadcrumb: {:?}", bread_crumb);
                self.path.push(bread_crumb.clone());
                self.cache.insert(bread_crumb.clone());

                // update the state
                self.direction = direction;
                self.current_room = room;
            },
            None => {

            }
        };
    }

    #[allow(dead_code)]
    fn solve(&mut self) -> Option<(usize, Room)> {
        // iterates until we get to the last square, at which point we push the area ofexit and the number of steps. 
        // if there is a cycle, return None

        loop {
            println!("starting the loop");
            if self.check_for_cycle() {
                println!("found cycle");
                return Option::None;
            }
            if self.has_next() {
                println!("iterating loop");
                self.next();
            } else {
                println!("breaking loop");
                break;
            }
        }
        
        Some((self.path.len(), self.current_room))
    }  
}



fn main() {
    println!("Hello, world!");
    let mut rooms: Vec<Room> = vec![];

    /*
        3, 4 E
        0 0 0 0 0
        0 0 / 0 /
        0 0 0 0 0
        0 0 \ 0 0


    */

    rooms.push(
        Room{
        m: 2,
        n: 1,
        content: RoomContents::ASC,
    });
    rooms.push(
        Room{
        m: 4,
        n: 1,
        content: RoomContents::ASC,
    });
    rooms.push(
        Room{
        m: 2,
        n: 3,
        content: RoomContents::DESC,
    });

    let maze: Maze = Maze::new((4, 5), (3, 4), Direction::West, rooms);
    let mut cursor: Cursor = Cursor::new(&maze);
    match cursor.solve() {
        Some((steps, room)) => {
            println!("{} steps, {:?}", steps, room);
        },
        None => {
            println!("no solution");
        }
    }

}
