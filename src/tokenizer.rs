const DELIMITERS: &'static str = "+-/*()^,&|";

pub fn get_tokens(str_expression: &str) -> Vec<String> {
    let expression = String::from(str_expression).replace(" ", "");
    
    let mut tokens: Vec<String> = Vec::new();

    let mut last_found = 0;

    for index in 0..expression.len(){
        let current_char = expression.chars().nth(index).unwrap();

        if is_delim(current_char){
            let t = String::from(&expression[last_found .. index]);
            if t.len() != 0 && t != "" {
                tokens.push(t);
            }

            tokens.push(String::from(current_char));
            last_found = index + 1;
        }
    }

    tokens.push(String::from(expression.chars().nth(last_found).unwrap()));

    tokens
}

fn is_delim(character: char) -> bool {
    String::from(DELIMITERS).contains(character)
}