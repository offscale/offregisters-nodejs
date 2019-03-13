extern crate serde;

extern crate semver;

use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

use serde::de;
use serde::de::{Deserializer, Visitor};
use serde::{Deserialize, Serialize};

use url::Url;

use semver::Version as SemverVersion;

use offregisters_lib::download::download;

const VERSIONS_URL: &'static str = "https://nodejs.org/dist/index.json";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    #[serde(deserialize_with = "bool_or_string")]
    pub lts: String,
}

fn bool_or_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = std::string::ParseError>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct BoolOrString<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for BoolOrString<T>
    where
        T: Deserialize<'de> + FromStr<Err = std::string::ParseError>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("bool or string")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(if value { "true" } else { "false" })
            // Err(Error::invalid_type(Unexpected::Bool(value), &self))
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }
    }

    deserializer.deserialize_any(BoolOrString(PhantomData))
}

lazy_static! {
    pub static ref VERSIONS: Vec<Version> = || -> Vec<Version> {
        if cfg!(test) {
            serde_json::from_reader(BufReader::new(
                File::open(Path::new("mocks").join("index.json")).unwrap(),
            ))
            .unwrap()
        } else {
            let url: Url = Url::parse(VERSIONS_URL).unwrap();
            serde_json::from_slice(
                &*download(None as Option<std::ffi::OsString>, vec![url.clone()], false).unwrap()
                    [&url]
                    .raw
                    .clone()
                    .unwrap(),
            )
            .unwrap()
        }
    }();
}

pub fn filter_versions(filter: &str) -> impl Iterator<Item = &Version> {
    VERSIONS.iter().filter(move |version: &&Version| {
        if filter == "lts" {
            version.lts != "false"
        } else {
            version.version == filter
        }
    })
}

