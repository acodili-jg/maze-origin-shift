use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    let maze = MazeGraph::<5>::default();
    writeln_maze(stdout(), &maze)?;
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
