//! This is a small example program that evaluates an expression and returns
//! the result on stdout in JSON format.  The values provided to the script
//! are the environment variables in the `env` dict.
use minijinja::Environment;
use std::collections::BTreeMap;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage: expr <expression>");
        std::process::exit(1);
    }

    let env = Environment::new();
    let expr = env.compile_expression(&args[1]).unwrap();
    let mut ctx = BTreeMap::new();
    ctx.insert("env", std::env::vars().collect::<BTreeMap<_, _>>());
    let result = expr.eval(&ctx).unwrap();
    let serialized = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", serialized);
}
