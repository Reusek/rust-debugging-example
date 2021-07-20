#[derive(Debug)]
struct ExampleStruct {
    a: i32,
    b: f32,
    v: Vec<i32>
}

fn main() {
    let es = ExampleStruct {
        a: 1,
        b: 2.0,
        v: vec![1, 2]
    };
    println!("{:?}", es);
}
