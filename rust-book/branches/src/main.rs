// the if condition has to be a bool

fn main() {
    let number = 3;

    if number < 5 {
        println!("The condition is met")
    }else{
        println!("the condition is not met")
    }

    // let another_numer = 6;

    // rust doesn't automatically converts
    // non boolean type to boolean(like js, ruby does) so we have to be explicit and
    // provide a bool in the condition
    
    // if another_number {
    //     println!("insert you weren't supposed to do this meme...")
    // }

    let dividend = 7;

    if dividend % 4 == 0{
        println!("number is divisible by 4");
    }
    else if dividend % 3 == 0{
        println!("number is divisible by 3");
    }
    else if dividend % 2 == 0{
        println!("number is divisible by 2");
    }
    else{
        println!("The number is not divisible by 4, 3 or 2");
    }

    let condition = false;
    let evaluate_my_value_on_some_condition = if condition {5} else {6};

    let x = evaluate_my_value_on_some_condition;

    println!("The value of x is: {}", x)

    // let var2 = if condition { 5 } else { "six" }; 
    // this will throw an error, since var2 must have a single type, it can't be a integer or string 
    // but any one of them only.
}
