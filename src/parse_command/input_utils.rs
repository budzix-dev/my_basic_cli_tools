pub fn split_input_outside_quotes_on_whitespace(input: String) -> Vec<String> {
    split_input_outside_quotes(input, ' ')
}

fn split_input_outside_quotes(input: String, delimiter: char) -> Vec<String> {
    let mut output = Vec::new();
    let mut current = String::new();
    let mut inside_quotes = false;

    for c in input.chars() {
        if c == '"' {
            inside_quotes = !inside_quotes;
            continue;
        }

        if c == delimiter && !inside_quotes {
            if current.is_empty() {
                continue;
            }

            output.push(current);
            current = String::new();
            continue;
        }

        current.push(c);
    }

    output.push(current);
    output
}
