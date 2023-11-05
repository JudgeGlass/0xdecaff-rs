mod tokenizer;

fn main() {
    let expression = "2+2/4";

    let tokens = tokenizer::get_tokens(expression);

    print!("Tokens: ");
    for token in tokens{
        print!("{} ", token);
    }
    println!("");
}
