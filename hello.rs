
fn main(){
    fizz_buzz()
}

fn fizz_buzz(){
    const fizz: &str = "Fizz";
    const buzz: &str = "Buzz";
    for i in 0..100 {
        if i % 15 == 0 {
            println!("{}", fizz.to_string()+&buzz);
        } else if i % 5 == 0 {
            println!("{}", buzz);
        } else if i % 3 == 0 {
            println!("{}", fizz);
        } else {
            println!("{}", i);
        }
    }
}