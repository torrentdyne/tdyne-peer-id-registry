use crate::client_styles::azureus::version_styles::four_base16::FourBase16;
use crate::client_styles::azureus::version_styles::one_base10_unknown::OneBase10Unknown;
use crate::client_styles::azureus::version_styles::three_base16::ThreeBase16;
use crate::client_styles::azureus::version_styles::three_base16_alpha_beta::ThreeBase16AlphaBeta;
use crate::client_styles::azureus::version_styles::transmission::Transmission;
use crate::client_styles::azureus::version_styles::two_base16::TwoBase16;
use crate::client_styles::azureus::version_styles::two_maj_two_min_base10::TwoMajTwoMinBase10;
use std::fmt;

pub(crate) mod four_base16;
pub(crate) mod one_base10_unknown;
pub(crate) mod three_base16;
pub(crate) mod three_base16_alpha_beta;
pub(crate) mod transmission;
pub(crate) mod two_base16;
pub(crate) mod two_maj_two_min_base10;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Styles {
    // I note correspondence to Webtorrent's parsers (that in turn come from much older code)
    // in comments below
    // VER_AZ_THREE_DIGITS
    // VER_AZ_DELUGE is the same and just allows base16
    ThreeBase16(ThreeBase16),
    // VER_AZ_THREE_DIGITS_PLUS_MNEMONIC
    ThreeBase16AlphaBeta(ThreeBase16AlphaBeta),
    // VER_AZ_FOUR_DIGITS
    FourBase16(FourBase16),
    // VER_AZ_TWO_MAJ_TWO_MIN
    TwoMajTwoMinBase10(TwoMajTwoMinBase10),
    // VER_AZ_SKIP_FIRST_ONE_MAJ_TWO_MIN
    // - this is just TwoMajTwoMinBase10, webtorrent skips leading zeros for some reason
    //   see http://wiki.bitcomet.com/inside_bitcomet#peerid_format
    //   and https://www.bittorrent.org/beps/bep_0020.html for flashget
    // VER_AZ_KTORRENT_STYLE
    // - seems that VER_AZ_KTORRENT_STYLE is just three decimal numbers
    //   https://github.com/KDE/libktorrent/blob/30adde89afcc73c0018eb30844b82faae1703dd1/src/version.cpp#L24
    //   I can use three_base_16 instead
    // VER_AZ_TRANSMISSION_STYLE
    Transmission(Transmission),
    // VER_AZ_WEBTORRENT_STYLE
    // -  VER_AZ_WEBTORRENT_STYLE seems identical to VER_AZ_TWO_MAJ_TWO_MIN, except
    //    that webtorrent does a weird thing with the leading zero?
    // VER_AZ_THREE_ALPHANUMERIC_DIGITS
    // - VER_AZ_THREE_ALPHANUMERIC_DIGITS is buggy?
    //   it's only used for libtorrent (both rasterbar and rakshasa)
    //   see
    //   https://github.com/arvidn/libtorrent/blob/5ce4c516479085eda9327d35891b7732d6d5037b/src/settings_pack.cpp#L151
    //   and
    //   https://github.com/rakshasa/libtorrent/blob/91f8cf4b0358d9b4480079ca7798fa7d9aec76b5/configure.ac#L10
    //   (and those lines' history)
    //   it seems that both actually use 3 hex digits
    // VER_NONE is handled with an upstream Option
    OneBase10Unknown(OneBase10Unknown),
    TwoBase16(TwoBase16),
}

impl fmt::Display for Styles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ThreeBase16(x) => x.fmt(f),
            Self::ThreeBase16AlphaBeta(x) => x.fmt(f),
            Self::FourBase16(x) => x.fmt(f),
            Self::TwoMajTwoMinBase10(x) => x.fmt(f),
            Self::Transmission(x) => x.fmt(f),
            Self::OneBase10Unknown(x) => x.fmt(f),
            Self::TwoBase16(x) => x.fmt(f),
        }
    }
}
