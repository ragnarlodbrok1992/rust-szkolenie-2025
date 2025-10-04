// This is example that shows what empty slices can be used for.

fn parse_input(input: &[u8]) -> (&[u8], &[u8]) {
    match input {
        [] => (&[], input),
        _ => {
            // Do some work with the value stored in _
            // Returning parsed for printing out the value
            (&input[0..1], &input[1..])
        }
    }
}

fn main() {
    // Defining some code to be parsed byte-by-byte
    let bytes = &[0x12, 0x13, 0xFF, 0xBA, 0xBE];

    // let mut input: &[u8] = &bytes; // Uncomment this to see an error
    let mut input: &[u8] = &bytes[..]; // Converting array to slice using slice syntax
                                       // This allows us to dropping requirements in pattern
                                       // matching for number of elements

    loop {
        match input {
            [] => break,
            _ => {
                let (parsed, remaining) = parse_input(input);
                input = remaining; // Moving the rest of the code as an input to another loop of
                                   // parsing
                // Printing
                println!("Parsed: {:02X?}, Remaining: {:02X?}", parsed, remaining);
            }
        }
    }
}
