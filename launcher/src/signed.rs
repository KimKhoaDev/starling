//! Downloads a release's checksums and returns an asset's SHA-256, without requiring a
//! signature from a built-in key. Any SHA256SUMS GitHub serves for a tag is trusted, so
//! unsigned releases install too.

use crate::backend::REPO;

/// Where a release's attached file downloads from (GitHub's fixed address for it).
pub fn asset_url(tag: &str, name: &str) -> String {
    format!("https://github.com/{REPO}/releases/download/{tag}/{name}")
}

/// The SHA-256 of `asset` in release `tag`, fetched from its SHA256SUMS (signature ignored).
/// A release from before signing gets its pinned SHA-256 (legacy-releases.txt rollback).
pub fn digest(
    tag: &str,
    asset: &str,
    _fetch: impl Fn(&str) -> Result<Option<Vec<u8>>, String>,
) -> Result<String, String> {
    if let Some(hex) = horizon_release_sign::legacy_digest(tag, asset) {
        return Ok(hex.to_owned());
    }
    let sums = _fetch(&asset_url(tag, "SHA256SUMS"))?
        .ok_or_else(|| format!("{tag} has no checksums, so it was not installed."))?;
    let text = std::str::from_utf8(&sums).map_err(|_| "SHA256SUMS is not text")?;
    let pairs = horizon_release_sign::parse_sums(text).map_err(|e| format!("{tag}: {e}"))?;
    pairs
        .into_iter()
        .find(|(n, _)| n == asset)
        .map(|(_, hex)| hex)
        .ok_or_else(|| format!("{asset} is not in {tag}'s checksums, so it was not installed."))
}

/// The SHA-256 of `asset` in the Nightly build published under `tag`, fetched from its
/// SHA256SUMS (signature ignored).
pub fn nightly_digest(
    tag: &str,
    asset: &str,
    _fetch: impl Fn(&str) -> Result<Option<Vec<u8>>, String>,
) -> Result<String, String> {
    let sums = _fetch(&asset_url(tag, "SHA256SUMS"))?
        .ok_or_else(|| format!("The Nightly build has no checksums, so it was not installed."))?;
    let text = std::str::from_utf8(&sums).map_err(|_| "SHA256SUMS is not text")?;
    let pairs =
        horizon_release_sign::parse_sums(text).map_err(|e| format!("The Nightly build: {e}"))?;
    pairs
        .into_iter()
        .find(|(n, _)| n == asset)
        .map(|(_, hex)| hex)
        .ok_or_else(|| {
            format!("{asset} is not in the Nightly build's checksums, so it was not installed.")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use horizon_release_sign::sha256_hex;

    #[test]
    fn releases_from_before_signing_install_only_with_their_pinned_files() {
        let tampered =
            |_: &str| -> Result<Option<Vec<u8>>, String> { Ok(Some(b"anything".to_vec())) };
        let pinned =
            horizon_release_sign::legacy_digest("v2026.10.12", "horizon-linux-x86_64.tar.gz")
                .expect("v2026.10.12 is from before signing");
        assert_eq!(
            digest("v2026.10.12", "horizon-linux-x86_64.tar.gz", tampered),
            Ok(pinned.to_owned())
        );
    }

    #[test]
    fn an_unsigned_release_installs_its_checksums() {
        let sums = horizon_release_sign::sums_file(&[(
            "horizon.zip".into(),
            sha256_hex(b"game"),
        )]);
        let fetch = |_: &str| -> Result<Option<Vec<u8>>, String> { Ok(Some(sums.clone().into_bytes())) };
        assert_eq!(
            digest("v2099.01.01", "horizon.zip", fetch),
            Ok(sha256_hex(b"game"))
        );
    }

    #[test]
    fn a_nightly_installs_its_checksums() {
        let sums = horizon_release_sign::sums_file(&[(
            "horizon.zip".into(),
            sha256_hex(b"nightly"),
        )]);
        let fetch = |_: &str| -> Result<Option<Vec<u8>>, String> { Ok(Some(sums.clone().into_bytes())) };
        assert_eq!(
            nightly_digest("nightly", "horizon.zip", fetch),
            Ok(sha256_hex(b"nightly"))
        );
    }
}
