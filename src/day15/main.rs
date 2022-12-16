use advent_2022::{simple_parse, Vec2};
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InclRange {
    min: i32,
    max: i32,
}

impl InclRange {
    pub const fn new(min: i32, max: i32) -> Self {
        assert!(min <= max, "min was greater than max");
        Self { min, max }
    }

    pub const fn try_new(min: i32, max: i32) -> Option<Self> {
        if min <= max {
            Some(Self { min, max })
        } else {
            None
        }
    }

    pub const fn from_center_radius(center: i32, radius: u32) -> Self {
        Self {
            min: center - radius as i32,
            max: center + radius as i32,
        }
    }

    pub const fn from_point(point: i32) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub const fn size(&self) -> u32 {
        (self.max - self.min) as u32 + 1
    }

    pub const fn extend_min(&self, diff: u32) -> Self {
        Self {
            min: self.min - diff as i32,
            max: self.max,
        }
    }

    pub const fn extend_max(&self, diff: u32) -> Self {
        Self {
            min: self.min,
            max: self.max + diff as i32,
        }
    }

    pub const fn contains(&self, point: i32) -> bool {
        self.min <= point && point <= self.max
    }

    pub const fn compare_point(&self, point: i32) -> Ordering {
        if point < self.min {
            Ordering::Greater
        } else if point > self.max {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RangeSet {
    ranges: Vec<InclRange>,
    coverage: u32,
}

impl RangeSet {
    pub fn coverage(&self) -> u32 {
        self.coverage
    }

    pub fn ranges(&self) -> &[InclRange] {
        self.ranges.as_slice()
    }

    pub fn add(&mut self, ranges: InclRange) {
        match self.find_intersection(ranges) {
            Ok((min_index, max_index)) => {
                let dropped_max = self
                    .ranges
                    .drain(min_index + 1..=max_index)
                    .inspect(|r| self.coverage -= r.size())
                    .last()
                    .map(|r| r.max);
                let updated = &mut self.ranges[min_index];

                self.coverage -= updated.size();
                if ranges.min < updated.min {
                    updated.min = ranges.min;
                }
                if let Some(dropped_max) = dropped_max {
                    updated.max = dropped_max.max(ranges.max);
                } else if ranges.max > updated.max {
                    updated.max = ranges.max;
                }
                self.coverage += updated.size();
            }
            Err(index) => {
                self.ranges.insert(index, ranges);
                self.coverage += ranges.size();
            }
        }
    }

    pub fn contains_point(&self, point: i32) -> bool {
        self.ranges
            .binary_search_by(|r| r.compare_point(point))
            .is_ok()
    }

    fn find_intersection(&self, ranges: InclRange) -> Result<(usize, usize), usize> {
        let min_index = self
            .ranges
            .binary_search_by(|r| r.extend_max(1).compare_point(ranges.min));
        let max_index = self
            .ranges
            .binary_search_by(|r| r.extend_min(1).compare_point(ranges.max));
        match (min_index, max_index) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Ok(a), Err(b)) => Ok((a, b - 1)),
            (Err(a), Ok(b)) => Ok((a, b)),
            (Err(a), Err(b)) => {
                if a == b {
                    Err(a)
                } else {
                    Ok((a, b - 1))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Region {
    pub pos: Vec2,
    pub radius: u32,
}

impl Region {
    pub fn contains(&self, point: Vec2) -> bool {
        self.pos.manhattan_dist(point) <= self.radius
    }

    pub fn border(&self) -> RegionBorder {
        RegionBorder {
            center: self.pos,
            side: 0,
            next_pos: self.pos + Vec2::UP * (self.radius as i32 + 1),
        }
    }
}

#[derive(Debug, Clone)]
struct RegionBorder {
    center: Vec2,
    next_pos: Vec2,
    side: u8,
}

impl Iterator for RegionBorder {
    type Item = Vec2;

    fn next(&mut self) -> Option<Vec2> {
        let step = match self.side {
            0 => Vec2::DOWN_RIGHT,
            1 => Vec2::DOWN_LEFT,
            2 => Vec2::UP_LEFT,
            3 => Vec2::UP_RIGHT,
            _ => return None,
        };
        let result = self.next_pos;
        self.next_pos += step;
        let end_of_side = if self.side & 1 == 0 {
            self.next_pos.y == self.center.y
        } else {
            self.next_pos.x == self.center.x
        };
        if end_of_side {
            self.side += 1;
        }
        Some(result)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Scan {
    sensors: Vec<Region>,
    beacons: HashSet<Vec2>,
}

impl Scan {
    pub fn sensors(&self) -> &[Region] {
        &self.sensors
    }

    pub fn is_covered(&self, point: Vec2) -> bool {
        self.sensors.iter().any(|s| s.contains(point))
    }

    pub fn add_sensor_beacon(&mut self, sensor: Vec2, beacon: Vec2) {
        self.sensors.push(Region {
            pos: sensor,
            radius: sensor.manhattan_dist(beacon),
        });
        self.beacons.insert(beacon);
    }

    pub fn ranges_on_row(&self, row: i32) -> RangeSet {
        let mut ranges = RangeSet::default();
        for sensor in &self.sensors {
            let dist_to_row = sensor.pos.y.abs_diff(row);
            if let Some(r) = sensor.radius.checked_sub(dist_to_row) {
                let range = InclRange::from_center_radius(sensor.pos.x, r);
                ranges.add(range);
            }
        }
        ranges
    }

    pub fn impossible_on_row(&self, row: i32) -> u32 {
        let mut result = self.ranges_on_row(row).coverage();
        for beacon in &self.beacons {
            if beacon.y == row {
                result -= 1;
            }
        }
        result
    }
}

fn main() {
    const ROW: i32 = 2_000_000;
    const BEACON_RANGE: InclRange = InclRange::new(0, ROW * 2);

    let mut scan = Scan::default();
    for line in include_str!("input.txt").lines() {
        let (sensor_x, sensor_y, beacon_x, beacon_y) = simple_parse!(
            line => "Sensor at x=", @, ", y=", @, ": closest beacon is at x=", @, ", y=", @,
        )
        .unwrap();
        let sensor = Vec2::new(sensor_x, sensor_y);
        let beacon = Vec2::new(beacon_x, beacon_y);
        scan.add_sensor_beacon(sensor, beacon);
    }

    println!(
        "Impossible positions at {ROW}: {}",
        scan.impossible_on_row(ROW)
    );

    let beacon_pos = scan
        .sensors()
        .iter()
        .flat_map(|s| s.border())
        .filter(|&p| BEACON_RANGE.contains(p.x) && BEACON_RANGE.contains(p.y) && !scan.is_covered(p))
        .next()
        .unwrap();
    let tuning_freq = beacon_pos.x as u64 * 4_000_000 + beacon_pos.y as u64;
    println!("Beacon position: {beacon_pos}, tuning frequency: {tuning_freq}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_set() {
        let mut ranges = RangeSet::default();
        let r12 = InclRange::try_new(1, 2).unwrap();
        let r33 = InclRange::try_new(3, 3).unwrap();
        let r48 = InclRange::try_new(4, 8).unwrap();
        ranges.add(r12);
        ranges.add(r48);
        assert_eq!(ranges.coverage(), 7);
        assert_eq!(ranges.ranges(), [r12, r48]);
        ranges.add(r33);
        assert_eq!(ranges.coverage(), 8);
        assert_eq!(ranges.ranges(), [InclRange::try_new(1, 8).unwrap()]);
    }

    #[test]
    fn test_border() {
        let r = Region {
            pos: Vec2::new(0, 1),
            radius: 0,
        };
        let mut b = r.border();

        assert_eq!(b.next(), Some(Vec2::new(0, 0)));
        assert_eq!(b.next(), Some(Vec2::new(1, 1)));
        assert_eq!(b.next(), Some(Vec2::new(0, 2)));
        assert_eq!(b.next(), Some(Vec2::new(-1, 1)));
        assert_eq!(b.next(), None);
    }
}
