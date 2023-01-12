use std::io;

fn main() {
    let mut option = String::new();

    loop {
        println!(
            "
            Select a option:
            0 - Exit
            1 - Prints
            2 - Input read
            3 - Array strings
            4 - Calculator
            5 -
            "
        );
        io::stdin()
            .read_line(&mut option)
            .expect("Ops... error trying read input");

        // let opt: i32 = option.trim().parse<str>().unwrap().except("Error..");

        let opt: i32 = option.trim().parse().unwrap();

        if opt == 0 {
            break;
        } else if opt == 1 {
            basic_print();
        } else if opt == 2 {
            read_name_and_print();
        } else {
            return println!("Option not mapped");
        }
    }
}

fn basic_print() {
    println!("Basic print... :) ");
    println!("{:-^20}", "Nice print");
    println!(
        "
             #####  #### \n
             Basic Print \n
             #####  #### \n
        ");
}

fn read_name_and_print() {
    let mut name = String::new();
    println!("Type your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Ops.. error reading input");

    println!("Your name is {}", name);
    println!("Your name is {}", name);
    println!(
        "The size of your name is {} (using .len())",
        name.trim().len()
    ); // When you are using .len() you are lookin the bytes
    println!(
        "The size of your name is {} (using .count())",
        name.trim().chars().count()
    ); // WHen you are using .chars() your are looking the characters

}
