use crate::config::Config;

pub fn remove(path: &String, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let index = config.jvm_paths.iter().position(|x| x == path);
    match index {
        None => {
            println!("The path {} is already added!", path);
            Ok(())
        }

        Some(idx) => {
            config.jvm_paths.remove(idx);
            confy::store("jvm-manager", config)?;

            println!("Removed path {}!", path);
            Ok(())
        }
    }
}
