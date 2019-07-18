use std::fs::File;
use std::io;

use ironsea_store::{Store, Load};

pub fn load<T: Load>(from: String) -> io::Result<T> {
    let file_in = File::open(from)?;

    T::load(file_in)
}

pub fn store<T: Store>(table: &mut T, to: String) -> io::Result<()> {
    let file_out = File::create(to)?;

    table.store(file_out)
}
