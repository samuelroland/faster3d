use std::{fmt::format, fs::read_to_string};

use colored::{ColoredString, Colorize};

use crate::util::ds::{Obj, Point};

mod util;

fn print_datasets_stats() {
    for file in std::fs::read_dir("data").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let filename = file.file_name().to_string_lossy().to_string();

        if !filename.ends_with(".txt") {
            return;
        }
        println!("Dataset {}", filename.blue());
        let obj = Obj::from_file_content(&read_to_string(path).unwrap());
        obj.print_stats();
        println!();
    }
}

fn print_dataset_details(dataset_file: &str) {
    let obj = Obj::from_file_content(&read_to_string(format!("data/{dataset_file}")).unwrap());
    let mut last_second_point = Point { x: 0, y: 0 }; //starting at origin, making the first movement an idle one
    // let mut last_second_point = obj
    //     .layers
    //     .first()
    //     .expect("At least one layer should exist...")
    //     .segments
    //     .first()
    //     .expect("At least one segment per layer")
    //     .second
    //     .clone();
    //
    let mut total_idle_movement_distance = 0.;
    for (i, layer) in obj.layers.iter().enumerate() {
        let mut total_idle_movement_distance_per_layer = 0.;
        println!("\nLayer {i}");
        for (j, segment) in layer.segments.iter().enumerate() {
            let distance = segment.distance();
            if segment.first != last_second_point {
                let idle_movement_dist = last_second_point.distance(&segment.first);
                total_idle_movement_distance_per_layer += idle_movement_dist;
                println!(
                    "Segment {j} {} -> {}: {:.2} {}",
                    segment.first,
                    segment.second,
                    distance,
                    if idle_movement_dist > 0. {
                        format!("+{:.2}", idle_movement_dist).red().bold()
                    } else {
                        ColoredString::default()
                    }
                );
            }
            last_second_point = segment.second.clone();
        }

        total_idle_movement_distance += total_idle_movement_distance_per_layer;
        println!("Total idle distance: {total_idle_movement_distance:.2}");
    }
    println!("Total idle distance: {total_idle_movement_distance:.2}");
    obj.print_stats();
    println!();
}

fn main() {
    // print_datasets_stats();

    print_dataset_details("de.txt");
}
