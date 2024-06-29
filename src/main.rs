use itertools::Itertools;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

fn main() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = WIDTH;
    let mut maze = MazeGraph::<WIDTH, HEIGHT>::default();

    println!("Initial maze\n{}", format_maze(&maze));

    for idx in 1..(WIDTH * HEIGHT * 10) {
        maze.move_origin(&mut rand::thread_rng());

        println!("After iteration #{idx}\n{}", format_maze(&maze));
    }
}

fn format_maze<const W: usize, const H: usize>(it: &MazeGraph<W, H>) -> String {
    #[allow(clippy::expect_used)]
    fn format_maze_nodes<const W: usize, const H: usize>(it: &MazeGraph<W, H>) -> [[char; W]; H] {
        let mut nodes = [[' '; W]; H];
        for (y, row) in nodes.iter_mut().enumerate() {
            for (x, node) in row.iter_mut().enumerate() {
                *node = format_maze_node(it.get(x, y).expect("node in bounds"));
            }
        }
        nodes
    }

    #[allow(clippy::expect_used)]
    fn collect_horizontal_edges<const W: usize, const H: usize>(
        it: &MazeGraph<W, H>,
    ) -> Vec<[bool; W]> {
        let mut horizontal_edges = vec![[true; W]; H + 1];
        for (y, edges) in horizontal_edges.iter_mut().enumerate().take(H).skip(1) {
            for (x, edge) in edges.iter_mut().enumerate() {
                if matches!(
                    it.get(x, y - 1).expect("node in bounds").direction(),
                    Some(Direction::Down)
                ) || matches!(
                    it.get(x, y).expect("node in bounds").direction(),
                    Some(Direction::Up)
                ) {
                    *edge = false;
                }
            }
        }
        horizontal_edges
    }

    #[allow(clippy::expect_used)]
    fn collect_vertical_edges<const W: usize, const H: usize>(
        it: &MazeGraph<W, H>,
    ) -> Vec<Vec<bool>> {
        let mut vertical_edges = vec![vec![true; W + 1]; H];
        for (y, edges) in vertical_edges.iter_mut().enumerate().take(H) {
            for (x1, x2) in (0..W).tuple_windows() {
                if matches!(
                    it.get(x1, y).expect("node in bounds").direction(),
                    Some(Direction::Right)
                ) || matches!(
                    it.get(x2, y).expect("node in bounds").direction(),
                    Some(Direction::Left)
                ) {
                    edges[x2] = false;
                }
            }
        }
        vertical_edges
    }

    fn collect_vertices<const W: usize, const H: usize>(
        horizontal_edges: &[[bool; W]],
        vertical_edges: &[Vec<bool>],
    ) -> Vec<Vec<(bool, bool, bool, bool)>> {
        let mut vertices = vec![vec![(false, false, false, false); W + 1]; H + 1];
        for (edges, row) in horizontal_edges.iter().zip(&mut vertices) {
            for (x, edge) in edges.iter().enumerate() {
                row[x].2 |= *edge;
                row[x + 1].0 |= *edge;
            }
        }
        for (y, edges) in vertical_edges.iter().enumerate() {
            for (x, edge) in edges.iter().enumerate() {
                vertices[y][x].3 |= *edge;
                vertices[y + 1][x].1 |= *edge;
            }
        }
        vertices
    }

    match (W, H) {
        (0, 0) => return String::from("┌┐\n└┘"),
        (w, 0) => {
            let wall = std::iter::repeat("───").take(w).join("─");
            return format!("┌{wall}┐\n└{wall}┘");
        }
        (0, h) => return format!("┌┐\n{}└┘", "││\n".repeat(h)),
        _ => {}
    }

    let nodes = format_maze_nodes(it);
    let horizontal_edges = collect_horizontal_edges(it);
    let vertical_edges = collect_vertical_edges(it);
    let vertices = collect_vertices::<W, H>(&horizontal_edges, &vertical_edges);

    std::iter::zip(vertices, horizontal_edges)
        .map(|(row1, row2)| {
            row1.into_iter()
                .map(format_vertex)
                .interleave(
                    row2.into_iter()
                        .map(|edge| if edge { "───" } else { "   " }),
                )
                .collect::<String>()
        })
        .interleave(std::iter::zip(vertical_edges, nodes).map(|(row1, row2)| {
            row1.into_iter()
                .map(|edge| if edge { '│' } else { ' ' })
                .interleave(row2)
                .join(" ")
        }))
        .join("\n")
}

#[inline]
const fn format_vertex((left, up, right, down): (bool, bool, bool, bool)) -> &'static str {
    match (left, up, right, down) {
        (false, false, false, false) => " ",
        (false, false, false, true) => "╷",
        (false, false, true, false) => "╶",
        (false, false, true, true) => "┌",
        (false, true, false, false) => "╵",
        (false, true, false, true) => "│",
        (false, true, true, false) => "└",
        (false, true, true, true) => "├",
        (true, false, false, false) => "╴",
        (true, false, false, true) => "┐",
        (true, false, true, false) => "─",
        (true, false, true, true) => "┬",
        (true, true, false, false) => "┘",
        (true, true, false, true) => "┤",
        (true, true, true, false) => "┴",
        (true, true, true, true) => "┼",
    }
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

    #[inline]
    pub fn move_origin<R: Rng + ?Sized>(&mut self, rng: &mut R) -> bool {
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

        const fn offset_towards(direction: Direction, x: usize, y: usize) -> (usize, usize) {
            match direction {
                Direction::Left => (x - 1, y),
                Direction::Up => (x, y - 1),
                Direction::Right => (x + 1, y),
                Direction::Down => (x, y + 1),
            }
        }

        if W <= 1 || H <= 1 {
            return false;
        }

        let (x, y) = self.origin;
        let direction = gen_bounded_direction::<W, H, R>(x, y, rng);
        self.data[y][x].direction_mut().replace(direction);

        self.origin = offset_towards(direction, x, y);
        let (x, y) = self.origin;
        self.data[y][x].direction_mut().take();

        true
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
                if W != 0 {
                    row[0] = MazeNode::new_towards(Direction::Up);
                }
                row
            }

            let mut data = [default_row(); H];
            if W != 0 && H != 0 {
                data[0][0] = MazeNode::new_origin();
            }
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
