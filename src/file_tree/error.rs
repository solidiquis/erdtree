use std::io;

pub fn not_dir_err() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Argument 'root_location' must be a path to a directory.")
}
