use std::borrow::Borrow;
use std::str::FromStr;

use quick_xml::{
    events::{attributes::Attributes, Event},
    Reader,
};

use crate::path::{Path, Point};

pub fn parse_gpx(xml: &str) -> Vec<Path> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    let mut paths = vec![];
    let mut current_points = vec![];

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) if e.name() == b"trkpt" => {
                if let Some(pt) = parse_trkpt_attributes(e.attributes()) {
                    current_points.push(pt);
                }
            }
            Ok(Event::End(e)) if e.name() == b"trkseg" && !current_points.is_empty() => {
                let mut temp = vec![];
                std::mem::swap(&mut current_points, &mut temp);
                paths.push(Path::new(temp));
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    paths
}

fn parse_trkpt_attributes(attributes: Attributes) -> Option<Point> {
    let mut lat = None;
    let mut lon = None;

    for attribute in attributes.filter_map(|a| a.ok()) {
        match attribute.key {
            b"lat" => {
                lat = std::str::from_utf8(attribute.value.borrow())
                    .ok()
                    .map(|val| f64::from_str(val).ok())
                    .flatten();
            }
            b"lon" => {
                lon = std::str::from_utf8(attribute.value.borrow())
                    .ok()
                    .map(|val| f64::from_str(val).ok())
                    .flatten();
            }
            _ => {}
        }
    }

    Some(Point::new(lat?, lon?))
}
