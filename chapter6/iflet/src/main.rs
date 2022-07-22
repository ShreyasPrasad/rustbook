fn usingiflet(){
    // instead of having catchall arms in a match statement, we can use if let for more concise control flow
    let config_max = Some(3u8);
    /*
        The syntax if let takes a pattern and an expression separated by an equal sign.
        The code in the if let block isn’t run if the value doesn’t match the pattern.
        However, you lose the exhaustive checking that match enforces. It is a tradeoff.
        Conciseness vs exhaustiveness.
    */
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn main() {
    println!("Hello, world!");
}
