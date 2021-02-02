use std::{
    collections::HashMap,
    fmt,
    hash::Hash,
    ops::{Add, Div, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

impl Point {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    fn distance(&self, other: Self) -> f64 {
        ((self.lat - other.lat).powi(2) + (self.lon - other.lon).powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.lat, self.lon)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            lat: self.lat - rhs.lat,
            lon: self.lon - rhs.lon,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lat: self.lat + rhs.lat,
            lon: self.lon + rhs.lon,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            lat: self.lat / rhs,
            lon: self.lon / rhs,
        }
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ((self.lat * 1.0e6) as u64, (self.lon * 1.0e6) as u64).hash(state)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        ((self.lat * 1.0e6) as u64, (self.lon * 1.0e6) as u64)
            == ((other.lat * 1.0e6) as u64, (other.lon * 1.0e6) as u64)
    }
}

impl Eq for Point {}

#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn combine(&self, all_points: &Points) -> Path {
        let mut combined_pts = vec![];
        for point in &self.points {
            let close = all_points.get_close(*point);
            let average: Point = close
                .iter()
                .fold(Point::new(0.0, 0.0), |sum, next| sum + *next)
                / (close.len() as f64);
            combined_pts.push(average);
        }
        Self::new(combined_pts)
    }

    pub fn smooth(&self) -> Self {
        let mut smoothed = vec![];

        if !self.points.is_empty() {
            smoothed.push(*self.points.first().unwrap());
        }

        for index in 1..(self.points.len() - 1) {
            let (prev, cur, next) = (
                self.points[index - 1],
                self.points[index],
                self.points[index + 1],
            );
            let average = (prev + cur + next) / 3.0;
            smoothed.push(average);
        }

        if !self.points.is_empty() {
            smoothed.push(*self.points.last().unwrap());
        }

        Self::new(smoothed)
    }

    pub fn as_json_array(&self) -> String {
        format!(
            "[{}]",
            self.points
                .iter()
                .map(|p| format!("[{},{}]", p.lat, p.lon))
                .fold_first(|array, next| array + "," + &next)
                .unwrap_or_else(String::new)
        )
    }
}

pub struct Points {
    radius: f64,
    buckets: HashMap<(usize, usize), Vec<Point>>,
}

impl Points {
    pub fn new(points: Vec<Point>, radius: f64) -> Self {
        let mut buckets = HashMap::new();

        for point in points {
            buckets
                .entry(Self::address(point, radius))
                .or_insert_with(Vec::new)
                .push(point);
        }

        Self { buckets, radius }
    }

    fn address(point: Point, radius: f64) -> (usize, usize) {
        (
            (point.lat / radius).abs() as usize,
            (point.lon / radius).abs() as usize,
        )
    }

    pub fn get_close(&self, point: Point) -> Vec<Point> {
        let address = Self::address(point, self.radius);

        let mut close = vec![];
        for lat_offset in 0..3 {
            for lon_offset in 0..3 {
                close.append(
                    &mut self
                        .buckets
                        .get(&(address.0 + lat_offset - 1, address.1 + lon_offset - 1))
                        .map_or_else(Vec::new, |p| p.clone()),
                );
            }
        }

        close
            .into_iter()
            .filter(|p| p.distance(point) <= self.radius)
            .collect()
    }
}
