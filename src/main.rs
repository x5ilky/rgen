pub mod ast;
pub mod loottable;
use std::{collections::{HashMap, VecDeque}, env::args, fs::{self, read_to_string}};

use ast::Expr;
use grammar::StmtParser;
use lalrpop_util::lalrpop_mod;
use loottable::LootTable;
use text_io::read;

lalrpop_mod!(pub grammar);

struct Context {
    rules: HashMap<String, Expr>,
}

enum RGenError {
    NoRefError(String),
    JoinNotString()
}

fn eval(ctx: &mut Context, expr: Expr) -> Result<Expr, RGenError> {
    match expr {
        Expr::Define { name, options } => {
            ctx.rules.insert(name, *options);
        }
        Expr::String(..) => return Ok(expr),
        Expr::Generate(value) => {
            let out = random(ctx, *value);
            return out;
        }
        Expr::Ref(n) => {
            return match ctx.rules.get(&n) {
                None => Err(RGenError::NoRefError(n)),
                Some(v) => Ok(v.clone()),
            }
        }
        Expr::List(..) => {}
        Expr::Display(value) => {
            println!("{}", eval(ctx, *value)?);
        }
        Expr::Weighted { value, .. } => return eval(ctx, *value),
        Expr::Null => {},
        Expr::Join { left, right } => {
            let left_str = eval(ctx, *left)?;
            let right_str = eval(ctx, *right)?;
            
            if let Expr::String(l) = left_str {
                if let Expr::String(r) = right_str {
                    return Ok(Expr::String(l + &r));
                }
            }
            return Err(RGenError::JoinNotString())
        }
    }
    Ok(Expr::Null)
}

fn random(ctx: &mut Context, expr: Expr) -> Result<Expr, RGenError> {
    match expr {
        Expr::Define { options, .. } => Ok(*options),
        Expr::Generate(..) => Ok(Expr::Null),
        Expr::List(values) => {
            let mut lt = LootTable::new(vec![]);
            for value in values {
                match value {
                    Expr::Weighted { value, weight } => {
                        lt.push(*value, weight);
                    }
                    _ => lt.push(value, 1),
                }
            }
            return if lt.total() > 0 {
                Ok(
                    eval(ctx, lt.random().clone())?
                )
            } else {
                Ok(Expr::Null)
            };
        }
        Expr::Ref(..) => {
            let value = eval(ctx, expr)?;
            return random(ctx, value);
        }
        Expr::String(..) => Ok(expr),
        Expr::Display(..) => Ok(Expr::Null),
        Expr::Weighted { value, .. } => Ok(*value),
        Expr::Null => Ok(Expr::Null),
        Expr::Join { .. } => Ok(Expr::Null)
    }
}

fn run_line(ctx: &mut Context, src: &str, defs: &mut Vec<String>) {
    
        let parser = StmtParser::new();
        let result = parser.parse(src);
        match result {
            Ok(v) => {
                match v {
                    Expr::Define { .. } => {
                        defs.push(src.to_string())
                    },
                    _ => {}
                }
                if let Err(e) = eval(ctx, v.clone()) {
                    match e {
                        RGenError::NoRefError(var_name) => {
                            eprintln!("No ref called {var_name}!");
                        }
                        RGenError::JoinNotString() => {
                            eprintln!("Join operator didnt get strings as both sides");
                        }

                    }
                };
            }
            Err(e) => {
                eprintln!("Error: {}", e.to_string());
            }
        }
}
fn main() {
    let mut argv: VecDeque<String> = args().collect();
    let _program = argv.pop_front().unwrap();
    let take_path = argv.pop_front();
    let mut ctx = Context {
        rules: HashMap::new(),
    };
    if let Some(path) = take_path {
        let f = fs::read_to_string(path).expect("Couldn't open file");
        for ln in f.lines() {
            let s = ln.trim();
            if !s.is_empty() {
                run_line(&mut ctx, s, &mut vec![]);
            }
        }
    }
    let mut defs = vec![];
    'run: loop {
        print!("rgen> ");
        let src: String = read!("{}\n");
        let src = src.trim();
        if src == "dump" {
            fs::write("dump.rgen", defs.join("\n")).expect("Failed to dump to dump.rgen");
            continue 'run;    
        }
        if src == "exit" {
            break 'run;
        }
        run_line(&mut ctx, src, &mut defs)
    }
}
