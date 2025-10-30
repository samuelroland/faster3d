use std::fs::read_to_string;

use colored::Colorize;

use crate::util::ds::Obj;

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
        println!(
            "Layers: {}\nTotal segments: {}\nMin segments/layer: {}\nMax segments/layer: {}\n{}",
            obj.layers.len(),
            obj.layers.iter().map(|a| a.segments.len()).sum::<usize>(),
            obj.layers
                .iter()
                .min_by(|a, b| a.segments.len().cmp(&b.segments.len()))
                .unwrap()
                .segments
                .len(),
            obj.layers
                .iter()
                .max_by(|a, b| a.segments.len().cmp(&b.segments.len()))
                .unwrap()
                .segments
                .len(),
            format!("Total distance: {:.1}", obj.total_distance()).yellow()
        );
        println!();
    }
}

fn main() {
    print_datasets_stats();
}
