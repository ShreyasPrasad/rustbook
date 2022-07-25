use std::collections::HashMap;

fn hashmaps(){
    /*
        All hashmap keys must have same type and all values must have the same type.
    */
    let mut scores = HashMap::new(); 
    scores.insert(String::from("Blue"), 10);

    /*
        Another way to construct a hashmap is using iterators and the collect method
        on a vector of tuples. Rust infers type of the hashmap key/value.
    */
    let teams = vec![String::from("first"), String::from("second")];
    let initial_scores = vec![5,6];
    let mut scores_iter: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    /*
        Inserting a new item
    */
    let field_name = String::from("field");
    let field_value = String::from("value");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field name and value are moved; original vars no longer valid


    /* 
        Accessing an existing item
    */
    let team_name = String::from("Blue");
    let team_value = scores.get(&team_name); // get returns an Option<&V> where V is the value type

    /*
        Inserting the same key value pair overwrites the original key value pair in the same hashmap.
        Otherwise, we can use the entry method on the HashMap API for conditional insertion.
    */
    scores.entry(String::from("Yellow")).or_insert(50); // inserts Yellow, 50 only if "Yellow" doesn't exist in hashmap
}

fn main() {
    hashmaps();
}
