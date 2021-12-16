use crate::repos::deb::package::DebPackage;

use super::utils::PackageFormat;
use super::config::Config;
use rusqlite::Result;

pub fn list_installed(config: &mut Config) {
	config.setup_db().expect("Failed to open the database");
	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				use super::deb;
				deb::pkg_list(&config)
					.unwrap()
					.into_iter()
					.for_each(|pkg| {
						println!("{} {} - {}", pkg.control.package, pkg.control.version, pkg.control.description)
					});
			},
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
				}
				PackageFormat::Other => {
					println!("Actually we do not have support for you distro!");
				}
			}
	} else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
    }
}

///
/// Find all installed packages and then insert 'em into the database
/// 
//TODO: Make better to read
pub fn populate_db(config: &mut Config) -> Result<()> {
	config.setup_db().expect("Failed to open the database");

	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				use super::deb::{cache, package::PkgKind};

				let pkgs = cache::dpkg_cache_dump(&config); // dump all the installed
				println!("Detected a dpkg database (assuming it's debian)");
				for pkg in pkgs.into_iter().filter_map(|pkg| pkg.ok())  {
					// println!("Dumping: {:#?}", pkg);
					let deb_pkg = DebPackage {
						control: pkg,
						kind: PkgKind::Binary,
					};

					if let Some(_) = config.sqlite.as_ref() {
						cache::add_package(config, deb_pkg, false)?;
					} else {
						println!("Meh");
					}
				};

				Ok(())
			}
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
				Ok(())
			}
			PackageFormat::Other => {
				println!("Actually we do not have support for you distro!");
				Ok(())
			}
		}
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
	}
}

pub fn update_cache(config: &Config) -> Result<()> {
	if let Some(pkg_fmt) = PackageFormat::get_format() {
		match pkg_fmt {
			PackageFormat::Deb => {
				use super::deb::{cache, package::PkgKind};

				let pkgs = cache::cache_dump(&config); // dump all the installed
				println!("Updating the database (assuming it's debian)");
				for pkg in pkgs.into_iter()  {
					let deb_pkg = DebPackage {
						control: pkg,
						kind: PkgKind::Binary,
					};

					if let Some(sqlite) = config.sqlite.as_ref() {
						cache::add_package(config, deb_pkg, true)?;
					}
					// add here
				};

				Ok(())
			}
			PackageFormat::Rpm => {
				println!("It's a RHEL(-based) distro");
				Ok(())
			}
			PackageFormat::Other => {
				println!("Actually we do not have support for you distro!");
				Ok(())
			}
		}
    } else {
        eprintln!("Consider define `PKG_FMT` environment variable!");
        std::process::exit(1);
	}
}

pub fn lookup(config: &Config, name: &str, exact_match: bool,
	cache: bool) {

}