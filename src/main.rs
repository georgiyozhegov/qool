use qoolang::lex::lex;
use qoolang::parse::parse;
use qoolang::generate::*;

fn main()
{
        let source = "a : 12 b : 3 c : (a + b) * 2";
        let source = source.chars().peekable();
        let tokens = lex(source);
        let tokens = tokens.into_iter().peekable();
        let tree = parse(tokens);
        let instructions = generate(tree);
        println!("{instructions:?}");
}
