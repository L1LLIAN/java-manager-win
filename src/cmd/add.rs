use crate::config::Config;

pub fn add(path: &String, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    if config.jvm_paths.contains(path) {
        println!("The path {} is already added!", path);
        return Ok(());
    }

    config.jvm_paths.push(path.to_string());
    println!("Added path {}!", path);

    confy::store("jvm-manager", config)?;
    Ok(())
}
