mod tokenizer;

fn main() {
    let tokens = tokenizer::tokenize("+%\n!=\n$");
    println!("{:#?}", tokens);
}
