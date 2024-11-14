
// THIS PROGRAM DETERMINES AN EMPLOYEE'S INCENTIVE BASED ON AGE AND EXPERIENCE

use std::io;

fn main() {

    println!("WELCOME, EMPLOYEE");

    let mut rln1 = String::new();
    let mut rln2 = String::new();
    let mut inct = 0;

    println!("What is your age?");
        io::stdin().read_line(&mut rln1).expect("Failed to read input");
    let age:u32 = rln1.trim().parse().expect("Invalid input");

    
    println!("Are you experienced? \nInput 'true' or 'false' in SMALL LETTERS");
        io::stdin().read_line(&mut rln2).expect("Failed to read input");
    let xp:bool = rln2.trim().parse::<bool>().expect("Invalid input");


    if xp == true {

        if age >=40 {
            inct = 1_560_000;
        }

        else if age >=30 {
            inct = 1_480_000;
        }

        else if age <28 {
            inct = 1_300_000;
        }

    }

    else {
        inct = 100_000;
    }

    println!("Your incentive is ₦{}", inct);

}