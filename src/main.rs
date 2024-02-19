use rand::random;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Serialize)]
struct TestCase {
    id: i32,
    time_out: i32,
    input: String,
    output: String,
}

#[derive(Serialize)]
struct OuterObject {
    test_cases: Vec<TestCase>,
}

fn decimal_to_binary(mut n: i32) -> i32 {
    let mut ans = 0;
    let mut k = 0;
    while n > 0 {
        let p = n % 2;
        ans = ans + p * 10_i32.pow(k);
        k += 1;
        n = n / 2;
    }
    ans
}

fn get_char() -> char {
    let chars = ['e', '+', '-', 'c', '^'];
    let weights = [3, 15, 15, 10, 1]; // Adjust weights as desired
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let random_index = dist.sample(&mut rng);
    let random_char = chars[random_index];
    random_char
}

/// Function to generate random characters for longer test cases for `calculator` 
/// of fixed length (number of calculation operations). This does not have an `e` option.
fn get_char_long() -> char {
    let chars = ['+', '-', 'c'];
    let weights = [1, 1, 1]; // Adjust weights as desired
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let random_index = dist.sample(&mut rng);
    let random_char = chars[random_index];
    random_char
}

fn get_number() -> i32 {
    let mut rng = rand::thread_rng();
    let x1: i32 = rng.gen_range(-1000..1000);
    x1
}

/// Standard implementation of `calculator` function, which returns (input, output) as a String tuple.
fn calculator(mut balance: i32) -> (String, String) {
    let mut output: String = String::new();
    let mut input: String = String::new();
    let mut stack: Vec<(char, i32)> = Vec::new();
    loop {
        output.push_str(format!("Balance: {}\n", balance).as_str());
        let option = get_char();
        input.push(option);
        input.push('\n');
        if option == '+' {
            let x = get_number();
            input.push_str(x.to_string().as_str());
            input.push('\n');
            balance += x;
            stack.push(('+', x));
        } else if option == '-' {
            let x = get_number();
            input.push_str(x.to_string().as_str());
            input.push('\n');
            balance -= x;
            stack.push(('-', x));
        } else if option == 'e' {
            break;
        } else if option == 'c' {
            let last = stack.pop();
            match last {
                Some((option, x)) => {
                    if option == '+' {
                        balance -= x;
                    } else {
                        balance += x;
                    }
                }
                None => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    (input, output)
}

/// Implementation of `calculator` function with a fixed number of operations (100000) for longer test cases.
fn calculator_with_operations(mut balance: i32, operations: i32) -> (String, String) {
    let mut output: String = String::new();
    let mut input: String = String::new();
    let mut stack: Vec<(char, i32)> = Vec::new();
    for r in 0..operations {
        output.push_str(format!("Balance: {}\n", balance).as_str());
        let mut option = get_char_long();
        while option == 'c' && stack.len() == 0 {
            option = get_char_long();
        }
        if r == operations - 1 {
            option = 'e';
        }
        input.push(option);
        input.push('\n');
        if option == '+' {
            let x = get_number();
            input.push_str(x.to_string().as_str());
            input.push('\n');
            balance += x;
            stack.push(('+', x));
        } else if option == '-' {
            let x = get_number();
            input.push_str(x.to_string().as_str());
            input.push('\n');
            balance -= x;
            stack.push(('-', x));
        } else if option == 'e' {
            break;
        } else if option == 'c' {
            let last = stack.pop();
            match last {
                Some((option, x)) => {
                    if option == '+' {
                        balance -= x;
                    } else {
                        balance += x;
                    }
                }
                None => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    (input, output)
}

fn generate_test_cases() -> Vec<TestCase> {
    // 1. Test cases for decimal_to_binary
    let mut test_cases = Vec::new();
    let prompt = "1. decimal to binary  2. calculator(0)\nWhich function do you want to test? Enter 1 or 2 accordingly.\nEnter a number between 0 and 1000 (both inclusive): ";
    for test_case_id in 1..=1000 {
        let n = test_case_id;
        // Weird string interpolation to match the format of input & output as per question.
        let output = format!("{}Binary is: {}\n", prompt, decimal_to_binary(n));
        let test_case = TestCase {
            id: test_case_id,
            time_out: 2,
            input: format!("1\n{}\n", n),
            output: output,
        };
        test_cases.push(test_case);
    }

    // 2. Test cases for calculator.
    let prompt = "1. decimal to binary  2. calculator(0)\nWhich function do you want to test? Enter 1 or 2 accordingly.\n";
    for test_case_id in 1001..=1500 {
        let (input, output) = calculator(0);
        let test_case = TestCase {
            id: test_case_id,
            time_out: 2,
            input: format!("2\n{}", input),
            output: format!("{}{}", prompt, output),
        };
        test_cases.push(test_case);
    }

    // 3. Test cases for calculator with fixed number of operations (N).
    // Consider changing this value to generate slightly smaller test cases, 
    // like N = 1000 or N = 10000.
    const N: i32 = 100000;
    for test_case_id in 1501..=1510 {
        let (input, output) = calculator_with_operations(0, N);
        let test_case = TestCase {
            id: test_case_id,
            time_out: 2,
            input: format!("2\n{}", input),
            output: format!("{}{}", prompt, output),
        };
        test_cases.push(test_case);
    }
    test_cases
}

fn main() {
    let data = OuterObject {
        test_cases: generate_test_cases(),
    };

    // Following code only writes the data to a .json file.
    let json_data = serde_json::to_string(&data).expect("Failed to serialize data to JSON.");

    let mut file = File::create("data.json").expect("Failed to create file.");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file.");

    println!("Data has been written to data.json.");
}
