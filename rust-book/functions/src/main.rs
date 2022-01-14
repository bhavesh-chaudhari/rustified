// rust doesn't care about where we define our functions, it only cares about 
// whether they are declared somewhere or not.
// by default the last expression is the return value of a rust function
// we can return early by using the return keyword
// we have to provide typename to function parameters, we can pass 
// multiple paramters by separating them with commas
// statement doesn't return anything whereas an expression does return an abosulte value
// expressions don't include ending semicolons and they can be part of a statement.

fn main() {
    another_function(10);
}

fn multiply_two_numbers(num1: i32, num2: i32){
    println!("The value of {} X {} is: {}", num1, num2, num1*num2)
}

fn six() -> i32 {
    6 // if we add a semicolon here then it will become a statement
      // and therefore throw an error 
} 

fn another_function(y: i32) {
    println!("The value of y is: {}", y);

    let x = six();

    println!("The value of x is: {}", x);

    multiply_two_numbers(36, 36);

    let z = {
        let x = 4;
        x + 1
    };

    println!("The value of z is : {}", z)
}

