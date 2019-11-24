#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::fmt;
use std::os::raw::c_uint;

// pub const VCOS_ALIGN_DOWN: c_uint = mmal_fourcc!p,n) (((ptrdiff_t)(p)) & ~((n)-1));
// pub const VCOS_ALIGN_UP: c_uint = mmal_fourcc!p,n) VCOS_ALIGN_DOWN((ptrdiff_t)(p)+(n)-1,(n));

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
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        (($a as c_uint) | (($b as c_uint) << 8) | (($c as c_uint) << 16) | (($d as c_uint) << 24))
            as c_uint
    }};
}

// mmal_encodings.h

/** \defgroup MmalEncodings List of pre-defined encodings
 * This defines a list of common encodings. This list isn't exhaustive and is only
 * provided as a convenience to avoid clients having to use FourCC codes directly.
 * However components are allowed to define and use their own FourCC codes. */
/* @{ */

/** \name Pre-defined video encodings */
/* @{ */
pub const MMAL_ENCODING_H264: c_uint = mmal_fourcc!('H', '2', '6', '4');
pub const MMAL_ENCODING_MVC: c_uint = mmal_fourcc!('M', 'V', 'C', ' ');
pub const MMAL_ENCODING_H263: c_uint = mmal_fourcc!('H', '2', '6', '3');
pub const MMAL_ENCODING_MP4V: c_uint = mmal_fourcc!('M', 'P', '4', 'V');
pub const MMAL_ENCODING_MP2V: c_uint = mmal_fourcc!('M', 'P', '2', 'V');
pub const MMAL_ENCODING_MP1V: c_uint = mmal_fourcc!('M', 'P', '1', 'V');
pub const MMAL_ENCODING_WMV3: c_uint = mmal_fourcc!('W', 'M', 'V', '3');
pub const MMAL_ENCODING_WMV2: c_uint = mmal_fourcc!('W', 'M', 'V', '2');
pub const MMAL_ENCODING_WMV1: c_uint = mmal_fourcc!('W', 'M', 'V', '1');
pub const MMAL_ENCODING_WVC1: c_uint = mmal_fourcc!('W', 'V', 'C', '1');
pub const MMAL_ENCODING_VP8: c_uint = mmal_fourcc!('V', 'P', '8', ' ');
pub const MMAL_ENCODING_VP7: c_uint = mmal_fourcc!('V', 'P', '7', ' ');
pub const MMAL_ENCODING_VP6: c_uint = mmal_fourcc!('V', 'P', '6', ' ');
pub const MMAL_ENCODING_THEORA: c_uint = mmal_fourcc!('T', 'H', 'E', 'O');
pub const MMAL_ENCODING_SPARK: c_uint = mmal_fourcc!('S', 'P', 'R', 'K');
pub const MMAL_ENCODING_MJPEG: c_uint = mmal_fourcc!('M', 'J', 'P', 'G');

pub const MMAL_ENCODING_JPEG: c_uint = mmal_fourcc!('J', 'P', 'E', 'G');
pub const MMAL_ENCODING_GIF: c_uint = mmal_fourcc!('G', 'I', 'F', ' ');
pub const MMAL_ENCODING_PNG: c_uint = mmal_fourcc!('P', 'N', 'G', ' ');
pub const MMAL_ENCODING_PPM: c_uint = mmal_fourcc!('P', 'P', 'M', ' ');
pub const MMAL_ENCODING_TGA: c_uint = mmal_fourcc!('T', 'G', 'A', ' ');
pub const MMAL_ENCODING_BMP: c_uint = mmal_fourcc!('B', 'M', 'P', ' ');

