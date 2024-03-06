#![allow(unused_variables)]

fn main() {
    let durk = "Durk";
    let airline = "Airlines";

    //using concation
    let join = [durk, " ", airline, " is awesome!"].concat();
    println!("{}", join);

    // let join = durk.to_string() + " " + airline + " is awesome!";
    // println!("{}", join);

    // let join = format!("{} {} {}", durk, airline, "is awesome!");
    // println!("{}", join);

    //initializing an empty string and pushing into it
    let mut noel = String::new();
    noel.push_str("We hit the ground!");
    noel.push('!');
    noel = noel + "All the time!";
    println!("{}", noel);

    //casting variables
    let number_1 : f32 = 15.2;
    let number_2 : u32 = 3;
    let number_3 = number_2 as f32;

    let sum = number_1 / number_3;
    println!("{}", sum);

    //Variable scope
    let number_4 = 1;
    {
        let number_5 = 2;
        println!("{}", number_4 + number_5);
    }
    //println!("{}", number_4 + number_5);//This will fial because number_5 is out of scope
    //Another example on scope variable
    let scope_variable = "outside scope";
    println!("{}", scope_variable);
    {
        let scope_variable = "inside scope";
        println!("{}", scope_variable);
    }
    println!("{}", scope_variable);

    //Math Operators +, -, *, /, %
    let modulus = 10 % 3;
    println!("{}", modulus);

    //Exponent Operations
    let squared = i32::pow(2, 2);
    let float_float = f32::powf(2.0, 2.0);
    let float_integer = f32::powi(2.0, 2);
    println!("{}", squared);
    println!("{}", float_float);
    println!("{}", float_integer);

    //Logical operations
    let is_equal_true = 1 == 1;
    let not_equal_false = 1 == 2;
    let is_not_equal = 1 != 1;
    let greater_than = 1 > 1;
    let less_than = 1 < 1;
    let is_true = true;
    let is_false = !is_true;

    let have_driver_license = false;
    let have_passport = true;
    let have_proof = have_driver_license || have_passport;
    println!("Have proof: {}", have_proof);

    let have_boarding_true = true;
    let have_id = have_proof;
    let can_board = have_boarding_true && have_id;
    println!("Boarding Pass: {} ID:{}", have_boarding_true, have_id);
    println!("Can board: {}", can_board);
    println!("{}", is_equal_true);
    println!("{}", not_equal_false);
    println!("{}", is_not_equal);
    println!("{}", greater_than);
    println!("{}", less_than);
}
