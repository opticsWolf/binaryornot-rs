/// Known binary file extensions, loaded from binary_extensions.csv at compile time.
///
/// A file with one of these extensions is classified as binary without reading its contents.

use std::collections::HashSet;

/// Return a `HashSet` of all known binary file extensions (lowercase, without the dot).
pub fn binary_extensions() -> &'static HashSet<&'static str> {
    use std::sync::OnceLock;
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut set = HashSet::new();
        for &ext in EXTENSIONS {
            set.insert(ext);
        }
        set
    })
}

/// All known binary extensions (lowercase, no dot prefix).
const EXTENSIONS: &[&str] = &[
    // 3D models
    "3ds", "blend", "dae", "fbx", "glb", "stl", "usdz",
    // Archives
    "7z", "bz2", "cab", "gz", "jar", "lz", "lz4", "lzma",
    "rar", "tar", "xz", "z", "zip", "zst",
    // Audio
    "aac", "aif", "aiff", "alac", "ape", "flac", "m4a",
    "mid", "midi", "mp3", "ogg", "opus", "wav", "wma",
    // CAD
    "dwg", "dwf", "dxb", "step", "stp",
    // Compiled
    "class", "dll", "o", "pyc", "pyo", "so", "wasm",
    // Databases
    "accdb", "db", "dbf", "mdb", "sqlite", "sqlite3",
    // Disk images
    "dmg", "img", "iso", "qcow2", "vdi", "vhd", "vmdk",
    // Documents
    "doc", "docx", "epub", "mobi", "odt", "pages", "pdf",
    "ppt", "pptx", "xls", "xlsx",
    // Executables
    "com", "exe", "msi",
    // Firmware
    "bin", "fw", "rom",
    // Fonts
    "eot", "otf", "ttc", "ttf", "woff", "woff2",
    // Game files
    "gba", "n64", "nds", "nes", "pak", "sav", "sfc", "smc",
    // GIS
    "shp", "shx",
    // Images
    "avif", "bmp", "cr2", "cr3", "dng", "gif", "heic", "heif",
    "ico", "jfif", "jp2", "jpeg", "jpg", "jxl", "nef", "orf",
    "png", "psd", "raw", "tif", "tiff", "webp", "xcf",
    // Scientific data
    "arrow", "avro", "fit", "fits", "hdf5", "mat", "npy",
    "npz", "parquet", "pcap", "pcapng",
    // Video
    "avi", "flv", "m4v", "mkv", "mov", "mp4", "mpeg", "mpg",
    "ogv", "webm", "wmv",
];

/// Check if a file extension is a known binary extension.
pub fn has_binary_extension(ext: &str) -> bool {
    let ext = ext.trim_start_matches('.').to_lowercase();
    binary_extensions().contains(ext.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_binary_extensions() {
        assert!(has_binary_extension("png"));
        assert!(has_binary_extension("jpg"));
        assert!(has_binary_extension("exe"));
        assert!(has_binary_extension("zip"));
        assert!(has_binary_extension("pyc"));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(has_binary_extension("PNG"));
        assert!(has_binary_extension("Png"));
        assert!(has_binary_extension(".JPG"));
    }

    #[test]
    fn test_text_extensions_not_binary() {
        assert!(!has_binary_extension("txt"));
        assert!(!has_binary_extension("py"));
        assert!(!has_binary_extension("js"));
        assert!(!has_binary_extension("md"));
    }
}
