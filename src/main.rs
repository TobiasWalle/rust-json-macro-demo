mod json;

fn main() {
    let employees = json!([
        { "name": "Susan", "age": 30 },
        { "name": "Karl", "age": 40 },
    ]);
    dbg!(employees);
}
