use crate::config::Config;

pub fn add(path: &String, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut final_path = path.clone();
    if !final_path.ends_with('\\') {
        final_path.push('\\');
    }

    if config.jvm_paths.contains(&final_path) {
        println!("The path {} is already added!", path);
        return Ok(());
    }

    config.jvm_paths.push(final_path);
    println!("Added path {}!", path);

    confy::store("jvm-manager", config)?;
    Ok(())
}
