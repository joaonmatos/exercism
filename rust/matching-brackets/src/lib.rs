pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::with_capacity(string.len());
    for ch in string.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                if let Some(pair) = stack.pop() {
                    if matching_bracket(pair) != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        };
    }
    stack.len() == 0
}

fn matching_bracket(bracket: char) -> char {
    match bracket {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        _ => unreachable!("should not be called on any other chars"),
    }
}
