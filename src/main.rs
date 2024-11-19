use libva::{
    Display, DrmDeviceIterator,
    VAConfigAttribType::{VAConfigAttribMaxPictureHeight, VAConfigAttribMaxPictureWidth},
};

fn main() {
    env_logger::init();

    for path in DrmDeviceIterator::default() {
        let display = Display::open_drm_display(path).expect("open display");
        let vendor = display.query_vendor_string().expect("vendor");
        println!("Vendor: {vendor}");
        println!("  Image Profiles:");
        let image_profiles = display.query_image_formats().expect("image formats");
        for p in image_profiles {
            println!("    {}: {p:?}", fourcc_to_string(p.fourcc));
        }

        println!("  Config Profiles:");
        let config_profiles = display
            .query_config_profiles()
            .expect("config profiles")
            .into_iter()
            .map(VAProfile::from);
        for p in config_profiles {
            println!("    {p:?}:");
            let eps = display
                .query_config_entrypoints(p.into())
                .expect("entrypoints")
                .into_iter()
                .map(VAEntrypoint::from);
            for ep in eps {
                let mut attributes = [
                    libva::VAConfigAttrib {
                        type_: VAConfigAttribMaxPictureWidth,
                        value: 0,
                    },
                    libva::VAConfigAttrib {
                        type_: VAConfigAttribMaxPictureHeight,
                        value: 0,
                    },
                ];
                display
                    .get_config_attributes(p.into(), ep.into(), &mut attributes)
                    .expect("config attributes");
                println!(
                    "      {ep:?}: {}x{}",
                    attributes[0].value, attributes[1].value
                );
            }
        }
    }
}

fn fourcc_to_string(fourcc: u32) -> String {
    String::from_utf8_lossy(&fourcc.to_ne_bytes()).into()
}

impl From<i32> for VAProfile {
    fn from(value: i32) -> Self {
        if value > VAProfile::VVCMultilayerMain10.into() || value < VAProfile::None.into() {
            Self::Unknown
        } else {
            unsafe { std::mem::transmute::<i32, VAProfile>(value) }
        }
    }
}

impl From<VAProfile> for i32 {
    fn from(val: VAProfile) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
enum VAProfile {
    Unknown = -2,
    None = -1,
    MPEG2Simple = 0,
    MPEG2Main = 1,
    MPEG4Simple = 2,
    MPEG4AdvancedSimple = 3,
    MPEG4Main = 4,
    H264Baseline = 5,
    H264Main = 6,
    H264High = 7,
    VC1Simple = 8,
    VC1Main = 9,
    VC1Advanced = 10,
    H263Baseline = 11,
    JPEGBaseline = 12,
    H264ConstrainedBaseline = 13,
    VP8Version0_3 = 14,
    H264MultiviewHigh = 15,
    H264StereoHigh = 16,
    HEVCMain = 17,
    HEVCMain10 = 18,
    VP9Profile0 = 19,
    VP9Profile1 = 20,
    VP9Profile2 = 21,
    VP9Profile3 = 22,
    HEVCMain12 = 23,
    HEVCMain422_10 = 24,
    HEVCMain422_12 = 25,
    HEVCMain444 = 26,
    HEVCMain444_10 = 27,
    HEVCMain444_12 = 28,
    HEVCSccMain = 29,
    HEVCSccMain10 = 30,
    HEVCSccMain444 = 31,
    AV1Profile0 = 32,
    AV1Profile1 = 33,
    HEVCSccMain444_10 = 34,
    Protected = 35,
    H264High10 = 36,
    VVCMain10 = 37,
    VVCMultilayerMain10 = 38,
}

impl From<u32> for VAEntrypoint {
    fn from(value: u32) -> Self {
        if value > VAEntrypoint::ProtectedContent.into() {
            Self::Unknown
        } else {
            unsafe { std::mem::transmute::<u32, VAEntrypoint>(value) }
        }
    }
}

impl From<VAEntrypoint> for u32 {
    fn from(val: VAEntrypoint) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

#[allow(dead_code, clippy::upper_case_acronyms)]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum VAEntrypoint {
    Unknown = 0,
    VLD = 1,
    IZZ = 2,
    IDCT = 3,
    MoComp = 4,
    Deblocking = 5,
    EncSlice = 6,
    EncPicture = 7,
    EncSliceLP = 8,
    VideoProc = 10,
    FEI = 11,
    Stats = 12,
    ProtectedTEEComm = 13,
    ProtectedContent = 14,
}
