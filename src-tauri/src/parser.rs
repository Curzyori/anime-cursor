use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Seek, SeekFrom};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frame {
    pub data: Vec<u8>, // Raw PNG atau ICO bytes
    pub is_png: bool,  // True jika PNG, false jika ICO
    pub width: u32,
    pub height: u32,
    pub hotspot_x: u32, // hotspot from CUR directory entry (bytes 10-11)
    pub hotspot_y: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniCursor {
    pub frames: Vec<Frame>,
    pub frame_count: u32,
    pub rate: u32,
}

// ANI Header Structure (anih chunk - 36 bytes)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct AniHeader {
    size: u32,         // size of this structure (usually 36)
    frames: u32,       // number of unique frames
    steps: u32,        // number of animation steps
    width: u32,        // width (usually 0 or 32)
    height: u32,       // height (usually 0 or 32)
    bit_count: u32,    // color depth
    planes: u32,       // color planes
    display_rate: u32, // default display rate (jiffies - 1/60th of a second)
    flags: u32,        // flags (usually 1 for icon/cursor, 3 if contains sequence)
}

const MAX_FRAME_DATA: u64 = 10 * 1024 * 1024; // 10MB per frame — sanity cap
const MAX_FRAMES: usize = 1000;

pub fn parse_ani(data: &[u8]) -> Result<AniCursor, String> {
    let mut cursor = Cursor::new(data);

    // 1. Read RIFF header
    let mut riff_magic = [0u8; 4];
    cursor
        .read_exact(&mut riff_magic)
        .map_err(|e| e.to_string())?;
    if &riff_magic != b"RIFF" {
        return Err("Not a valid RIFF file".to_string());
    }

    // Skip file size
    cursor
        .seek(SeekFrom::Current(4))
        .map_err(|e| e.to_string())?;

    // Read ACON signature
    let mut acon_magic = [0u8; 4];
    cursor
        .read_exact(&mut acon_magic)
        .map_err(|e| e.to_string())?;
    if &acon_magic != b"ACON" {
        return Err("Not a valid ANI cursor file (missing ACON)".to_string());
    }

    let mut ani_header: Option<AniHeader> = None;
    let mut frames: Vec<Frame> = Vec::new();
    let mut default_rate: u32 = 10; // default rate (10 jiffies = 1/6th sec)

    // 2. Parse RIFF chunks
    let file_len = data.len() as u64;
    while cursor.position() < file_len {
        let mut chunk_id = [0u8; 4];
        if cursor.read_exact(&mut chunk_id).is_err() {
            break; // EOF
        }

        let mut chunk_size_bytes = [0u8; 4];
        if cursor.read_exact(&mut chunk_size_bytes).is_err() {
            break;
        }
        let chunk_size = u32::from_le_bytes(chunk_size_bytes) as u64;
        let start_pos = cursor.position();

        match &chunk_id {
            b"anih" => {
                if chunk_size >= 36 {
                    let mut buf = [0u8; 36];
                    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;

                    let header = AniHeader {
                        size: u32::from_le_bytes(buf[0..4].try_into().unwrap()),
                        frames: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
                        steps: u32::from_le_bytes(buf[8..12].try_into().unwrap()),
                        width: u32::from_le_bytes(buf[12..16].try_into().unwrap()),
                        height: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
                        bit_count: u32::from_le_bytes(buf[20..24].try_into().unwrap()),
                        planes: u32::from_le_bytes(buf[24..28].try_into().unwrap()),
                        display_rate: u32::from_le_bytes(buf[28..32].try_into().unwrap()),
                        flags: u32::from_le_bytes(buf[32..36].try_into().unwrap()),
                    };
                    ani_header = Some(header);
                    default_rate = header.display_rate;
                }
            }
            b"LIST" => {
                let mut list_type = [0u8; 4];
                cursor
                    .read_exact(&mut list_type)
                    .map_err(|e| e.to_string())?;

                if &list_type == b"fram" {
                    // Internal chunks of fram list
                    let list_end = start_pos
                        .checked_add(chunk_size)
                        .filter(|end| *end <= file_len)
                        .ok_or("Invalid fram list boundary")?;
                    while cursor.position() < list_end {
                        if list_end - cursor.position() < 8 {
                            return Err("Truncated fram subchunk header".to_string());
                        }
                        let mut sub_id = [0u8; 4];
                        if cursor.read_exact(&mut sub_id).is_err() {
                            break;
                        }
                        let mut sub_size_bytes = [0u8; 4];
                        if cursor.read_exact(&mut sub_size_bytes).is_err() {
                            break;
                        }
                        let sub_size = u32::from_le_bytes(sub_size_bytes) as u64;
                        let sub_start = cursor.position();
                        let sub_end = sub_start
                            .checked_add(sub_size)
                            .filter(|end| *end <= list_end)
                            .ok_or("Invalid fram subchunk boundary")?;

                        if &sub_id == b"icon" {
                            if sub_size > MAX_FRAME_DATA {
                                return Err(format!(
                                    "Frame data too large: {} bytes (max {})",
                                    sub_size, MAX_FRAME_DATA
                                ));
                            }
                            if frames.len() >= MAX_FRAMES {
                                return Err(format!(
                                    "Too many frames: {} (max {})",
                                    frames.len() + 1,
                                    MAX_FRAMES
                                ));
                            }
                            let mut icon_data = vec![0u8; sub_size as usize];
                            cursor
                                .read_exact(&mut icon_data)
                                .map_err(|e| e.to_string())?;

                            // Detect PNG magic vs Windows ICO/CUR header
                            let is_png =
                                icon_data.len() >= 8 && &icon_data[0..8] == b"\x89PNG\r\n\x1a\n";

                            // ICO/CUR format:
                            // Bytes 0-1: reserved (0)
                            // Bytes 2-3: type (1=ICO, 2=CUR)
                            // Bytes 4-5: image count
                            // Then for each image (16 bytes per entry):
                            //   Byte 0: width (0 = 256)
                            //   Byte 1: height (0 = 256)
                            //   Byte 2: colors
                            //   Byte 3: reserved
                            //   Bytes 4-5: hotspot_x (CUR only)
                            //   Bytes 6-7: hotspot_y (CUR only)
                            //   Bytes 8-11: size
                            //   Bytes 12-15: offset
                            // ICO/CUR format: directory entry at offset 6
                            let (hotspot_x, hotspot_y, fw, fh) = if !is_png && icon_data.len() >= 22
                            {
                                let dir_offset = 6;
                                let w = if icon_data[dir_offset] == 0 {
                                    256
                                } else {
                                    icon_data[dir_offset] as u32
                                };
                                let h = if icon_data[dir_offset + 1] == 0 {
                                    256
                                } else {
                                    icon_data[dir_offset + 1] as u32
                                };
                                let cursor_type = u16::from_le_bytes([icon_data[2], icon_data[3]]);
                                if cursor_type == 2 {
                                    let hx = u16::from_le_bytes([
                                        icon_data[dir_offset + 4],
                                        icon_data[dir_offset + 5],
                                    ]);
                                    let hy = u16::from_le_bytes([
                                        icon_data[dir_offset + 6],
                                        icon_data[dir_offset + 7],
                                    ]);
                                    (u32::from(hx), u32::from(hy), w, h)
                                } else {
                                    (0, 0, w, h)
                                }
                            } else {
                                (0, 0, 32, 32)
                            };

                            frames.push(Frame {
                                data: icon_data,
                                is_png,
                                width: fw,
                                height: fh,
                                hotspot_x,
                                hotspot_y,
                            });
                        } else {
                            cursor
                                .seek(SeekFrom::Start(sub_end))
                                .map_err(|e| e.to_string())?;
                        }

                        // Padding byte if sub_size is odd
                        if !sub_size.is_multiple_of(2) {
                            if cursor.position() >= list_end {
                                return Err("Missing fram subchunk padding".to_string());
                            }
                            cursor
                                .seek(SeekFrom::Current(1))
                                .map_err(|e| e.to_string())?;
                        }
                    }
                }
            }
            _ => {}
        }

        // Align position to next even address as per RIFF spec
        let padded_size = if !chunk_size.is_multiple_of(2) {
            chunk_size + 1
        } else {
            chunk_size
        };
        let chunk_end = start_pos
            .checked_add(padded_size)
            .filter(|end| *end <= file_len)
            .ok_or("Invalid RIFF chunk boundary")?;
        cursor
            .seek(SeekFrom::Start(chunk_end))
            .map_err(|e| e.to_string())?;
    }

    ani_header.ok_or_else(|| "Missing 'anih' chunk in ANI file".to_string())?;
    let frame_count = frames.len() as u32;

    Ok(AniCursor {
        frames,
        frame_count,
        rate: default_rate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reports_actual_frame_count_not_untrusted_header_count() {
        let mut data = b"RIFF".to_vec();
        data.extend_from_slice(&40u32.to_le_bytes());
        data.extend_from_slice(b"ACONanih");
        data.extend_from_slice(&36u32.to_le_bytes());
        data.extend_from_slice(&36u32.to_le_bytes());
        data.extend_from_slice(&999u32.to_le_bytes());
        data.extend_from_slice(&[0u8; 28]);

        let cursor = parse_ani(&data).expect("minimal ANI should parse");
        assert!(cursor.frames.is_empty());
        assert_eq!(cursor.frame_count, 0);
    }

    #[test]
    fn rejects_subchunk_that_exceeds_fram_list_boundary() {
        let mut data = b"RIFF".to_vec();
        data.extend_from_slice(&64u32.to_le_bytes());
        data.extend_from_slice(b"ACONanih");
        data.extend_from_slice(&36u32.to_le_bytes());
        data.extend_from_slice(&36u32.to_le_bytes());
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(&[0u8; 28]);
        data.extend_from_slice(b"LIST");
        data.extend_from_slice(&12u32.to_le_bytes());
        data.extend_from_slice(b"framicon");
        data.extend_from_slice(&1u32.to_le_bytes());
        data.push(0);
        data.push(0);

        assert!(parse_ani(&data).is_err());
    }

    #[test]
    fn test_parse_example_ani() {
        let path = "../.example/ani/madoka-magica-normal-9f4a71cf/Madoka Magica - Miki Sayaka - Puella Magi Madoka M_256.ani";
        let data = fs::read(path).expect("Failed to read test file");
        let cursor = parse_ani(&data).expect("Failed to parse ANI");
        assert!(!cursor.frames.is_empty());

        let magic = &cursor.frames[0].data[0..4];
        // Must be a valid ICO/CUR file header ([0, 0, 1/2, 0])
        assert!(magic == [0, 0, 1, 0] || magic == [0, 0, 2, 0]);

        // Hotspot should be read from CUR directory entry, not from count field
        let f = &cursor.frames[0];
        println!(
            "Frame 0: {}x{}, hotspot ({}, {})",
            f.width, f.height, f.hotspot_x, f.hotspot_y
        );
    }
}