pub const MMAL_ENCODING_I420: c_uint = mmal_fourcc!('I', '4', '2', '0');
pub const MMAL_ENCODING_I420_SLICE: c_uint = mmal_fourcc!('S', '4', '2', '0');
pub const MMAL_ENCODING_YV12: c_uint = mmal_fourcc!('Y', 'V', '1', '2');
pub const MMAL_ENCODING_I422: c_uint = mmal_fourcc!('I', '4', '2', '2');
pub const MMAL_ENCODING_I422_SLICE: c_uint = mmal_fourcc!('S', '4', '2', '2');
pub const MMAL_ENCODING_YUYV: c_uint = mmal_fourcc!('Y', 'U', 'Y', 'V');
pub const MMAL_ENCODING_YVYU: c_uint = mmal_fourcc!('Y', 'V', 'Y', 'U');
pub const MMAL_ENCODING_UYVY: c_uint = mmal_fourcc!('U', 'Y', 'V', 'Y');
pub const MMAL_ENCODING_VYUY: c_uint = mmal_fourcc!('V', 'Y', 'U', 'Y');
pub const MMAL_ENCODING_NV12: c_uint = mmal_fourcc!('N', 'V', '1', '2');
pub const MMAL_ENCODING_NV21: c_uint = mmal_fourcc!('N', 'V', '2', '1');
pub const MMAL_ENCODING_ARGB: c_uint = mmal_fourcc!('A', 'R', 'G', 'B');
pub const MMAL_ENCODING_ARGB_SLICE: c_uint = mmal_fourcc!('a', 'r', 'g', 'b');
pub const MMAL_ENCODING_RGBA: c_uint = mmal_fourcc!('R', 'G', 'B', 'A');
pub const MMAL_ENCODING_RGBA_SLICE: c_uint = mmal_fourcc!('r', 'g', 'b', 'a');
pub const MMAL_ENCODING_ABGR: c_uint = mmal_fourcc!('A', 'B', 'G', 'R');
pub const MMAL_ENCODING_ABGR_SLICE: c_uint = mmal_fourcc!('a', 'b', 'g', 'r');
pub const MMAL_ENCODING_BGRA: c_uint = mmal_fourcc!('B', 'G', 'R', 'A');
pub const MMAL_ENCODING_BGRA_SLICE: c_uint = mmal_fourcc!('b', 'g', 'r', 'a');
pub const MMAL_ENCODING_RGB16: c_uint = mmal_fourcc!('R', 'G', 'B', '2');
pub const MMAL_ENCODING_RGB16_SLICE: c_uint = mmal_fourcc!('r', 'g', 'b', '2');
pub const MMAL_ENCODING_RGB24: c_uint = mmal_fourcc!('R', 'G', 'B', '3');
pub const MMAL_ENCODING_RGB24_SLICE: c_uint = mmal_fourcc!('r', 'g', 'b', '3');
pub const MMAL_ENCODING_RGB32: c_uint = mmal_fourcc!('R', 'G', 'B', '4');
pub const MMAL_ENCODING_RGB32_SLICE: c_uint = mmal_fourcc!('r', 'g', 'b', '4');
pub const MMAL_ENCODING_BGR16: c_uint = mmal_fourcc!('B', 'G', 'R', '2');
pub const MMAL_ENCODING_BGR16_SLICE: c_uint = mmal_fourcc!('b', 'g', 'r', '2');
pub const MMAL_ENCODING_BGR24: c_uint = mmal_fourcc!('B', 'G', 'R', '3');
pub const MMAL_ENCODING_BGR24_SLICE: c_uint = mmal_fourcc!('b', 'g', 'r', '3');
pub const MMAL_ENCODING_BGR32: c_uint = mmal_fourcc!('B', 'G', 'R', '4');
pub const MMAL_ENCODING_BGR32_SLICE: c_uint = mmal_fourcc!('b', 'g', 'r', '4');

/// YUV 4:2:0 planar, 16bit/component.
pub const MMAL_ENCODING_I420_16: c_uint = mmal_fourcc!('i', '4', '2', '0');
/// YUV 4:2:0 planar, 10bit/component as least sig 10bits of 16 bit words.
pub const MMAL_ENCODING_I420_10: c_uint = mmal_fourcc!('i', '4', '1', '0');

/// Bayer formats
/// FourCC values copied from V4L2 where defined.
/// 10 bit per pixel packed Bayer formats.
pub const MMAL_ENCODING_BAYER_SBGGR10P: c_uint = mmal_fourcc!('p', 'B', 'A', 'A'); //BGGR
pub const MMAL_ENCODING_BAYER_SGRBG10P: c_uint = mmal_fourcc!('p', 'g', 'A', 'A'); //GRBG
pub const MMAL_ENCODING_BAYER_SGBRG10P: c_uint = mmal_fourcc!('p', 'G', 'A', 'A'); //GBRG
pub const MMAL_ENCODING_BAYER_SRGGB10P: c_uint = mmal_fourcc!('p', 'R', 'A', 'A'); //RGGB

