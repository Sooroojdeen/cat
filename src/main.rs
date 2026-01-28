use std::env;
use std::fs;

fn add_line_numbers(input_string: String) -> String {
    let mut result = String::new();
    let mut lc: i32 = 1;

    for line in input_string.lines() {
        let combined = format!("{}. {}\n", lc, line);
        result.push_str(&combined);
        lc += 1;
    }

    result
}

fn add_line_numbers_b(input_string: String) -> String {
    let mut result = String::new();
    let mut lc: i32 = 1;

    for line in input_string.lines() {
        if line.trim().is_empty(){
            let combined = format!("\n");
            result.push_str(&combined);
        }
        else{
            let combined = format!("{}. {}\n", lc, line);
            result.push_str(&combined);
            lc += 1;
        }
    }

    result
}

fn add_line_end(input_string: String) -> String {
    let mut result = String::new();
    for line in input_string.lines() {
        let combined = format!("{}$\n", line);
        result.push_str(&combined);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_amount = args.iter().count();
    let mut all_text = fs::read_to_string(&args[1])
    .expect("Should have been able to read the file");


    let mut has_both = false;

    let mut has_n = false;
    let mut has_e = false;
    let mut has_b = false;
    for check in 0..args_amount{
        if args[check].contains("-n"){
            has_n = true;
        }
        if args[check].contains("-b"){
            has_b = true;
        }

        if args[check].contains("-e"){
            has_e = true;
        }

        if has_b && has_n{
            has_both = true;
        }
    }

    if args.len() < 2 {
        println!("enter a file name!");
    }
    else{
        
        if has_n{
            all_text = add_line_numbers(all_text);
        }
        if has_e{
            all_text = add_line_end(all_text);
        }
        if has_b{
            all_text = add_line_numbers_b(all_text);
        }

        if has_both{
            all_text = String::from("You can use either -b or -n, not both!");
        }
    }
    
    println!("{}", all_text);
    }