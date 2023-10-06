use crate::client_styles::azureus;
use crate::client_styles::mainline::MainlineVersion;
use crate::client_styles::shadow::ShadowVersion;
use std::fmt;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum BitCometMod {
    Solidox,
    Mod2,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum Version {
    Azureus(azureus::version_styles::Styles),

    Shadow(ShadowVersion),

    Mainline(MainlineVersion),

    BitSpirit(u8),
    BitCometBitLord(u8, u8, Option<BitCometMod>),

    // for special cases when it's just one version
    Fixed(&'static str),

    // for cases when I know that a version is there, I just don't parse it yet
    // everything that uses it is a "todo"
    Unknown,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Azureus(x) => x.fmt(f),
            Self::Shadow(x) => x.fmt(f),
            Self::Mainline(x) => x.fmt(f),
            Self::BitSpirit(v1) => write!(f, "{v1}"),
            Self::BitCometBitLord(v1, v2, None) => write!(f, "{v1}.{v2}"),
            Self::BitCometBitLord(v1, v2, Some(m)) => {
                let suffix = match m {
                    BitCometMod::Mod2 => " (Mod 2)",
                    BitCometMod::Solidox => " (Solidox Mod)",
                };
                write!(f, "{v1}.{v2}{suffix}")
            }
            Self::Fixed(x) => write!(f, "{x}"),
            Self::Unknown => write!(f, "[unknown version]"),
        }
    }
}