/// 8 bit per pixel Bayer formats.
pub const MMAL_ENCODING_BAYER_SBGGR8: c_uint = mmal_fourcc!('B', 'A', '8', '1'); //BGGR
pub const MMAL_ENCODING_BAYER_SGBRG8: c_uint = mmal_fourcc!('G', 'B', 'R', 'G'); //GBRG
pub const MMAL_ENCODING_BAYER_SGRBG8: c_uint = mmal_fourcc!('G', 'R', 'B', 'G'); //GRBG
pub const MMAL_ENCODING_BAYER_SRGGB8: c_uint = mmal_fourcc!('R', 'G', 'G', 'B'); //RGGB

/// 12 bit per pixel Bayer formats - not defined in V4L2, only 12bit expanded to 16.
/// Copy 10bpp packed 4CC pattern
pub const MMAL_ENCODING_BAYER_SBGGR12P: c_uint = mmal_fourcc!('p', 'B', '1', '2'); //BGGR
pub const MMAL_ENCODING_BAYER_SGRBG12P: c_uint = mmal_fourcc!('p', 'g', '1', '2'); //GRBG
pub const MMAL_ENCODING_BAYER_SGBRG12P: c_uint = mmal_fourcc!('p', 'G', '1', '2'); //GBRG
pub const MMAL_ENCODING_BAYER_SRGGB12P: c_uint = mmal_fourcc!('p', 'R', '1', '2'); //RGGB

/// 16 bit per pixel Bayer formats.
pub const MMAL_ENCODING_BAYER_SBGGR16: c_uint = mmal_fourcc!('R', 'G', '1', '6'); //BGGR
pub const MMAL_ENCODING_BAYER_SGBRG16: c_uint = mmal_fourcc!('G', 'B', '1', '6'); //GBRG
pub const MMAL_ENCODING_BAYER_SGRBG16: c_uint = mmal_fourcc!('G', 'R', '1', '6'); //GRBG
pub const MMAL_ENCODING_BAYER_SRGGB16: c_uint = mmal_fourcc!('R', 'G', '1', '6'); //RGGB

/// 10 bit per pixel DPCM compressed to 8bits Bayer formats.
pub const MMAL_ENCODING_BAYER_SBGGR10DPCM8: c_uint = mmal_fourcc!('b', 'B', 'A', '8'); //BGGR
pub const MMAL_ENCODING_BAYER_SGBRG10DPCM8: c_uint = mmal_fourcc!('b', 'G', 'A', '8'); //GBRG
pub const MMAL_ENCODING_BAYER_SGRBG10DPCM8: c_uint = mmal_fourcc!('B', 'D', '1', '0'); //GRBG
pub const MMAL_ENCODING_BAYER_SRGGB10DPCM8: c_uint = mmal_fourcc!('b', 'R', 'A', '8'); //RGGB

/// SAND Video (YUVUV128) format, native format understood by VideoCore.
/// This format is *not* opaque - if requested you will receive full frames
/// of YUV_UV video.
pub const MMAL_ENCODING_YUVUV128: c_uint = mmal_fourcc!('S', 'A', 'N', 'D');
/// 16 bit SAND Video (YUVUV64_16) format.
/// This format is *not* opaque - if requested you will receive full frames
/// of YUV_UV_16 video.
pub const MMAL_ENCODING_YUVUV64_16: c_uint = mmal_fourcc!('S', 'A', '1', '6');
/// 10 bit SAND Video format, packed as least sig 10 bits of 16 bit words.
pub const MMAL_ENCODING_YUVUV64_10: c_uint = mmal_fourcc!('S', 'A', '1', '0');

/// VideoCore opaque image format, image handles are returned to
/// the host but not the actual image data.
pub const MMAL_ENCODING_OPAQUE: c_uint = mmal_fourcc!('O', 'P', 'Q', 'V');

/// An EGL image handle
pub const MMAL_ENCODING_EGL_IMAGE: c_uint = mmal_fourcc!('E', 'G', 'L', 'I');

