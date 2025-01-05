// A PROGRAM TO GATHER COMPREHENSIVE DETAILS ABOUT CLIENT'S SIBLING(S)

use std::io;

fn main() {

    let mut in1 = String::new();
    let mut in2 = String::new();
    let mut in3 = String::new();
    let mut in4 = String::new();
    let mut in5 = String::new();
    let mut in6 = String::new();
    let mut in7 = String::new();
    let mut in8 = String::new();
    let mut in9 = String::new();
    let mut in10 = String::new();
    let mut in11 = String::new();
    let mut in12 = String::new();
    let mut in13 = String::new();
    let mut in14 = String::new();
    let mut in15 = String::new();
    let mut in16 = String::new();
    let mut in17 = String::new();
    let mut in18 = String::new();
    let mut in19 = String::new();

    println!("Welcome! \nHow many siblings do you have?");
        io::stdin().read_line(&mut in1).expect("Failed to read input");
    let num_sib:u32 = in1.trim().parse().expect("Invalid input");

    
    for x in 0..num_sib {

        println!("What is their first name?");
            io::stdin().read_line(&mut in2).expect("Failed to read input");
        let f_name = in2.trim();

        println!("How old are they?");
            io::stdin().read_line(&mut in3).expect("Failed to read input");
        let age:u32 = in3.trim().parse().expect("Invalid input");

        println!("What is their gender?");
            io::stdin().read_line(&mut in4).expect("Failed to read input");
        let sex = in4.trim();

        println!("What country do they live?");
            io::stdin().read_line(&mut in5).expect("Failed to read input");
        let addy = in5.trim();


        if age >= 18 {

            println!("Are you single, married or in a relationship? \nInput 'Single', 'Married' or 'Relationship' \n(case-sensitive)");
                io::stdin().read_line(&mut in6).expect("Failed to read input");
            let marital = in6.trim();

            if marital == "Married" {

                println!("How many children do they have?");
                    io::stdin().read_line(&mut in7).expect("Failed to read input");
                let num_child:u32 = in7.trim().parse().expect("Invalid input");
            

                if num_child >=1 {

                    for y in 1..num_child {

                        println!("What is the child's name?");
                        io::stdin().read_line(&mut in8).expect("Failed to read input");
                        let child_name = in8.trim();

            
                        println!("How old is the child?");
                            io::stdin().read_line(&mut in9).expect("Failed to read input");
                        let child_age:u32 = in9.trim().parse().expect("Invalid input");


                        println!("What is the name of the child's school or daycare?");
                            io::stdin().read_line(&mut in10).expect("Failed to read input");
                        let child_schl = in10.trim();

                    }

                }

            }

            else if marital == "Single" {

                println!("Are they a 'Student' or 'Employed' \n(case-sensitive)");
                    io::stdin().read_line(&mut in11).expect("Failed to read input");
                let work = in11.trim();

                if work == "Student" {

                    println!("University of {addy}, \n{addy}");

                    println!("Are they studying 'home' or 'abroad'? \n(case-sensitive)");
                        io::stdin().read_line(&mut in12).expect("Failed to read input");
                    let study_addy = in12.trim();

                }

                else {

                    println!("Is their job 'remote', 'onsite' or 'hybrid'? \n(case-sensitive)");
                        io::stdin().read_line(&mut in13).expect("Failed to read input");
                    let job_nature = in13.trim();

                    if job_nature == "onsite" {
                        println!("Company Name \nJob Title \nIndustry Sector");
                    }

                }

            }

            else {

                println!("How many years has the relationship been on for?");
                    io::stdin().read_line(&mut in14).expect("Failed to read input");
                let rel_years:u32 = in14.trim().parse().expect("Invalid input");

                println!("What is their partner's first name?");
                    io::stdin().read_line(&mut in15).expect("Failed to read input");
                let part_fname = in15.trim();

                println!("Do they live with their partner? \nInput 'true' or 'false' \n(case-sensitve)");
                    io::stdin().read_line(&mut in16).expect("Failed to read input");
                let live_part:bool = in16.trim().parse::<bool>().expect("Invalid input");

                if live_part == true {

                    println!("City");

                }

            }


        }

        else {

            println!("Have they completed WAEC? \nInput 'true' or 'false' \n(case-sensitve)");
                io::stdin().read_line(&mut in17).expect("Failed to read input");
            let waec:bool = in17.trim().parse::<bool>().expect("Invalid input");

            if waec == true {

                println!("Secondary School \nFinal Grade \nYear of Completion");

            }

            else {

                println!("What class are they?");
                    io::stdin().read_line(&mut in18).expect("Failed to read input");
                let class:u32 = in18.trim().parse().expect("Invalid input");

                println!("DO they plan to write WAEC? \nInput 'true' or 'false' \n(case-sensitve)");
                    io::stdin().read_line(&mut in19).expect("Failed to read input");
                let write_waec:bool = in19.trim().parse::<bool>().expect("Invalid input");

                if write_waec == true {

                    println!("2024");
                }

            }


        }

        

    }

}
