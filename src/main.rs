use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

mod demo_header;
use demo_header::DemoHeader;

use crate::demo_header::DEMO_HEADER_BYTES_LEN;

pub const HL2_MAX_OS_PATH: usize = 260;

enum DemoCmd {
    /// a startup message, process as fast as possible
    SignOn = 1,

    /// a normal network packet
    Packet = 2,

    /// sync client clock to demo tick
    SyncTick = 3,

    /// console command
    ConsoleCmd = 4,

    /// user input command
    UserCmd = 5,

    /// network data tables
    DataTables = 6,

    /// end of time
    Stop = 7,

    StringTables = 8,
    LastCmd, // = StringTables
}

mod dem {
    pub const SIGNON: u8 = 1;
    pub const PACKET: u8 = 2;
    pub const SYNCTICK: u8 = 3;
    pub const CONSOLECMD: u8 = 4;
    pub const USERCMD: u8 = 5;
    pub const DATATABLES: u8 = 6;
    pub const STOP: u8 = 7;
    pub const STRINGTABLES: u8 = 8;
    pub const LASTCMD: u8 = 8;
}

pub struct Frame {
    pub server: i32,
    pub client: i32,
    pub sub_packet_size: i32,
}

fn body(buf: &[u8]) {
    println!(
        "{:?}",
        buf.chunks(4)
            .collect::<Vec<_>>()
            .iter()
            .take(100)
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        buf.chunks_exact(4)
            .map(|x| { i32::from_ne_bytes([x[0], x[1], x[2], x[3]]) })
            .take(100)
            .collect::<Vec<_>>()
    );
    // let mut i = 0;
    // while i < buf.len() {
    //     let size = {
    //         let b = &buf[i..i + 4];
    //         i32::from_ne_bytes([b[0], b[1], b[2], b[3]])
    //     };
    //     println!("{}", size);
    //     i += size as usize;
    // }
}

#[derive(Debug, Parser)]
struct Cli {
    /// path to the demo file (.dem) to inspect
    dem: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.dem.is_file() {
        bail!(".dem file appears to be not a file?")
    }

    let buf = fs::read(cli.dem)?;

    let header = DemoHeader::read(&buf)?;
    println!("{:#?}", header);

    body(&buf[DEMO_HEADER_BYTES_LEN..]);

    Ok(())
}
