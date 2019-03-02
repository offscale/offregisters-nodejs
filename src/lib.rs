#[macro_use]
extern crate lazy_static;

use failure::Error;

use url::Url;

use offregisters_lib::archive::untar_all_in_dir;
use offregisters_lib::download::download;
use offregisters_lib::env::env_or;
use offregisters_lib::OffRegisters;

mod helpers;

pub struct NodeJs;

impl OffRegisters for NodeJs {
    fn already_setup() -> Result<bool, Error> {
        Ok(false)
    }

    fn pre_install() -> Result<(), Error> {
        let download_dir: std::ffi::OsString = env_or("ASSET_DIR", "assets");
        std::fs::create_dir_all(&download_dir)?;
        download(Some(&download_dir), URLS_V.to_vec(), false)?;
        untar_all_in_dir(&download_dir, Some(&download_dir))?;

        Ok(())
    }

    fn install() -> Result<(), Error> {
        Ok(())
    }

    fn post_install() -> Result<(), Error> {
        Ok(())
    }

    fn uninstall() -> Result<(), Error> {
        Ok(())
    }
}

lazy_static! {
    static ref NODE_VERSION: String = match std::env::var_os("NODE_VERSION") {
        Some(val) => val.to_string_lossy().into(),
        None => String::from("10.15.1"),
    };
    static ref URLS: [&'static str; 4] = [
        Box::leak(
            format!(
                "https://nodejs.org/dist/v{ver}/SHASUMS256.txt",
                ver = *NODE_VERSION
            )
            .into_boxed_str()
        ),
        Box::leak(
            format!(
                "https://nodejs.org/dist/v{ver}/SHASUMS256.txt.asc",
                ver = *NODE_VERSION
            )
            .into_boxed_str()
        ),
        Box::leak(
            format!(
                "https://nodejs.org/dist/v{ver}/SHASUMS256.txt.sig",
                ver = *NODE_VERSION
            )
            .into_boxed_str()
        ),
        Box::leak(
            format!(
                "https://nodejs.org/dist/v{ver}/node-v{ver}-{os}-{arch}.tar.gz",
                ver = *NODE_VERSION,
                os = std::env::consts::OS.replace("macos", "darwin"),
                arch = std::env::consts::ARCH.replace("x86_64", "x64"),
            )
            .into_boxed_str()
        ),
    ];
    static ref URLS_V: Vec<Url> = URLS.iter().map(|url| Url::parse(url).unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use super::{NodeJs, OffRegisters, URLS};

    #[test]
    fn already_setup() {
        assert_eq!(NodeJs::already_setup().unwrap(), false);
    }

    #[test]
    fn pre_install() {
        NodeJs::pre_install().unwrap();
    }

    #[test]
    fn url_parsed() {
        assert_eq!(URLS.len(), 4);
        assert_eq!(URLS[0], "https://nodejs.org/dist/v10.15.1/SHASUMS256.txt");
        assert_eq!(
            URLS[1],
            "https://nodejs.org/dist/v10.15.1/SHASUMS256.txt.asc"
        );
        assert_eq!(
            URLS[2],
            "https://nodejs.org/dist/v10.15.1/SHASUMS256.txt.sig"
        );
        assert_eq!(
            URLS[3],
            "https://nodejs.org/dist/v10.15.1/node-v10.15.1-darwin-x64.tar.gz"
        );
    }
}
