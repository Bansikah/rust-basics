#![allow(unused_variables)]
#![allow(unused_assignments)]
//enums are created using the "enum "  keyword

// #[repr(u8)]
// enum NavigationAids {
//     NDB = 3,
//     VOR = 2,
//     VORDME = 5,
//     FIX {
//         name: String,
//         lat: f32,
//         lng: f32
//     }
// }
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

    //Bitwise operators & and |, ^
    let bitwise_and = 87 & 67;
    let bitwise_or = 87 | 67;
    println!("Bitwise and: {}", bitwise_and);
    println!("Bitwise or: {}", bitwise_or);

    let bitwise_xor = 87 ^ 67;
    println!("Bitwise xor: {} ", bitwise_xor);

    //shift operators
    let left_shift = 87<<1;//shift one place to the left
    println!("Shift to the left: {}", left_shift);

    let right_shift = 87>>1;//shift one place to the right
    println!("Shift to the right: {}", right_shift);

    //if else and if statements
    let word = "Dog";
    if word == "Duck" {
        println!("Quack!");
    }else if word == "Cat" {
        println!("Meow!");
    }else if word == "Dog" {
        println!("Woof!");
    }else if word == "Cow" {
        println!("Moo!");
    }else if word == "Horse" {
        println!("Neigh!");
    }else{
        println!("I don't know what to say!");
        
    }

    //Another example using logical operators
    let available_craft = "Boeng";
    let minimum_crow = 7;
    let available_crow = 4;

    if available_craft == "Boeng" || available_craft == "AirBus" && available_crow >= minimum_crow {
        println!("You can fly!");
    } else{
        println!("You can't fly!");
    }

    //  ENUMS
//    println!("NDB: \t{}", NavigationAids::NDB as u8);
//     println!("VOR: \t{}", NavigationAids::VOR as u8);
//     println!("VORDME: \t{}", NavigationAids::VORDME as u8);
//     println!("FIX: \t{}", NavigationAids::FIX as u8);
//     println!("FIX: \t{}", NavigationAids::FIX { name: "test".to_string(), lat: 0.0, lng: 0.0} as u8);

    //Option enumurations : it is an enum which has two values
    let phrase = String::from("Durk Airlines");
    let letter = phrase.chars().nth(3);
    match letter {
        Some(c) => println!("The 4th letter is {}", c),
        None => println!("No 4th letter")
    }

    //Match Statements
    let animal = "Horse";
    match animal {
        "Duck" => println!("Quack!"),
        "Cat" => println!("Meow!"),
        "Dog" => println!("Woof!"),
        "Cow" => println!("Moo!"),
        "Horse" => println!("Neigh!"),
        _ => println!("I don't know what to say!")
    }

    //if let 
    // let person = "noel";

    // if let person = "noel"{
    //     println!("Hello, {}!", person);
    // }

    //Looops in rust
    loop{
let mut counter = 0;
counter += 1;
println!("{}", counter);
if counter == 10{
    break;
}
}

//While  loops
let mut counter2 = 0;
counter2 += 1;
while counter2 < 10{
    println!("{}", counter2);
    break;
}

//for loops
let aircraft_array = ["Boeng 346", "usa 234", "Boeng 346", "china 2345"];
for aircraft in aircraft_array{
    println!("{}", aircraft);
}

//ownership and cloning

 let original = String::from("Original Value");
 println!("\nOrigin value is:{}", original);

 let original = String::from("Another Value");
 let copy = original.clone();
 println!("\nOrigin value is:{}", original);
 println!("\nCopy value is:{}", copy);


 //Borrowing: Allow another variable to take temporal ownership of data without deallocating the origal value

 let orignal = String::from("Example original");
 println!("\nThe original value is: {}", orignal);

 {
    let next = &orignal;
    println!("\nThe next value is: {}", next);
    
 }
 println!("\nThe original value is: {}", orignal);


 //life time, applies to items only stored in a heap
 let reference_int = 6;
 let return_value = return_one_parameter(&reference_int);
 println!("The return value is: {}", return_value);

 let value_one = 45;
 let value_two = 30;
 let value = explicit_lifetme(&value_one, &value_two);

}

fn return_one_parameter(value: &i32) -> &i32{
    value
}
//Printing the greater functions examples
let greater = return_greater(10, 5);
println!("{}", greater);



//example of borrowing with functions
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: 5 };

    let distance = distance(&p1, &p2);
    println!("Distance between points: {}", distance);

    let mut p3 = Point { x: 3, y: 4 };
    scale_point(&mut p3, 2.0);
    println!("Point after scaling: ({}, {})", p3.x, p3.y);


//Closures: closures are functions without names
let name = "Durk Airlines"

let write_message = |slogan String| -> String{
String::from(format!("{} .{}", name, slogan))
};
let phrase = write_message(String::from("hello this is Noel Draxler"));
println!("{}", phrase);
}
//life time example two
fn expelicit_lifetme<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32{
    if p1 > p2 {
        p1
    }else {
        p2
    }

  //Functions: a function is a block of code that can be resused

fn return_greater(first: u8, second: u8) -> u8 {
if first > second {
first
}else {
second
}
}
