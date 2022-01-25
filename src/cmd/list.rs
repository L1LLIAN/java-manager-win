use crate::config::Config;

pub fn list(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let paths = config.jvm_paths;
    if paths.len() == 0 {
        println!("You have no paths added!");
    } else {
        println!(
            "You have {} path{} added: (index, path)",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        );

        let mut i = 0;
        for path in paths {
            println!("{} - {}", i, path);
            i = i + 1;
        }
    }
    Ok(())
}
