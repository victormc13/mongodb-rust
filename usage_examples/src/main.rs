//mod find_one;
mod find_many;
//mod insert_one;

fn main() {
    println!("-->Usage Examples mongodb rust driver!");

    //println!("*****find_one usage example");
    //let _find_one_example = find_one::find_one();
    println!("*****find usage example('find_many')");
    let _find_many_example = find_many::find_many();
    //println!("*****insert_one usage example");
    //let _insert_one_example = insert_one::insert_one();
}
