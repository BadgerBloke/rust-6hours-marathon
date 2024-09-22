// Variable, loop and function

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 1..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    second
}

pub fn get_str_len(str: String) -> usize {
    str.chars().count()
}
