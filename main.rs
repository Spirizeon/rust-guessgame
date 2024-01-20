use std::io;

fn main() {
    println!("Jaiwanth had 3 laptops, he gave one away");
    println!("Please enter how many he has now: ");
/* create a new input string
 * read the input in a line, remove line spaces
 * save in variable, convert to integer
*/
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); 

    let num: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    if num == 2 {
        println!("Correct!");
    } else {
        println!("Wrong answer.");
    }
}

