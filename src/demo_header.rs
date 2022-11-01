use anyhow::{bail, Result};
use std::mem;

use crate::HL2_MAX_OS_PATH;

pub const DEMO_PROTOCOL: i32 = 3;
pub const DEMO_HEADER_ID: &[u8] = b"HL2DEMO\0";

#[repr(C)]
struct DemoHeaderRaw {
    demofilestamp: [u8; 8],
    demoprotocol: i32,
    networkprotocol: i32,
    servername: [u8; HL2_MAX_OS_PATH],
    clientname: [u8; HL2_MAX_OS_PATH],
    mapname: [u8; HL2_MAX_OS_PATH],
    gamedirectory: [u8; HL2_MAX_OS_PATH],
    playback_time: f32,
    playback_ticks: i32,
    playback_frames: i32,
    signonlength: i32,
}

#[derive(Debug)]
pub struct DemoHeader {
    pub demo_protocol: i32,
    pub net_protocol: i32,
    pub server_name: String,
    pub client_name: String,
    pub map_name: String,
    pub game_dir: String,

    /// The length of the demo, in seconds
    pub time: f32,

    /// The number of ticks in the demo
    pub ticks: i32,

    /// The number of frames in the demo
    pub frames: i32,

    /// Sign on length.
    /// Length of the signon data (Init for first frame)
    pub sign_on_length: i32,
}

impl DemoHeader {
    fn hl2_string(buf: [u8; HL2_MAX_OS_PATH]) -> Result<String> {
        let buf: Vec<u8> = buf
            .iter()
            .take_while(|val| **val != 0)
            .map(|x| *x)
            .collect();
        Ok(String::from_utf8(buf)?)
    }

    pub fn read(buf: &[u8]) -> Result<Self> {
        let dh_id = &buf[..DEMO_HEADER_ID.len()];

        if DEMO_HEADER_ID != dh_id {
            bail!("expected hl2 demo file")
        }

        let buf = &buf[..mem::size_of::<DemoHeaderRaw>()];

        // SAFETY:
        // Before reading the entire header, we checked first 8 bytes,
        // which turned out to be the expected header id (b"HL2DEMO\0").
        // It's reasonable to assume that the file in question is indeed
        // an hl2 demo file, thus reading the entire header with a pre
        // known sturcture should be fine.
        let dh: Option<&DemoHeaderRaw> = unsafe {
            let (_, chunks, _) = buf.align_to::<DemoHeaderRaw>();
            chunks.first()
        };

        let dh = match dh {
            Some(dh) => dh,
            None => bail!("could not read demo header"),
        };

        if dh.demoprotocol != DEMO_PROTOCOL {
            bail!("only known demo protocol at this time is 3")
        }

        Ok(DemoHeader {
            demo_protocol: dh.demoprotocol,
            net_protocol: dh.networkprotocol,
            server_name: DemoHeader::hl2_string(dh.servername)?,
            client_name: DemoHeader::hl2_string(dh.clientname)?,
            map_name: DemoHeader::hl2_string(dh.mapname)?,
            game_dir: DemoHeader::hl2_string(dh.gamedirectory)?,
            time: dh.playback_time,
            ticks: dh.playback_ticks,
            frames: dh.playback_frames,
            sign_on_length: dh.signonlength,
        })
    }
}
