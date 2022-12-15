use advent_2022::{simple_parse, Vec2};
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub const fn try_new(min: i32, max: i32) -> Option<Self> {
        if min <= max {
            Some(Self { min, max })
        } else {
            None
        }
    }

    pub const fn new_center_radius(center: i32, radius: u32) -> Self {
        Self {
            min: center - radius as i32,
            max: center + radius as i32,
        }
    }

    pub const fn new_point(point: i32) -> Self {
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
    ranges: Vec<Range>,
    coverage: u32,
}

impl RangeSet {
    pub fn coverage(&self) -> u32 {
        self.coverage
    }

    pub fn ranges(&self) -> &[Range] {
        self.ranges.as_slice()
    }

    pub fn add(&mut self, range: Range) {
        match self.find_intersection(range) {
            Ok((min_index, max_index)) => {
                let dropped_max = self
                    .ranges
                    .drain(min_index + 1..=max_index)
                    .inspect(|r| self.coverage -= r.size())
                    .last()
                    .map(|r| r.max);
                let updated = &mut self.ranges[min_index];

                self.coverage -= updated.size();
                if range.min < updated.min {
                    updated.min = range.min;
                }
                if let Some(dropped_max) = dropped_max {
                    updated.max = dropped_max.max(range.max);
                } else if range.max > updated.max {
                    updated.max = range.max;
                }
                self.coverage += updated.size();
            }
            Err(index) => {
                self.ranges.insert(index, range);
                self.coverage += range.size();
            }
        }
    }

    pub fn contains_point(&self, point: i32) -> bool {
        self.ranges
            .binary_search_by(|r| r.compare_point(point))
            .is_ok()
    }

    fn find_intersection(&self, range: Range) -> Result<(usize, usize), usize> {
        let min_index = self
            .ranges
            .binary_search_by(|r| r.extend_max(1).compare_point(range.min));
        let max_index = self
            .ranges
            .binary_search_by(|r| r.extend_min(1).compare_point(range.max));
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

fn main() {
    const ROW: i32 = 2000000;
    let mut ranges = RangeSet::default();
    let mut beacons_on_row = HashSet::new();
    for line in include_str!("input.txt").lines() {
        let (sensor_x, sensor_y, beacon_x, beacon_y) = simple_parse!(
            line => "Sensor at x=", @, ", y=", @, ": closest beacon is at x=", @, ", y=", @,
        )
        .unwrap();
        let sensor = Vec2::new(sensor_x, sensor_y);
        let beacon = Vec2::new(beacon_x, beacon_y);
        if beacon.y == ROW {
            beacons_on_row.insert(beacon.x);
        }
        let radius = sensor.manhattan_dist(beacon);
        let dist_to_row = sensor.y.abs_diff(ROW);
        if let Some(radius_at_row) = radius.checked_sub(dist_to_row) {
            let range = Range::new_center_radius(sensor.x, radius_at_row);
            ranges.add(range);
        }
    }
    println!("{:?}", ranges);
    println!(
        "Impossible positions at {ROW}: {}",
        ranges.coverage() - beacons_on_row.len() as u32
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut ranges = RangeSet::default();
        let r12 = Range::try_new(1, 2).unwrap();
        let r33 = Range::try_new(3, 3).unwrap();
        let r48 = Range::try_new(4, 8).unwrap();
        ranges.add(r12);
        ranges.add(r48);
        assert_eq!(ranges.coverage(), 7);
        assert_eq!(ranges.ranges(), [r12, r48]);
        ranges.add(r33);
        assert_eq!(ranges.coverage(), 8);
        assert_eq!(ranges.ranges(), [Range::try_new(1, 8).unwrap()]);
    }
}
