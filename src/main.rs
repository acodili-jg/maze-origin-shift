use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    let mut maze = MazeGraph::<5>::default();

    println!("Initial maze");
    writeln_maze(stdout(), &maze)?;

    for idx in 1..=100 {
        maze.move_origin(&mut rand::thread_rng());

        println!();
        println!("After iteration #{idx}");
        writeln_maze(stdout(), &maze)?;
    }

    Ok(())
}

fn writeln_maze<const W: usize, const H: usize, O: Write>(
    mut out: O,
    it: &MazeGraph<W, H>,
) -> io::Result<()> {
    for y in 0..H {
        for x in 0..W {
            #[allow(clippy::expect_used)]
            let ch = format_maze_node(it.get(x, y).expect("node in bounds"));
            write!(out, " {ch} ")?;
        }
        writeln!(out)?;
    }
    Ok(())
}

#[inline]
const fn format_maze_node(it: MazeNode) -> char {
    match it.direction() {
        None => '·',
        Some(Direction::Left) => '←',
        Some(Direction::Up) => '↑',
        Some(Direction::Right) => '→',
        Some(Direction::Down) => '↓',
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MazeGraph<const W: usize, const H: usize = W> {
    data: [[MazeNode; W]; H],
    origin: (usize, usize),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MazeNode {
    direction: Option<Direction>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    #[inline]
    pub const fn from_ordinal(ordinal: usize) -> Result<Self, usize> {
        Ok(match ordinal {
            0 => Self::Left,
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            ordinal => return Err(ordinal),
        })
    }
}

impl<const W: usize, const H: usize> MazeGraph<W, H> {
    #[inline]
    pub fn get(self, x: usize, y: usize) -> Option<MazeNode> {
        self.data.get(y)?.get(x).copied()
    }

    pub fn move_origin<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        fn gen_bounded_direction<const W: usize, const H: usize, R: Rng + ?Sized>(
            x: usize,
            y: usize,
            rng: &mut R,
        ) -> Direction {
            #[allow(overlapping_range_endpoints, clippy::match_same_arms)]
            match (x, y, rng.gen_range(0u8..12u8)) {
                (x, y, _) if x >= W || y >= H => unreachable!("origin should be in-bounds"),
                (_, _, 12..) => {
                    unreachable!("rng direction seed should be within [0, lcm(2, 3, 4))")
                }

                // Top-left corner
                (0, 0, 0..=5) => Direction::Right,
                (0, 0, 6..=11) => Direction::Down,

                // Top-right corner
                (x, 0, 0..=5) if x == W - 1 => Direction::Left,
                (x, 0, 6..=11) if x == W - 1 => Direction::Down,

                // Bottom-right corner
                (x, y, 0..=5) if x == W - 1 && y == H - 1 => Direction::Left,
                (x, y, 6..=11) if x == W - 1 && y == H - 1 => Direction::Up,

                // Botton-left corner
                (0, y, 0..=5) if y == H - 1 => Direction::Up,
                (0, y, 6..=11) if y == H - 1 => Direction::Right,

                // Left edge
                (0, _, 0..=3) => Direction::Up,
                (0, _, 4..=7) => Direction::Right,
                (0, _, 8..=11) => Direction::Down,

                // Top edge
                (_, 0, 0..=3) => Direction::Left,
                (_, 0, 4..=7) => Direction::Right,
                (_, 0, 8..=11) => Direction::Down,

                // Right edge
                (x, _, 0..=3) if x == W - 1 => Direction::Left,
                (x, _, 4..=7) if x == W - 1 => Direction::Up,
                (x, _, 8..=11) if x == W - 1 => Direction::Down,

                // Bottom edge
                (_, y, 0..=3) if y == H - 1 => Direction::Left,
                (_, y, 4..=7) if y == H - 1 => Direction::Up,
                (_, y, 8..=11) if y == H - 1 => Direction::Right,

                // Inside
                (_, _, 0..=2) => Direction::Left,
                (_, _, 3..=5) => Direction::Up,
                (_, _, 6..=8) => Direction::Right,
                (_, _, 9..=11) => Direction::Down,
            }
        }

        #[inline]
        const fn offset_towards(direction: Direction, x: usize, y: usize) -> (usize, usize) {
            match direction {
                Direction::Left => (x - 1, y),
                Direction::Up => (x, y - 1),
                Direction::Right => (x + 1, y),
                Direction::Down => (x, y + 1),
            }
        }

        let (x, y) = self.origin;
        let direction = gen_bounded_direction::<W, H, R>(x, y, rng);
        self.data[y][x].direction_mut().replace(direction);

        self.origin = offset_towards(direction, x, y);
        let (x, y) = self.origin;
        self.data[y][x].direction_mut().take();
    }
}

impl MazeNode {
    #[inline]
    pub const fn new(direction: Option<Direction>) -> Self {
        Self { direction }
    }

    #[inline]
    pub const fn new_origin() -> Self {
        Self::new(None)
    }

    #[inline]
    pub const fn new_towards(direction: Direction) -> Self {
        Self::new(Some(direction))
    }

    #[inline]
    pub const fn direction(self) -> Option<Direction> {
        self.direction
    }

    #[inline]
    pub fn direction_mut(&mut self) -> &mut Option<Direction> {
        &mut self.direction
    }
}

impl<const W: usize, const H: usize> Default for MazeGraph<W, H> {
    #[inline]
    fn default() -> Self {
        #[inline]
        const fn default_data<const W: usize, const H: usize>() -> [[MazeNode; W]; H] {
            #[inline]
            const fn default_row<const W: usize>() -> [MazeNode; W] {
                let mut row = [MazeNode::new_towards(Direction::Left); W];
                row[0] = MazeNode::new_towards(Direction::Up);
                row
            }

            let mut data = [default_row(); H];
            data[0][0] = MazeNode::new_origin();
            data
        }

        let data = default_data();
        let origin = Default::default();

        Self { data, origin }
    }
}

impl Distribution<Direction> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        #[allow(clippy::expect_used)]
        Direction::from_ordinal(rng.gen_range(0..4)).expect("ordinal is valid")
    }
}

impl From<Direction> for MazeNode {
    #[inline]
    fn from(direction: Direction) -> Self {
        Self::new_towards(direction)
    }
}

impl From<Option<Direction>> for MazeNode {
    #[inline]
    fn from(direction: Option<Direction>) -> Self {
        Self::new(direction)
    }
}
