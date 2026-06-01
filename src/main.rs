mod tokenizer;

fn main() {
    let tokens = tokenizer::tokenize("(+*) {-!} 5.34 + 93. - foo and varif kojo // This is a comment!\n\t!= \"kojo bailey\"");
    println!("{:#?}", tokens);
}
