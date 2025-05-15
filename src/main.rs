use system::NullnetSystem;

mod rule;
mod system;
mod table;

fn main() {
    let system = match NullnetSystem::new() {
        Ok(system) => system,
        Err(err) => {
            eprintln!("Failed to initialize NullnetSystem: {err}");
            return;
        }
    };

    system.greeting();
}
