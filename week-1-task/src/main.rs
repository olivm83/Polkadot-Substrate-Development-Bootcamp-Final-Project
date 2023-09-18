fn main() {
    let string1 = "I am learning ";
    let string2 = "Rust!";
    
    let concatenated_string = concatenate_strings(string1, string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();  // Create a new empty String
    
    result.push_str(str1);
    result.push_str(str2);
    
    result  // Return the concatenated result
}