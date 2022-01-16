//! Implementation of the Event Handler With State Pattern - Main Style
use rhai::{Dynamic, Engine, Scope, AST};

use std::io::{stdin, stdout, Write};

const SCRIPT_FILE: &str = "event_handler_main/script.rhai";

#[derive(Debug)]
struct Handler {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub ast: AST,
}

fn print_scope(scope: &Scope) {
    scope
        .iter_raw()
        .enumerate()
        .for_each(|(i, (name, constant, value))| {
            println!(
                "[{}] {}{}{} = {:?}",
                i + 1,
                if constant { "const " } else { "" },
                name,
                if value.is_shared() { " (shared)" } else { "" },
                *value.read_lock::<Dynamic>().unwrap(),
            )
        });
    println!();
}

pub fn main() {
    println!("Events Handler Example - Main Style");
    println!("===================================");

    let mut input = String::new();

    // Read script file
    print!("Script file [{}]: ", SCRIPT_FILE);
    stdout().flush().expect("flush stdout");

    input.clear();

    stdin().read_line(&mut input).expect("read input");

    let path = match input.trim() {
        "" => SCRIPT_FILE,
        path => path,
    };

    // Create Engine
    let engine = Engine::new();

    // Create a custom 'Scope' to hold state
    let mut scope = Scope::new();

    // Add any system-provided state into the custom 'Scope'.
    // Constants can be used to optimize the script.
    scope.push_constant("MY_CONSTANT", 42_i64);

    // Compile the handler script.
    println!("> Loading script file: {}", path);

    let ast = match engine.compile_file_with_scope(&mut scope, path.into()) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("! Error: {}", err);
            println!("Cannot continue. Bye!");
            return;
        }
    };

    println!("> Script file loaded.");
    println!();
    println!("quit      = exit program");
    println!("scope     = print scope");
    println!("event arg = run function with argument");
    println!();

    // Run the 'init' function to initialize the state, retaining variables.
    let result = engine.call_fn_raw(&mut scope, &ast, false, false, "init", None, []);

    if let Err(err) = result {
        eprintln!("! {}", err)
    }

    // Create handler instance
    let mut handler = Handler { engine, scope, ast };

    // Events loop
    loop {
        print!("event> ");
        stdout().flush().expect("flush stdout");

        // Read event
        input.clear();
        stdin().read_line(&mut input).expect("read input");

        let mut fields = input.trim().splitn(2, ' ');

        let event = fields.next().expect("event").trim();
        let arg = fields.next().unwrap_or("");

        // Process event
        match event {
            "quit" => break,

            "scope" => {
                print_scope(&handler.scope);
                continue;
            }

            // Map all other events to function calls
            _ => {
                let engine = &handler.engine;
                let scope = &mut handler.scope;
                let ast = &handler.ast;

                let result: Result<(), _> = engine.call_fn(scope, ast, event, (arg.to_string(),));

                if let Err(err) = result {
                    eprintln!("! {}", err)
                }
            }
        }
    }

    println!("Bye!");
}