pub const MMAL_ENCODING_MP4A: c_uint = mmal_fourcc!('M', 'P', '4', 'A');
pub const MMAL_ENCODING_MPGA: c_uint = mmal_fourcc!('M', 'P', 'G', 'A');
pub const MMAL_ENCODING_ALAW: c_uint = mmal_fourcc!('A', 'L', 'A', 'W');
pub const MMAL_ENCODING_MULAW: c_uint = mmal_fourcc!('U', 'L', 'A', 'W');
pub const MMAL_ENCODING_ADPCM_MS: c_uint = mmal_fourcc!('M', 'S', 0x0, 0x2);
pub const MMAL_ENCODING_ADPCM_IMA_MS: c_uint = mmal_fourcc!('M', 'S', 0x0, 0x1);
pub const MMAL_ENCODING_ADPCM_SWF: c_uint = mmal_fourcc!('A', 'S', 'W', 'F');
pub const MMAL_ENCODING_WMA1: c_uint = mmal_fourcc!('W', 'M', 'A', '1');
pub const MMAL_ENCODING_WMA2: c_uint = mmal_fourcc!('W', 'M', 'A', '2');
pub const MMAL_ENCODING_WMAP: c_uint = mmal_fourcc!('W', 'M', 'A', 'P');
pub const MMAL_ENCODING_WMAL: c_uint = mmal_fourcc!('W', 'M', 'A', 'L');
pub const MMAL_ENCODING_WMAV: c_uint = mmal_fourcc!('W', 'M', 'A', 'V');
pub const MMAL_ENCODING_AMRNB: c_uint = mmal_fourcc!('A', 'M', 'R', 'N');
pub const MMAL_ENCODING_AMRWB: c_uint = mmal_fourcc!('A', 'M', 'R', 'W');
pub const MMAL_ENCODING_AMRWBP: c_uint = mmal_fourcc!('A', 'M', 'R', 'P');
pub const MMAL_ENCODING_AC3: c_uint = mmal_fourcc!('A', 'C', '3', ' ');
pub const MMAL_ENCODING_EAC3: c_uint = mmal_fourcc!('E', 'A', 'C', '3');
pub const MMAL_ENCODING_DTS: c_uint = mmal_fourcc!('D', 'T', 'S', ' ');
pub const MMAL_ENCODING_MLP: c_uint = mmal_fourcc!('M', 'L', 'P', ' ');
pub const MMAL_ENCODING_FLAC: c_uint = mmal_fourcc!('F', 'L', 'A', 'C');
pub const MMAL_ENCODING_VORBIS: c_uint = mmal_fourcc!('V', 'O', 'R', 'B');
pub const MMAL_ENCODING_SPEEX: c_uint = mmal_fourcc!('S', 'P', 'X', ' ');
pub const MMAL_ENCODING_ATRAC3: c_uint = mmal_fourcc!('A', 'T', 'R', '3');
pub const MMAL_ENCODING_ATRACX: c_uint = mmal_fourcc!('A', 'T', 'R', 'X');
pub const MMAL_ENCODING_ATRACL: c_uint = mmal_fourcc!('A', 'T', 'R', 'L');
pub const MMAL_ENCODING_MIDI: c_uint = mmal_fourcc!('M', 'I', 'D', 'I');
pub const MMAL_ENCODING_EVRC: c_uint = mmal_fourcc!('E', 'V', 'R', 'C');
pub const MMAL_ENCODING_NELLYMOSER: c_uint = mmal_fourcc!('N', 'E', 'L', 'Y');
pub const MMAL_ENCODING_QCELP: c_uint = mmal_fourcc!('Q', 'C', 'E', 'L');
pub const MMAL_ENCODING_MP4V_DIVX_DRM: c_uint = mmal_fourcc!('M', '4', 'V', 'D');

/** \defgroup MmalEncodingVariants List of pre-defined encoding variants
 * This defines a list of common encoding variants. This list isn't exhaustive and is only
 * provided as a convenience to avoid clients having to use FourCC codes directly.
 * However components are allowed to define and use their own FourCC codes. */
/* @{ */

/** \name Pre-defined H264 encoding variants */
/* @{ */
/// ISO 14496-10 Annex B byte stream format
// pub const MMAL_ENCODING_VARIANT_H264_DEFAULT: u32 = 0;
/// ISO 14496-15 AVC stream format
pub const MMAL_ENCODING_VARIANT_H264_AVC1: c_uint = mmal_fourcc!('A', 'V', 'C', '1');
/// Implicitly delineated NAL units without emulation prevention
pub const MMAL_ENCODING_VARIANT_H264_RAW: c_uint = mmal_fourcc!('R', 'A', 'W', ' ');
/* @} */

