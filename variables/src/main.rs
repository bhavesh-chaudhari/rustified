// in rust, variables are immutable by default
// constants are always immutable, variables can be muted.
/* 
rust's naming convention for constants is to use all
uppercase with underscores between words.
syntax :-
const DAYS_IN_WEEK: u32 = 7;
we can also write like this(the value will be calculated at compile time) :-
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
we must always provide a type for the constant like we provided u32 above.
*/

fn main() {
    // instead of assigning FACTORIAL_OF_FOUR as 24 we can also perform operations
    // like 1 * 2 * 3 * 4 as it makes easier to understand and verify the assignment, 
    // final value is calculated at compile time.
    const FACTORIAL_OF_FOUR: u32 = 1 * 2 * 3 *4; 
    println!("The value of 4! is: {}",FACTORIAL_OF_FOUR);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing - declaring a new variable with the same name as
    // a previous variable
    let y = 5;

    let y = y + 1;

    {
        // this is inner scope
        let y = y * 3;
        println!("The value of x in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y)
    // shadowing and marking a variable mut is different.
    //  By using let, we can perform a few transformations on a value but have 
    // the variable be immutable after those transformations have been completed
    // we cannot mutate a mutable variable's type but with shadowing we can do that since 
    // we are essentially creating a new variable with the same name by using the `let` keyword.
    /*

    This wouldn't compile -
    let mut spaces = "   "; //string
    spaces = spaces.len(); //number

    This would compile -
    let spaces = "   "; //string
    let spaces = spaces.len(); //number
    */
}