use std::io ;  // to obtain user input and then print the result as output, we need to bring the io library into scope
use std::cmp::Ordering ;  // used for comparing two values
use rand::Rng ;  // Rng trait defines methods that random numbers generators implement, and this trait must be in scope for us to use those methods

fn main() {
    println!( "Guess the number!" ) ;

    let secret_number = rand::thread_rng( ).gen_range( 1..=100 ) ;
    // rand::thread_rng function that gives us the particular random generator
    // .gen_range(1..=100) takes a range expression as an argument and fenerates a random number in the range

    loop {  // loops forever until break
        println!( "Please input your guess." ) ;

        let mut guess = String::new( ) ;
        // create a variable guess by using let
        // in rust variables are immutable by default, use mut to set mutable variables
        // String::new( ) function that returns a new instance of a String

        io::stdin( )
            .read_line( &mut guess )
            .expect( "Failed to read line" ) ;
        // io::stdin( ) requests the user to input a something
        // .readline( &mut guess ) calls the read_line method on the standard input handle, guess is being used to store the user inout
        // .expect( "Failed to read line" ) is used to crash the program and display the message you have passed as an argument to expect

        let guess: u32 = match guess.trim( ).parse( ) {
            Ok( num ) => num,
            Err( _ ) => continue,
        } ;
        // trim method will eliminate any whitespace at the beginning and end
        // parse converts a string to another type

        println!( "You guessed: {guess}" ) ;
        // prints the string that now contains the user's input
        // {} is a placeholder, 

        match guess.cmp( &secret_number ) {
            Ordering::Less => println!( "Too Small!" ),
            Ordering::Greater => println!( "Too Big!" ),
            Ordering::Equal => {
                println!( "You win!" ) ;
                break ;  // breaks out of loop
            },
        }
        // cmp method compares two values and can be called on anything that can be compared
        // use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp
    }
}