/** \name Pre-defined MPEG4 audio encoding variants */
/* @{ */
/// Raw stream format
// pub const MMAL_ENCODING_VARIANT_MP4A_DEFAULT: u32 = 0;
/// ADTS stream format
pub const MMAL_ENCODING_VARIANT_MP4A_ADTS: c_uint = mmal_fourcc!('A', 'D', 'T', 'S');
/* @} */

/* @} MmalEncodingVariants List */

/** \defgroup MmalColorSpace List of pre-defined video color spaces
 * This defines a list of common color spaces. This list isn't exhaustive and is only
 * provided as a convenience to avoid clients having to use FourCC codes directly.
 * However components are allowed to define and use their own FourCC codes. */
/* @{ */

/// Unknown color space
// pub const MMAL_COLOR_SPACE_UNKNOWN: u32 = 0;
/// ITU-R BT.601-5 [SDTV]
pub const MMAL_COLOR_SPACE_ITUR_BT601: c_uint = mmal_fourcc!('Y', '6', '0', '1');
/// ITU-R BT.709-3 [HDTV]
pub const MMAL_COLOR_SPACE_ITUR_BT709: c_uint = mmal_fourcc!('Y', '7', '0', '9');
/// JPEG JFIF
pub const MMAL_COLOR_SPACE_JPEG_JFIF: c_uint = mmal_fourcc!('Y', 'J', 'F', 'I');
/// Title 47 Code of Federal Regulations (2003) 73.682 (a) (20)
pub const MMAL_COLOR_SPACE_FCC: c_uint = mmal_fourcc!('Y', 'F', 'C', 'C');
/// Society of Motion Picture and Television Engineers 240M (1999)
pub const MMAL_COLOR_SPACE_SMPTE240M: c_uint = mmal_fourcc!('Y', '2', '4', '0');
/// ITU-R BT.470-2 System M
pub const MMAL_COLOR_SPACE_BT470_2_M: c_uint = mmal_fourcc!('Y', '_', '_', 'M');
/// ITU-R BT.470-2 System BG
pub const MMAL_COLOR_SPACE_BT470_2_BG: c_uint = mmal_fourcc!('Y', '_', 'B', 'G');
/// JPEG JFIF, but with 16..255 luma
pub const MMAL_COLOR_SPACE_JFIF_Y16_255: c_uint = mmal_fourcc!('Y', 'Y', '1', '6');
/* @} MmalColorSpace List */

// mmal_events.h

/// Error event. Data contains a \ref MMAL_STATUS_T
pub const MMAL_EVENT_ERROR: c_uint = mmal_fourcc!('E', 'R', 'R', 'O');

/// End-of-stream event. Data contains a \ref MMAL_EVENT_END_OF_STREAM_T
pub const MMAL_EVENT_EOS: c_uint = mmal_fourcc!('E', 'E', 'O', 'S');

/// Format changed event. Data contains a \ref MMAL_EVENT_FORMAT_CHANGED_T
pub const MMAL_EVENT_FORMAT_CHANGED: c_uint = mmal_fourcc!('E', 'F', 'C', 'H');

/// Parameter changed event. Data contains the new parameter value, see
/// \ref MMAL_EVENT_PARAMETER_CHANGED_T
pub const MMAL_EVENT_PARAMETER_CHANGED: c_uint = mmal_fourcc!('E', 'P', 'C', 'H');

/// Note that there appears to be no constant for the null sink but it does exist in the
/// binaries.
/// If this ever breaks because C has this then we can delete this one.
pub const MMAL_COMPONENT_NULL_SINK: &'static [u8; 13usize] = b"vc.null_sink\0";

impl fmt::Display for MMAL_PARAMETER_CAMERA_INFO_CAMERA_T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}x{}",
            ::std::str::from_utf8(&self.camera_name).unwrap(),
            self.max_width,
            self.max_height
        )
    }
}

impl fmt::Display for MMAL_PARAMETER_CAMERA_INFO_T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Found {} camera(s)", self.num_cameras).unwrap();

        // We can't iterate over all cameras because we will always have 4.
        // Alternatively, we could iterate and break early. Not sure if that is more rust-y
        for index in 0..self.num_cameras {
            let camera = self.cameras[index as usize];
            write!(f, "\n  {}", camera).unwrap();
        }

        // TODO: flashes?

        Ok(())
    }
}

#[cfg(feature = "generate_bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "generate_bindings"))]
include!("bindings.rs");
