use StructIter::IterateFields;

#[derive(IterateFields)]
struct MyStruct {
    a: i32,
    b: String,
    c: f64,
}

fn main() {
    let my_instance = MyStruct {
        a: 42,
        b: "Hello, World!".to_string(),
        c: 3.14,
    };

    for (field_name, field_value) in my_instance.iterate_fields() {
        println!("Field {}: {:?}", field_name, field_value);
    }
}
