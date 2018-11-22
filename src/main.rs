extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::io::{BufReader, BufRead, BufWriter, Lines, Write};
use std::fs::{File};
use std::iter::{Iterator};
use std::time::Instant;

#[derive(Clone, Copy, Deserialize, Serialize)]
struct Point (i32, i32, i32);

impl Point {
    /// Euclidean distance between 2 points
    fn eucl_dist(&self, rhs : Self) -> f32 {
        (
            (
                (self.0 - rhs.0).pow(2) +
                (self.1 - rhs.1).pow(2) +
                (self.2 - rhs.2).pow(2)
            ) as f32
        ).sqrt()
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let timer = Instant::now();
    let input = File::open("data.txt")?;
    let reader = BufReader::new(input);

    let output = File::create("prepared.txt")?;
    let mut writer = BufWriter::with_capacity(32 * 2usize.pow(20), output); // 32MB buffered

    let mut lines = reader.lines().into_iter();
    let record = _convert(&mut lines);
    let n = 5; // number of interested point to take
    let mut ref_points = record;

    for line in lines {
        let record = serde_json::from_str::<Vec<Point>>(&line.unwrap())?;
        let moved_points = record;
        let distances = _eucl_distance_str(&ref_points[0..n], &moved_points[0..n])?;
        ref_points = moved_points;

        // it prints float only in 8 precision while python print in 15
        // writer.write(serde_json::to_vec(&distances)?.as_slice())?;
        
        writer.write(b"[")?;
        writer.write(distances.join(", ").as_bytes())?;
        writer.write(b"]\n")?;
    }

    println!("Done in {:?}", timer.elapsed());
    Ok(())
}

fn _convert(lines : &mut Lines<BufReader<File>>) -> Vec<Point> {
    serde_json::from_str::<Vec<Point>>(&lines.next().unwrap().unwrap()).unwrap()
}

fn _eucl_distance_str(lhs : &[Point], rhs : &[Point]) -> Result<Vec<String>, &'static str> {
    if lhs.len() != rhs.len() {
        return Err("lhs and rhs have different length");
    }

    Ok(lhs.iter().enumerate().map(|(i, left)| {format!("{:.*}", 15, left.eucl_dist(rhs[i]))}).collect())
}