//! Browser assets are included in the crate, so consumers can serve them locally.

use std::io;
use std::path::Path;

macro_rules! asset {
    ($name:literal) => {
        (
            $name,
            include_bytes!(concat!("../assets/", $name)).as_slice(),
        )
    };
}

include!("bundled_assets.rs");

/// Export the complete, version-pinned Worker bundle from a native build script.
/// Serve this directory at [`crate::SyntaxConfig::assets_url`]. Existing bundle
/// files are overwritten; unrelated files are left alone.
pub fn write_syntax_assets(directory: impl AsRef<Path>) -> io::Result<()> {
    for (name, contents) in ASSETS {
        let path = directory.as_ref().join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, contents)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::BUNDLED_SYNTAX_LANGUAGES;
    use crate::CodeLanguage;

    #[test]
    fn bundles_every_editor_language_except_plain_text() {
        for language in CodeLanguage::ALL {
            assert_eq!(
                BUNDLED_SYNTAX_LANGUAGES.contains(&language.as_str()),
                language != CodeLanguage::PlainText,
                "{language}"
            );
        }
    }
}
