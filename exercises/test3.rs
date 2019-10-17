// strings3.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // Standard initialisation
    string_slice("blue");

    // Creates a String
    string("red".to_string());

    // Creates a String
    string(String::from("hi"));

    // Turns the borrowed str into owned, a String
    string("rust is fun!".to_owned());

    // With '.into()' you can convert values to String
    string_slice("nice weather".into());

    // Returns a String
    string(format!("Interpolation {}", "Station"));

    // Creates a String with '::from' but then creates a slice from it
    string_slice(&String::from("abc")[0..1]);

    // Only trims whitespace before and after str
    string_slice("  hello there ".trim());

    // Creates a String first, then modifies it
    string("Happy Monday!".to_string().replace("Mon", "Tues"));

    // Returns lowercase of str as String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
