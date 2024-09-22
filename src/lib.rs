use easy::{
    enums::{calculate_area, Shape},
    options::find_first_a,
    packages::get_current_time,
    results::read_file,
    structs::{Rect, User},
    vlf::{fib, get_str_len, is_even},
};

mod easy;

pub fn easy() {
    println!("is 5 a even number: {}", is_even(5));
    println!("8th element of fibonacci series: {}", fib(8));
    println!("Length of 'Mukesh': {}", get_str_len("Mukesh".to_owned()));

    let user = User {
        first_name: "Mukesh".to_owned(),
        last_name: "Singh".to_owned(),
        age: 27,
    };

    println!("This my info: {:#?}", user);

    let rect = Rect {
        height: 10,
        width: 5,
    };
    println!(
        "The are is {} and perimeter is {}",
        rect.area(),
        rect.perimeter()
    );

    let rect = Shape::Rectangle(5.5, 4.4);
    let circle = Shape::Circle(3.3);
    println!(
        "Area of rect {}, and circle {}",
        calculate_area(rect),
        calculate_area(circle)
    );

    let index = find_first_a("Mukesh".to_owned());
    match index {
        Some(val) => println!("char 'a' is at index: {}", val),
        None => println!("a not found"),
    }

    let result = read_file("./src/enums.rs".to_owned());
    match result {
        Ok(val) => println!("{}", val),
        Err(err) => println!("Failed to read the file: {}", err),
    }

    get_current_time();
}
