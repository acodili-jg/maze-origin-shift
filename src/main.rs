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
            let ch = format_maze_node(it.get(x, y).expect("node in bounds"));
            write!(out, "{ch}")?;
        }
        writeln!(out)?;
    }
    Ok(())
}

#[inline]
const fn format_maze_node(it: MazeNode) -> &'static str {
    match it.direction() {
        None => " · ",
        Some(Direction::Left) => " ← ",
        Some(Direction::Up) => " ↑ ",
        Some(Direction::Right) => " → ",
        Some(Direction::Down) => " ↓ ",
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MazeGraph<const W: usize, const H: usize = W> {
    data: [[MazeNode; W]; H],
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

impl<const W: usize, const H: usize> MazeGraph<W, H> {
    #[inline]
    pub fn get(self, x: usize, y: usize) -> Option<MazeNode> {
        self.data.get(y)?.get(x).copied()
    }
}

impl MazeNode {
    #[inline]
    pub const fn direction(self) -> Option<Direction> {
        self.direction
    }
}

impl<const W: usize, const H: usize> Default for MazeGraph<W, H> {
    #[inline]
    fn default() -> Self {
        let mut row = [Direction::Left.into(); W];
        row[0] = Direction::Up.into();

        let mut data = [row; H];
        data[0][0] = MazeNode::default();

        Self { data }
    }
}

impl From<Direction> for MazeNode {
    #[inline]
    fn from(direction: Direction) -> Self {
        Some(direction).into()
    }
}

impl From<Option<Direction>> for MazeNode {
    #[inline]
    fn from(direction: Option<Direction>) -> Self {
        Self { direction }
    }
}
