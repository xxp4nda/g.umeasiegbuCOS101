
// A PROGRAM TO FIND THE ROOTS OF A QUADRATIC EQUATION

use std::io;


fn main() {

    println!("A PROGRAM TO FIND THE ROOTS OF A QUADRATIC EQUATION");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    println!("Enter your value for a");
        io::stdin().read_line(&mut input1).expect("Invalid input");
    let a:f32 = input1.trim().parse().expect("Invalid input");
    
    println!("Enter your value for a");
        io::stdin().read_line(&mut input2).expect("Invalid input");
    let b:f32 = input2.trim().parse().expect("Invalid input");

    println!("Enter your value for a");
        io::stdin().read_line(&mut input3).expect("Invalid input");
    let c:f32 = input3.trim().parse().expect("Invalid input");

    let d:f32 = b.powf(2.0) - (4.0 * a * c);

    if d ==0.0
    {
        let x: f32 = (d.sqrt() - b)/ (2.0 * a);
        println!("\n\nThe discriminant, d is {d}.\nTherefore the equation has two roots that are real and equal.\nx = {}", x);
    }
    else if d>0.0 
    {
        let x1: f32 = (d.sqrt() - b)/ (2.0 * a);
        let x2: f32 = (-(d.sqrt()) - b)/ (2.0 * a);
        println!("\n\nThe discriminant,d is {d}.\nTherefore the equation has two roots that are real and distinct.\nx = {}        OR\nx = {}", x1,x2);
    }
    else{
        println!("\n\nThe discriminant,d is {d}.\nTherefore the equation has imaginary roots.");
    }
}

