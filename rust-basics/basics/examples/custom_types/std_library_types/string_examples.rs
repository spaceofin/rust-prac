pub fn string_basic_examples() {
    let greeting: &str = "Hello, world!";
    println!("greeting: {} - immutable str(&str)", greeting);

    let mut mutable_greeting = String::from("Hello");
    println!("mutable_greeting: {} - mutable String", mutable_greeting);

    mutable_greeting.push_str(", world!"); 
    println!("mutable_greeting: {} - mutable String", mutable_greeting);
}

pub fn string_examples() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram - immutable: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    print!("Words in reverse > ");
    for word in pangram.split_whitespace().rev() {
        print!("{} ", word);
    }
    println!();

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    println!("chars(Vec<char>) - mutable:\n{:?}",chars);
    chars.sort();
    println!("sorted chars:\n{:?}",chars);
    chars.dedup();
    println!("de-duplicated chars:\n{:?}",chars);

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }
    println!("string - heap allocated & mutable:\n{}",string);

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters - immutable: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {} - heap allocated & immutable", alice);
    println!("Bob says: {} - heap allocated & immutable", bob);
}