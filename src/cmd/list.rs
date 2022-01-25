use crate::config::Config;

pub fn list(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let paths = config.jvm_paths;
    if paths.len() == 0 {
        println!("You have no paths added!");
    } else {
        println!(
            "You have {} path{} added:",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        );
        for path in paths {
            println!(" - {}", path);
        }
    }
    Ok(())
}
