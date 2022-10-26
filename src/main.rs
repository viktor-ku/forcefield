use anyhow::{bail, Result};
use std::fs;

use crate::demo_header::DemoHeader;

mod demo_header;

pub const HL2_MAX_OS_PATH: usize = 260;

fn main() -> Result<()> {
    let buf = fs::read("./stuff.dem")?;

    let header = DemoHeader::read(&buf)?;
    println!("{:#?}", header);

    Ok(())
}
