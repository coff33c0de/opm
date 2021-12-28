use anyhow::Result;
use std::{path::Path, io::ErrorKind};
use super::{config::Config, utils::PackageFormat};

#[allow(deprecated)]
pub fn setup() -> Result<Config> {
    let home = std::env::home_dir().unwrap()
    .into_os_string().into_string().unwrap();
    let root = format!("{}/.opm/", home);
    let config_file = format!("{}/config.json", root);
    let config ;

    if Path::new(&config_file).exists() {
        config = Config::from(&config_file);
    } else if let Some(fmt) = PackageFormat::get_format() {
        match fmt {
            PackageFormat::Deb => config = Config::new("deb")?,
            PackageFormat::Rpm => panic!("We do not support RPM packages for now ..."),
            PackageFormat::Other => panic!("Unrecognized package"),
        }
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
    }
    
    if !Path::new(&config.root).exists() {
        config.setup()?;
    }

    config.save(&format!("{}/config.json", root));
    Ok(config)
}

#[allow(deprecated)]
pub fn roll_back() {
    println!("Rolling back ...");
    let home = std::env::home_dir().unwrap()
    .into_os_string().into_string().unwrap();
    let root = format!("{}/.opm/", home);

    match std::fs::remove_dir_all(root){
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("Clould not rollback due {}", e)
        }
    }
}