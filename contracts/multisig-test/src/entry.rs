use ckb_std::{
    ckb_constants::Source,
    high_level::load_cell_capacity,
};
use core::result::Result;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let capacity = load_cell_capacity(0, Source::Output)?;
    if capacity < 150_0000_0000 {
        return Err(Error::CapacityNotEnough);
    }
    Ok(())
}
