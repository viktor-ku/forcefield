use anyhow::{bail, Result};

use crate::HL2_MAX_OS_PATH;

pub const DEMO_PROTOCOL: i32 = 3;
pub const DEMO_HEADER_ID: &[u8] = b"HL2DEMO\0";

/// Total number of bytes to read from .dem
/// file to extract the header info.
/// 24 for all numeric fields + 1040 for 4
/// filesystem paths.
pub const DEMO_HEADER_BYTES_LEN: i32 = 24 + 1040;

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
    pub fn read(buf: &[u8]) -> Result<Self> {
        let mut cur = 0;

        let header = {
            let b = &buf[cur..(cur + 8)];
            cur += 8;
            b
        };

        if header != DEMO_HEADER_ID {
            bail!("expected hl2 demo");
        }

        let demo_protocol = {
            let b = &buf[cur..(cur + 4)];
            cur += 4;
            i32::from_le_bytes([b[0], b[1], b[2], b[3]])
        };

        let net_protocol = {
            let b = &buf[cur..(cur + 4)];
            cur += 4;
            i32::from_le_bytes([b[0], b[1], b[2], b[3]])
        };

        let server_name = {
            let b = &buf[cur..(cur + HL2_MAX_OS_PATH)];
            cur += HL2_MAX_OS_PATH;
            let v: Vec<u8> = b.into_iter().map(|x| *x).take_while(|x| *x != 0).collect();
            String::from_utf8(v)?
        };

        let client_name = {
            let b = &buf[cur..(cur + HL2_MAX_OS_PATH)];
            cur += HL2_MAX_OS_PATH;
            let v: Vec<u8> = b.into_iter().map(|x| *x).take_while(|x| *x != 0).collect();
            String::from_utf8(v)?
        };

        let map_name = {
            let b = &buf[cur..(cur + HL2_MAX_OS_PATH)];
            cur += HL2_MAX_OS_PATH;
            let v: Vec<u8> = b.into_iter().map(|x| *x).take_while(|x| *x != 0).collect();
            String::from_utf8(v)?
        };

        let game_dir = {
            let b = &buf[cur..(cur + HL2_MAX_OS_PATH)];
            cur += HL2_MAX_OS_PATH;
            let v: Vec<u8> = b.into_iter().map(|x| *x).take_while(|x| *x != 0).collect();
            String::from_utf8(v)?
        };

        let time = {
            let b = &buf[cur..(cur + 4)];
            cur += 4;
            f32::from_ne_bytes([b[0], b[1], b[2], b[3]])
        };

        let ticks = {
            let b = &buf[cur..(cur + 4)];
            cur += 4;
            i32::from_ne_bytes([b[0], b[1], b[2], b[3]])
        };

        let frames = {
            let b = &buf[cur..(cur + 4)];
            cur += 4;
            i32::from_ne_bytes([b[0], b[1], b[2], b[3]])
        };

        let sign_on_length = {
            let b = &buf[cur..(cur + 4)];
            i32::from_ne_bytes([b[0], b[1], b[2], b[3]])
        };

        Ok(DemoHeader {
            demo_protocol,
            net_protocol,
            server_name,
            client_name,
            map_name,
            game_dir,
            time,
            ticks,
            frames,
            sign_on_length,
        })
    }
}
