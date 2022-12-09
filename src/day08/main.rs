use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

struct Grid {
    width: usize,
    height: usize,
    values: Vec<u8>,
}

impl Grid {
    pub fn parse(text: &str) -> Self {
        let mut rows = text.lines();
        if let Some(first_row) = rows.next() {
            let width = first_row.len();
            let mut height = 1;
            let mut values: Vec<u8> = first_row.chars().map(Self::parse_digit).collect();
            for row in rows {
                assert_eq!(row.len(), width);
                values.extend(row.chars().map(Self::parse_digit));
                height += 1;
            }
            debug_assert_eq!(width * height, values.len());
            Self {
                width,
                height,
                values,
            }
        } else {
            Self {
                width: 0,
                height: 0,
                values: vec![],
            }
        }
    }

    fn parse_digit(c: char) -> u8 {
        c.to_digit(10).unwrap() as u8
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn max_x(&self) -> i32 {
        self.width as i32 - 1
    }

    pub const fn max_y(&self) -> i32 {
        self.height as i32 - 1
    }

    pub fn get(&self, pos: GridPos) -> Option<u8> {
        let x: usize = pos.x.try_into().ok()?;
        let y: usize = pos.y.try_into().ok()?;
        if x < self.width && y < self.height {
            Some(self.values[y * self.width + x])
        } else {
            None
        }
    }

    pub fn raycast(&self, start: GridPos, dx: i32, dy: i32, visible: &mut HashSet<GridPos>) {
        let mut hit_height = self.get(start).unwrap();
        visible.insert(start);
        let mut pos = start.offset(dx, dy);
        while let Some(h) = self.get(pos) {
            if h > hit_height {
                hit_height = h;
                visible.insert(pos);
            }
            pos = pos.offset(dx, dy);
        }
    }
}

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    let mx = grid.max_x();
    let my = grid.max_y();

    let mut visible = HashSet::new();
    for y in 0..grid.height() as i32 {
        grid.raycast(GridPos::new(0, y), 1, 0, &mut visible);
        grid.raycast(GridPos::new(mx, y as i32), -1, 0, &mut visible);
    }
    for x in 0..grid.width() as i32 {
        grid.raycast(GridPos::new(x, 0), 0, 1, &mut visible);
        grid.raycast(GridPos::new(x, my), 0, -1, &mut visible);
    }

    println!("Visible trees: {}", visible.len());
}
