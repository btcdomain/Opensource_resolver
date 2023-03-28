use btcdomain_resolver::PROGRAM_HASH;

fn main() {
    let data = hex::decode("02d4d009f837bfebbeb6e1fed2aadd336973e85971e574816791ca617faedd65").unwrap();
    println!("data: {:?}", data);
}