use core::panic;

/// Segment to be printed
#[derive(Debug, Eq, PartialEq)]
pub struct Segment {
    first: Point,
    second: Point,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Layer {
    /// The index of the layer, 0 is the bottom one and is the first to be printed
    index: usize,
    segments: Vec<Segment>,
}

/// An object to be printed
#[derive(Debug, Eq, PartialEq)]
pub struct Obj {
    /// Layers from bottom to top, with each layer's index matching the index in this Vec
    layers: Vec<Layer>,
}

// The provided files under data/ are given under this format.
//
// The first line contains the number of layers
// Then, we have all data from all layers from bottom to top.
// The first line of a layer contains the layer index and the number of segments
// There is the same amount of lines after to define each segment, with X then Y coordinate of the
// first point, then X and Y coordinate of the second point. The given sens is not a constraint,
// the segments could be printed in the reverse sens as well.
//
//
// 132 Nb couches
// 0 2029 Numéro de couche, nb segments
// 92187 76081 93055 75212
// 93055 75212 94671 74277
// 94671 74277 100626 70841
// 100626 70841 101812 70524
// ...
//
// 1 940 Numéro de couche, nb segments
// 90505 88409 90505 79629
// 90505 79629 90823 78443
// 90823 78443 92187 76081
// ...
// ...

impl Obj {
    pub fn from_file_content(content: &str) -> Self {
        let mut lines = content.lines();
        let first_line = lines
            .next()
            .expect("First line with layers count must exist...");
        let nb_layers = first_line
            .split(" ")
            .next()
            .map(&str::parse::<usize>)
            .expect("First line must include a number at start")
            .expect("Invalid number for usize");
        let mut obj = Obj {
            layers: Vec::with_capacity(nb_layers),
        };

        for i in 0..nb_layers {
            let first_layer_line = lines
                .next()
                .unwrap_or_else(|| panic!("First line of layer {i}"));
            let mut splits = first_layer_line.split(" ");

            let layer_index = splits
                .next()
                .map(&str::parse::<usize>)
                .expect("Layer index must be present before first space")
                .expect("Invalid Layer index");
            assert_eq!(
                i, layer_index,
                "Layer index and iteration index must be equal"
            );
            let nb_segments = splits
                .next()
                .map(&str::parse::<usize>)
                .expect("Second number must be present")
                .expect("Invalid number for nb of segments");
            let mut new_layer = Layer {
                index: i,
                segments: Vec::with_capacity(nb_segments),
            };

            for j in 0..nb_segments {
                let segment_line = lines
                    .next()
                    .unwrap_or_else(|| panic!("A segment should exist for index {j}"));

                let mut splits = segment_line.split(" ");

                let mut values: [u32; 4] = [0; 4];
                for k in 0..4 {
                    let value = splits
                        .next()
                        .map(&str::parse::<u32>)
                        .expect("Number must be present")
                        .unwrap_or_else(|_| {
                            panic!("Invalid number for point coordinate {k} in '{segment_line}'")
                        });
                    values[k] = value;
                }

                let new_segment = Segment {
                    first: Point {
                        x: values[0],
                        y: values[1],
                    },
                    second: Point {
                        x: values[2],
                        y: values[3],
                    },
                };
                new_layer.segments.push(new_segment);
            }

            obj.layers.push(new_layer);
        }

        obj
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ds::{Layer, Obj, Point, Segment};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_file_can_be_parsed() {
        let content = "2 Nb couches
0 4 Numéro de couche, nb segments
92187 76081 93055 75212
93055 75212 94671 74277
94671 74277 100626 70841
100626 70841 101812 70524
1 3 Numéro de couche, nb segments
90505 88409 90505 79629
90505 79629 90823 78443
90823 78443 92187 76081";

        let obj = Obj::from_file_content(content);
        assert_eq!(
            obj,
            Obj {
                layers: vec![
                    Layer {
                        index: 0,
                        segments: vec![
                            Segment {
                                first: Point { x: 92187, y: 76081 },
                                second: Point { x: 93055, y: 75212 }
                            },
                            Segment {
                                first: Point { x: 93055, y: 75212 },
                                second: Point { x: 94671, y: 74277 }
                            },
                            Segment {
                                first: Point { x: 94671, y: 74277 },
                                second: Point {
                                    x: 100626,
                                    y: 70841
                                }
                            },
                            Segment {
                                first: Point {
                                    x: 100626,
                                    y: 70841
                                },
                                second: Point {
                                    x: 101812,
                                    y: 70524
                                }
                            }
                        ]
                    },
                    Layer {
                        index: 1,
                        segments: vec![
                            Segment {
                                first: Point { x: 90505, y: 88409 },
                                second: Point { x: 90505, y: 79629 }
                            },
                            Segment {
                                first: Point { x: 90505, y: 79629 },
                                second: Point { x: 90823, y: 78443 }
                            },
                            Segment {
                                first: Point { x: 90823, y: 78443 },
                                second: Point { x: 92187, y: 76081 }
                            }
                        ]
                    }
                ]
            }
        );
    }
}
