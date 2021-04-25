
fn main(){
    // even();
    // fizz_buzz();
    fizz_buzz_evolusion();
}

fn even(){
    let even = |x| if x%2 == 0 { return "even" } else { "odd" };
    let result = (0..100).map(|i| even(i));
    for i in result {
        println!("{}", i);
    }
}

fn fizz_buzz(){
    const FIZZ: &str = "Fizz";
    const BUZZ: &str = "Buzz";
    for i in 0..100 {
        if i % 15 == 0 {
            println!("{}", FIZZ.to_string()+BUZZ);
        } else if i % 5 == 0 {
            println!("{}", BUZZ);
        } else if i % 3 == 0 {
            println!("{}", FIZZ);
        } else {
            println!("{}", i);
        }
    }
}

fn fizz_buzz_evolusion(){
    let fz = |x: u8| {
        match (x%3, x%5){
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => x.to_string(),
        }};
    (0..100).map(fz).for_each(|x| println!("{}", x));
}