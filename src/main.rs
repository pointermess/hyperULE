mod ast;
mod tokenizer;
mod tokens;
mod analyzer;
mod optimizer;
mod generator;
mod parser;
mod program;
mod ule;

use tokenizer::*;
use tokens::*;
use parser::*;

fn main() {
    let mut tokenizer = Tokenizer::new();

    println!("hyperULE Compiler v0.1");
    println!("----------------------");
    println!("{}", concat!(
    "Compile hyperULE to Datalogic User Label Edit scripts by running this compiler",
    "with a .hule file as first parameter or with the start up parameters ",
    "-i <input file> / -i <input_file> -o <output_file>."));
    println!("");


    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }


        input = input.trim().to_string();

        let mut parser = AstParser::new(&input);
        let ast = parser.parse(&input);

        println!("{:#?}", ast);

        // let source_code = r#"class start comment/* test adasdasas */endcomment Example { int x = 10; }"#;
        // let tokens = tokenizer.tokenize(&input);
        // for mut token in tokens {
        //     let test = token.get_cached_token_type().clone();
        //     println!("{:?} = {:?}", token.value, test);
        // }
    }
}
