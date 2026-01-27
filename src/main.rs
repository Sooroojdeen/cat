use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let args_amount = args.iter().count();
    let contents = fs::read_to_string(&args[1])
    .expect("Should have been able to read the file");
    if args.len() < 2 {
        println!("enter a file name!");
    }
    else{
        let mut matcher = 0;
        for check in 0..args_amount{
            if args[check].contains("-b"){
                matcher = 1;
                }
            }
        match matcher{
            0 => println!("{}", contents),
            1 =>{
                let mut lc = 1;
                    for line in contents.lines(){
                        println!("{}. {}", lc, line);
                        lc += 1;
                    }
                },
            _ => println!("idonno give up")
            }
        }
    }