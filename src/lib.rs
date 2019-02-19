#[macro_use]
extern crate lazy_static;

use failure::Error;

use offregisters_lib::OffRegisters;

mod helpers;

pub struct NodeJs;

impl OffRegisters for NodeJs {
    fn already_setup() -> Result<bool, Error> {
        Ok(false)
    }

    fn pre_install() -> Result<(), Error> {
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
    static ref URLS: [&'static str; 1] = [Box::leak(
        format!(
            "https://nodejs.org/dist/v{ver}/node-v{ver}-{os}-{arch}.tar.gz",
            ver = *NODE_VERSION,
            os = std::env::consts::OS.replace("macos", "darwin"),
            arch = std::env::consts::ARCH.replace("x86_64", "x64"),
        )
        .into_boxed_str()
    )];
}

#[cfg(test)]
mod tests {
    use super::{NodeJs, OffRegisters, URLS};

    #[test]
    fn already_setup() {
        assert_eq!(NodeJs::already_setup().unwrap(), false);
    }

    #[test]
    fn url_parsed() {
        for url in URLS.iter() {
            assert_eq!(
                url,
                &"https://nodejs.org/dist/v10.15.1/node-v10.15.1-darwin-x64.tar.gz"
            );
        }
    }
}
