// fn print_item1(item: Vec<u32>) {
//     for i in item {
//         println!("{}", i);
//     }
// }

// fn borrow_not_work() {
//     let item = vec![1,2,3];
//     // The value of item is used by the first call of print_item1, 
//     // so the second call of print_item1 will not work because value is being used by fist call, this line is the owner of the value.
//     // Basically the rust compiler check who is borrowing the value
//     // print_item1(item);
//     // print_item1(item);
// }


fn print_item2(item: &Vec<u32>) {
    for i in item {
        println!("{}", i);
    }
}
fn borrow_work() {
    let item = vec![1,2,3];

    // We can pass the reference of the value for the function print_item
    print_item2(&item);
    print_item2(&item);
}


#[derive(Debug)]
struct Foo{}
fn mutable_references() {
    let mut vector = vec![Foo{}, Foo{}, Foo{}];
    let last_foo = vector.last();

    println!("last foo: {:?}", last_foo);

    vector.pop();
    println!("last foo: {:?}", last_foo);
}


fn main() {
    borrow_work();
    mutable_references();
}

