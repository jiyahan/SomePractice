use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Rect {
    #[serde(flatten)]
    left_top: Point,
    right_bottom: Point,
    height: i32,
    width: i32,
}

pub fn print_rect() {
    let rect = Rect {
        left_top: Point { x: 0, y: 0 },
        right_bottom: Point { x: 10, y: 10 },
        height: 10,
        width: 10,
    };
    let serialized = serde_json::to_string(&rect).unwrap();
    // serialized = {"x":0,"y":0,"right_bottom":{"x":10,"y":10},"height":10,"width":10}
    println!("serialized = {}", serialized);
}
