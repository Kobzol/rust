use std::fs;
use std::path::Path;

/// A safe wrapper around `std::fs::remove_file` that prints the file path if an operation fails.
pub fn remove_file<P: AsRef<Path>>(path: P) {
    fs::remove_file(path.as_ref()).expect(&format!(
        "the file in path \"{:?}\" could not be removed.",
        path.as_ref().display()
    ));
}

/// A safe wrapper around `std::fs::copy` that prints the file paths if an operation fails.
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    fs::copy(from.as_ref(), to.as_ref()).expect(&format!(
        "the file \"{:?}\" could not be copied over to \"{:?}\".",
        from.as_ref().display(),
        to.as_ref().display(),
    ));
}

/// A safe wrapper around `std::fs::File::create` that prints the file path if an operation fails.
pub fn create_file<P: AsRef<Path>>(path: P) {
    fs::File::create(path.as_ref()).expect(&format!(
        "the file in path \"{:?}\" could not be created.",
        path.as_ref().display()
    ));
}

/// A safe wrapper around `std::fs::read` that prints the file path if an operation fails.
pub fn read<P: AsRef<Path>>(path: P) -> Vec<u8> {
    fs::read(path.as_ref())
        .expect(&format!("the file in path \"{:?}\" could not be read.", path.as_ref().display()))
}

/// A safe wrapper around `std::fs::read_to_string` that prints the file path if an operation fails.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path.as_ref()).expect(&format!(
        "the file in path \"{:?}\" could not be read into a String.",
        path.as_ref().display()
    ))
}

/// A safe wrapper around `std::fs::read_dir` that prints the file path if an operation fails.
pub fn read_dir<P: AsRef<Path>>(path: P) -> fs::ReadDir {
    fs::read_dir(path.as_ref()).expect(&format!(
        "the directory in path \"{:?}\" could not be read.",
        path.as_ref().display()
    ))
}

/// A safe wrapper around `std::fs::write` that prints the file path if an operation fails.
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) {
    fs::write(path.as_ref(), contents.as_ref()).expect(&format!(
        "the file in path \"{:?}\" could not be written to.",
        path.as_ref().display()
    ));
}

/// A safe wrapper around `std::fs::remove_dir_all` that prints the file path if an operation fails.
pub fn remove_dir_all<P: AsRef<Path>>(path: P) {
    fs::remove_dir_all(path.as_ref()).expect(&format!(
        "the directory in path \"{:?}\" could not be removed alongside all its contents.",
        path.as_ref().display(),
    ));
}

/// A safe wrapper around `std::fs::create_dir` that prints the file path if an operation fails.
pub fn create_dir<P: AsRef<Path>>(path: P) {
    fs::create_dir(path.as_ref()).expect(&format!(
        "the directory in path \"{:?}\" could not be created.",
        path.as_ref().display()
    ));
}

/// A safe wrapper around `std::fs::create_dir_all` that prints the file path if an operation fails.
pub fn create_dir_all<P: AsRef<Path>>(path: P) {
    fs::create_dir_all(path.as_ref()).expect(&format!(
        "the directory (and all its parents) in path \"{:?}\" could not be created.",
        path.as_ref().display()
    ));
}

/// A safe wrapper around `std::fs::metadata` that prints the file path if an operation fails.
pub fn metadata<P: AsRef<Path>>(path: P) -> fs::Metadata {
    fs::metadata(path.as_ref()).expect(&format!(
        "the file's metadata in path \"{:?}\" could not be read.",
        path.as_ref().display()
    ))
}

/// A safe wrapper around `std::fs::rename` that prints the file paths if an operation fails.
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    fs::rename(from.as_ref(), to.as_ref()).expect(&format!(
        "the file \"{:?}\" could not be moved over to \"{:?}\".",
        from.as_ref().display(),
        to.as_ref().display(),
    ));
}
