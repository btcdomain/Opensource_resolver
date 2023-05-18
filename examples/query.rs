use btcdomain_resolver::*;
use rocket::tokio::runtime::Runtime;
fn main() {
    let rt = Runtime::new().unwrap();
    let query = rt.block_on(get_inscribe_by_number_api(2249054));
    println!("{:?}", query);
}
