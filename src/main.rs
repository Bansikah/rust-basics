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
}
