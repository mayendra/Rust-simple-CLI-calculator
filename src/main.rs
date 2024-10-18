use std::io;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("1. (+) ");
        println!("2. (-) ");
        println!("3. (*) ");
        println!("4. (/) ");
        println!("5. Exit ");

        let mut choose = String::new();
        io::stdin().read_line(&mut choose).expect("Failed to read input! ");

        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choose == 5 {
            println!("Exit.....");
            break;
        }

        let (num1, num2) = get_numbers();

        match choose {
            1 => println!("Result : {}", num1 + num2),
            2 => println!("Result : {}", num1 - num2),
            3 => println!("Result : {}", num1 * num2),
            4 => {
                if num2 != 0.0 {
                    println!("Result : {}", num1 / num2);
                } else {
                    println!("Cant divide by zero!");
                }
            }

            _ => println!("Choice doesnt valid!"),
        }
    }
}

fn get_numbers() -> (f64,f64) {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Input first number : ");
    io::stdin().read_line(&mut input1).expect("Failed to read input!");
    println!("Input second number : ");
    io::stdin().read_line(&mut input2).expect("Failed to read input!");

    let num1: f64 = input1.trim().parse().expect("Type the valid numbers!");
    let num2: f64 = input2.trim().parse().expect("Type the valid numbers!");

    (num1, num2)
}