pub fn highest_version(versions: Vec<&Version>) -> &Version {
    versions
        .iter()
        .fold(versions[0].clone(), |previous, current| {
            if SemverVersion::parse(&previous.version) < SemverVersion::parse(&current.version) {
                current
            } else {
                previous
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_versions() {
        let lts_versions: Vec<&String> = filter_versions("lts")
            .map(move |version| &version.version)
            .collect();
        assert_eq!(
            lts_versions,
            vec![
                "v10.15.1", "v10.15.0", "v10.14.2", "v10.14.1", "v10.14.0", "v10.13.0", "v8.15.0",
                "v8.14.1", "v8.14.0", "v8.13.0", "v8.12.0", "v8.11.4", "v8.11.3", "v8.11.2",
                "v8.11.1", "v8.11.0", "v8.10.0", "v8.9.4", "v8.9.3", "v8.9.2", "v8.9.1", "v8.9.0",
                "v6.16.0", "v6.15.1", "v6.15.0", "v6.14.4", "v6.14.3", "v6.14.2", "v6.14.1",
                "v6.14.0", "v6.13.1", "v6.13.0", "v6.12.3", "v6.12.2", "v6.12.1", "v6.12.0",
                "v6.11.5", "v6.11.4", "v6.11.3", "v6.11.2", "v6.11.1", "v6.11.0", "v6.10.3",
                "v6.10.2", "v6.10.1", "v6.10.0", "v6.9.5", "v6.9.4", "v6.9.3", "v6.9.2", "v6.9.1",
                "v6.9.0", "v4.9.1", "v4.9.0", "v4.8.7", "v4.8.6", "v4.8.5", "v4.8.4", "v4.8.3",
                "v4.8.2", "v4.8.1", "v4.8.0", "v4.7.3", "v4.7.2", "v4.7.1", "v4.7.0", "v4.6.2",
                "v4.6.1", "v4.6.0", "v4.5.0", "v4.4.7", "v4.4.6", "v4.4.5", "v4.4.4", "v4.4.3",
                "v4.4.2", "v4.4.1", "v4.4.0", "v4.3.2", "v4.3.1", "v4.3.0", "v4.2.6", "v4.2.5",
                "v4.2.4", "v4.2.3", "v4.2.2", "v4.2.1", "v4.2.0"
            ]
        );

        let onetime_lts: Vec<&Version> = filter_versions("v10.15.1").collect();

        assert_eq!(
            onetime_lts[0],
            &Version {
                version: String::from("v10.15.1"),
                date: String::from("2019-01-29"),
                files: vec![
                    String::from("aix-ppc64"),
                    String::from("headers"),
                    String::from("linux-arm64"),
                    String::from("linux-armv6l"),
                    String::from("linux-armv7l"),
                    String::from("linux-ppc64le"),
                    String::from("linux-s390x"),
                    String::from("linux-x64"),
                    String::from("osx-x64-pkg"),
                    String::from("osx-x64-tar"),
                    String::from("src"),
                    String::from("sunos-x64"),
                    String::from("win-x64-7z"),
                    String::from("win-x64-exe"),
                    String::from("win-x64-msi"),
                    String::from("win-x64-zip"),
                    String::from("win-x86-7z"),
                    String::from("win-x86-exe"),
                    String::from("win-x86-msi"),
                    String::from("win-x86-zip")
                ],
                npm: Some(String::from("6.4.1")),
                v8: String::from("6.8.275.32"),
                uv: Some(String::from("1.23.2")),
                zlib: Some(String::from("1.2.11")),
                openssl: Some(String::from("1.1.0j")),
                modules: Some(String::from("64")),
                lts: String::from("Dubnium"),
            }
        );

        let all_lts_versions: Vec<&Version> = filter_versions("lts").collect();

        assert_ne!(
            all_lts_versions[0],
            &Version {
                date: String::new(),
                lts: String::new(),
                files: vec![],
                npm: None,
                modules: None,
                openssl: None,
                uv: None,
                v8: String::new(),
                version: String::new(),
                zlib: None,
            }
        );

        assert_eq!(
            highest_version(all_lts_versions),
            &Version {
                version: String::from("v10.15.1"),
                date: String::from("2019-01-29"),
                files: vec![
                    String::from("aix-ppc64"),
                    String::from("headers"),
                    String::from("linux-arm64"),
                    String::from("linux-armv6l"),
                    String::from("linux-armv7l"),
                    String::from("linux-ppc64le"),
                    String::from("linux-s390x"),
                    String::from("linux-x64"),
                    String::from("osx-x64-pkg"),
                    String::from("osx-x64-tar"),
                    String::from("src"),
                    String::from("sunos-x64"),
                    String::from("win-x64-7z"),
                    String::from("win-x64-exe"),
                    String::from("win-x64-msi"),
                    String::from("win-x64-zip"),
                    String::from("win-x86-7z"),
                    String::from("win-x86-exe"),
                    String::from("win-x86-msi"),
                    String::from("win-x86-zip")
                ],
                npm: Some(String::from("6.4.1")),
                v8: String::from("6.8.275.32"),
                uv: Some(String::from("1.23.2")),
                zlib: Some(String::from("1.2.11")),
                openssl: Some(String::from("1.1.0j")),
                modules: Some(String::from("64")),
                lts: String::from("Dubnium"),
            }
        );

        let versions: Vec<&Version> = VERSIONS.iter().map(|v| v).collect();

        assert_eq!(
            highest_version(versions),
            &Version {
                version: String::from("v11.10.0"),
                date: String::from("2019-02-14"),
                files: vec![
                    String::from("aix-ppc64"),
                    String::from("headers"),
                    String::from("linux-arm64"),
                    String::from("linux-armv6l"),
                    String::from("linux-armv7l"),
                    String::from("linux-ppc64le"),
                    String::from("linux-s390x"),
                    String::from("linux-x64"),
                    String::from("osx-x64-pkg"),
                    String::from("osx-x64-tar"),
                    String::from("src"),
                    String::from("sunos-x64"),
                    String::from("win-x64-7z"),
                    String::from("win-x64-exe"),
                    String::from("win-x64-msi"),
                    String::from("win-x64-zip"),
                    String::from("win-x86-7z"),
                    String::from("win-x86-exe"),
                    String::from("win-x86-msi"),
                    String::from("win-x86-zip")
                ],
                npm: Some(String::from("6.7.0")),
                v8: String::from("7.0.276.38"),
                uv: Some(String::from("1.26.0")),
                zlib: Some(String::from("1.2.11")),
                openssl: Some(String::from("1.1.1a")),
                modules: Some(String::from("67")),
                lts: String::from("false"),
            }
        );
    }
}
