use std::str::FromStr;

use quick_xml::{
    events::{BytesText, Event},
    Reader,
};

use crate::path::{Path, Point};

pub fn parse_tcx(xml: &str) -> Vec<Path> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    let mut paths = vec![];
    let mut current_points = vec![];

    let mut last_lon = None;
    let mut last_lat = None;

    let mut is_in_lat = false;
    let mut is_in_lon = false;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) if e.name() == b"LatitudeDegrees" => is_in_lat = true,
            Ok(Event::Text(e)) if is_in_lat => last_lat = parse_float(e, &reader),
            Ok(Event::End(e)) if e.name() == b"LatitudeDegrees" => is_in_lat = false,

            Ok(Event::Start(e)) if e.name() == b"LongitudeDegrees" => is_in_lon = true,
            Ok(Event::Text(e)) if is_in_lon => last_lon = parse_float(e, &reader),
            Ok(Event::End(e)) if e.name() == b"LongitudeDegrees" => is_in_lon = false,

            Ok(Event::End(e)) if e.name() == b"Trackpoint" => {
                if let (Some(lat), Some(lon)) = (last_lat, last_lon) {
                    current_points.push(Point::new(lat, lon));
                    last_lon = None;
                    last_lat = None;
                }
            }

            Ok(Event::End(e)) if e.name() == b"Track" && !current_points.is_empty() => {
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

fn parse_float(text: BytesText, reader: &Reader<&[u8]>) -> Option<f64> {
    text.unescape_and_decode(reader)
        .ok()
        .map(|text| f64::from_str(&text).ok())
        .flatten()
}
