//! Generation of the `PackageInfo` XML metadata file for a flat component
//! package.

use super::metadata::PkgMetadata;

/// Render the `PackageInfo` XML for a single-component package.
///
/// `num_files` is the number of payload entries (files + directories) and
/// `install_kb` the installed size in kilobytes; both are surfaced to the
/// installer via the `<payload>` element.
pub fn render(metadata: &PkgMetadata, num_files: u32, install_kb: u64) -> String {
    format!(
        concat!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n",
            "<pkg-info overwrite-permissions=\"true\" relocatable=\"false\" ",
            "identifier=\"{identifier}\" postinstall-action=\"none\" ",
            "version=\"{version}\" format-version=\"2\" install-location=\"{install_location}\" ",
            "auth=\"root\">\n",
            "    <payload numberOfFiles=\"{num_files}\" installKBytes=\"{install_kb}\"/>\n",
            "</pkg-info>\n"
        ),
        identifier = xml_escape(&metadata.identifier),
        version = xml_escape(&metadata.version),
        install_location = xml_escape(&metadata.install_location),
        num_files = num_files,
        install_kb = install_kb,
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_identifier_and_payload() {
        let md = PkgMetadata {
            identifier: "com.example.tool".to_string(),
            version: "2.1.0".to_string(),
            ..Default::default()
        };
        let xml = render(&md, 9, 2);
        assert!(xml.contains("identifier=\"com.example.tool\""));
        assert!(xml.contains("version=\"2.1.0\""));
        assert!(xml.contains("numberOfFiles=\"9\""));
        assert!(xml.contains("installKBytes=\"2\""));
    }
}
