#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_fn)]

// #define VCOS_ALIGN_DOWN(p,n) (((ptrdiff_t)(p)) & ~((n)-1))
// #define VCOS_ALIGN_UP(p,n) VCOS_ALIGN_DOWN((ptrdiff_t)(p)+(n)-1,(n))

pub fn vcos_align_down(p: u32, n: u8) -> u32 {
    p & !((n - 1) as u32)
}

#[test]
fn test_vcos_align_down() {
    let mut result;

    result = vcos_align_down(1, 32);
    assert_eq!(result, 0, concat!("(1, 32): ", stringify!(result)));
    result = vcos_align_down(100, 16);
    assert_eq!(result, 96, concat!("(100, 16): ", stringify!(result)));
    result = vcos_align_down(10000, 32);
    assert_eq!(result, 9984, concat!("(10000, 32): ", stringify!(result)));
}

pub fn vcos_align_up(p: u32, n: u8) -> u32 {
    vcos_align_down(p + (n as u32) - 1, n)
}

#[test]
fn test_vcos_align_up() {
    let mut result;

    result = vcos_align_up(1, 32);
    assert_eq!(result, 32, concat!("(1, 32): ", stringify!(result)));
    result = vcos_align_up(100, 16);
    assert_eq!(result, 112, concat!("(100, 16): ", stringify!(result)));
    result = vcos_align_up(10000, 32);
    assert_eq!(result, 10016, concat!("(10000, 32): ", stringify!(result)));
}


// #[macro_export]
// macro_rules! mmal_fourcc {
//     ($s:expr) => {
//         {
//             let s = ::std::vec::Vec::from("abcd");
//             ((s[0]) | (s[1] << 8) | (s[2] << 16) | (s[3] << 24)) as u32
//         }
//     }
// }

#[macro_export]
macro_rules! mmal_fourcc {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            (($a as u32) | (($b as u32) << 8) | (($c as u32) << 16) | (($d as u32) << 24)) as u32
        }
    }
}

pub const MMAL_ENCODING_JPEG: u32 = mmal_fourcc!('J','P','E','G');
pub const MMAL_ENCODING_GIF: u32 = mmal_fourcc!('G','I','F',' ');
pub const MMAL_ENCODING_PNG: u32 = mmal_fourcc!('P','N','G',' ');
pub const MMAL_ENCODING_PPM: u32 = mmal_fourcc!('P','P','M',' ');
pub const MMAL_ENCODING_TGA: u32 = mmal_fourcc!('T','G','A',' ');
pub const MMAL_ENCODING_BMP: u32 = mmal_fourcc!('B','M','P',' ');

pub const MMAL_ENCODING_I420: u32 = mmal_fourcc!('I','4','2','0');
pub const MMAL_ENCODING_RGB24: u32 = mmal_fourcc!('R', 'G', 'B', '3');
pub const MMAL_ENCODING_BGR24: u32 = mmal_fourcc!('B', 'G', 'R', '3');

/**
 * VideoCore opaque image format, image handles are returned to
 * the host but not the actual image data.
 */
pub const MMAL_ENCODING_OPAQUE: u32 = mmal_fourcc!('O','P','Q','V');


// mmal_events.h
/** Error event. Data contains a \ref MMAL_STATUS_T */
pub const MMAL_EVENT_ERROR: u32 = mmal_fourcc!('E','R','R','O');

/** End-of-stream event. Data contains a \ref MMAL_EVENT_END_OF_STREAM_T */
pub const MMAL_EVENT_EOS: u32 = mmal_fourcc!('E','E','O','S');

/** Format changed event. Data contains a \ref MMAL_EVENT_FORMAT_CHANGED_T */
pub const MMAL_EVENT_FORMAT_CHANGED: u32 = mmal_fourcc!('E','F','C','H');

/** Parameter changed event. Data contains the new parameter value, see
 * \ref MMAL_EVENT_PARAMETER_CHANGED_T
 */
pub const MMAL_EVENT_PARAMETER_CHANGED: u32 = mmal_fourcc!('E','P','C','H');


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
