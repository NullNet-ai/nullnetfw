use system::NullnetSystem;
use validator::Validator;

mod chain;
mod rule;
mod system;
mod table;
mod validator;

fn main() {
    let system = match NullnetSystem::new() {
        Ok(system) => system,
        Err(err) => {
            eprintln!("Failed to initialize NullnetSystem: {err}");
            return;
        }
    };

    system.greeting();

    let _ = Validator::new(system);
}
