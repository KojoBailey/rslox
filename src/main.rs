mod tokenizer;
mod parser;

fn main() {
    let input = "\"hello\" + 5.4 + true == 2";

    let tokens = match tokenizer::tokenize(input) {
        Ok(v) => v,
        Err(errors) => {
            for err in errors {
                eprintln!("{}\n", err);
            }
            return;
        },
    };
    println!("== Tokens ==\n{:#?}\n", tokens);

    let ast = match parser::parse(tokens) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}\n", err);
            return;
        }
    };
    println!("== AST ==\n{:#?}\n", ast);
}
