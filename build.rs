use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

struct TagMap {
    filename: &'static str,
    definition: &'static str,
    phf: String,
}

fn codegen_tags_azureus() -> TagMap {
    let filename = "codegen_tags_azureus.rs";
    let definition = "static TAGS: phf::Map<&'static [u8], Azureus> =";
    let tags: Vec<(&[u8], &str)> = vec![
        (b"A~", "Azureus::AresThreeDigits"),
        (b"AG", "Azureus::AresThreeDigits"),
        (b"AN", "Azureus::AresFourDigits"),
        (b"AR", "Azureus::AresFourDigits"),
        (b"AV", "Azureus::Avicora"),
        (b"AX", "Azureus::BitPump"),
        (b"AT", "Azureus::Artemis"),
        (b"AZ", "Azureus::Vuze"),
        (b"BB", "Azureus::BitBuddy"),
        (b"BC", "Azureus::BitComet"),
        (b"BE", "Azureus::BitTorrentSDK"),
        (b"BF", "Azureus::BitFlu"),
        (b"BG", "Azureus::Btg"),
        (b"BI", "Azureus::BiglyBt"),
        (b"bk", "Azureus::BitKitten"),
        (b"BR", "Azureus::BitRocket"),
        (b"BS", "Azureus::BTSlave"),
        (b"BT", "Azureus::BitTorrent"),
        (b"BW", "Azureus::BitWombat"),
        (b"BX", "Azureus::BittorrentX"),
        (b"CB", "Azureus::ShareazaPlus"),
        (b"CD", "Azureus::EnhancedCTorrent"),
        (b"CT", "Azureus::CTorrent"),
        (b"DP", "Azureus::PropogateDataClient"),
        (b"DE", "Azureus::Deluge"),
        (b"EB", "Azureus::EBit"),
        (b"ES", "Azureus::ElectricSheep"),
        (b"FC", "Azureus::FileCroc"),
        (b"FD", "Azureus::FreeDownloadManager"),
        (b"FG", "Azureus::FlashGet"),
        (b"FL", "Azureus::Folx"),
        (b"FX", "Azureus::FreeboxBitTorrent"),
        (b"FT", "Azureus::FoxTorrentRedSwoosh"),
        (b"GR", "Azureus::GetRight"),
        (b"GS", "Azureus::GSTorrent"),
        (b"HL", "Azureus::Halite"),
        (b"HN", "Azureus::Hydranode"),
        (b"KG", "Azureus::KGet"),
        (b"KT", "Azureus::KTorrent"),
        (b"LC", "Azureus::LeechCraft"),
        (b"LH", "Azureus::LhAbc"),
        (b"LK", "Azureus::Linkage"),
        (b"LP", "Azureus::Lphant"),
        (b"LT", "Azureus::LibtorrentRasterbar"),
        (b"lt", "Azureus::LibTorrentRakshasa"),
        (b"LW", "Azureus::LimeWire"),
        (b"MG", "Azureus::MediaGet"),
        (b"MO", "Azureus::MonoTorrent"),
        (b"MP", "Azureus::MooPolice"),
        (b"MR", "Azureus::Miro"),
        (b"MT", "Azureus::MoonlightTorrent"),
        (b"NE", "Azureus::BTNextEvolution"),
        (b"NX", "Azureus::NetTransport"),
        (b"OS", "Azureus::OneSwarm"),
        (b"OT", "Azureus::OmegaTorrent"),
        (b"PC", "Azureus::CacheLogic"),
        (b"PT", "Azureus::PopcornTime"),
        (b"PD", "Azureus::Pando"),
        (b"PE", "Azureus::PeerProject"),
        (b"pX", "Azureus::PHoeniX"),
        (b"qB", "Azureus::QBittorrent"),
        (b"QD", "Azureus::QqDownload"),
        (b"RM", "Azureus::RumTorrent"),
        (b"RT", "Azureus::Retriever"),
        (b"RZ", "Azureus::RezTorrent"),
        (b"S~", "Azureus::ShareazaAlphaBeta"),
        (b"SB", "Azureus::SwiftBit"),
        (b"SD", "Azureus::Xunlei"),
        (b"SG", "Azureus::GSTorrent"),
        (b"SN", "Azureus::ShareNET"),
        (b"SP", "Azureus::BitSpirit"),
        (b"SS", "Azureus::SwarmScope"),
        (b"ST", "Azureus::SymTorrent"),
        (b"st", "Azureus::SharkTorrent"),
        (b"SZ", "Azureus::Shareaza"),
        (b"TG", "Azureus::TorrentGO"),
        (b"TN", "Azureus::TorrentDotNET"),
        (b"TR", "Azureus::Transmission"),
        (b"TS", "Azureus::TorrentStorm"),
        (b"TT", "Azureus::TuoTu"),
        (b"UL", "Azureus::ULeecher"),
        (b"UE", "Azureus::UTorrentEmbedded"),
        (b"UT", "Azureus::UTorrent"),
        (b"UM", "Azureus::UTorrentMac"),
        (b"UW", "Azureus::UTorrentWeb"),
        (b"WD", "Azureus::WebTorrentDesktop"),
        (b"WT", "Azureus::Bitlet"),
        (b"WW", "Azureus::WebTorrent"),
        (b"WY", "Azureus::FireTorrent"),
        (b"VG", "Azureus::Vagaa"),
        (b"XL", "Azureus::Xunlei"),
        (b"XT", "Azureus::XanTorrent"),
        (b"XF", "Azureus::Xfplay"),
        (b"XX", "Azureus::XTorrent"),
        (b"XC", "Azureus::XTorrent"),
        (b"ZT", "Azureus::ZipTorrent"),
        (b"7T", "Azureus::ATorrent"),
        (b"ZO", "Azureus::Zona"),
        (b"#@", "Azureus::InvalidPeerId"),
    ];

    TagMap {
        filename,
        definition,
        phf: tags
            .into_iter()
            .fold(phf_codegen::Map::new(), |mut builder, (k, v)| {
                builder.entry(k, v);
                builder
            })
            .build()
            .to_string(),
    }
}

fn codegen_tags_shadow() -> TagMap {
    let filename = "codegen_tags_shadow.rs";
    let definition = "static TAGS: phf::Map<u8, Shadow> =";
    let tags = vec![
        (b'A', "Shadow::Abc"),
        (b'O', "Shadow::OspreyPermaseed"),
        (b'Q', "Shadow::BTQueue"),
        (b'R', "Shadow::Tribler"),
        (b'S', "Shadow::Shad0w"),
        (b'T', "Shadow::BitTornado"),
        (b'U', "Shadow::UPnPNat"),
    ];

    TagMap {
        filename,
        definition,
        phf: tags
            .into_iter()
            .fold(phf_codegen::Map::new(), |mut builder, (k, v)| {
                builder.entry(k, v);
                builder
            })
            .build()
            .to_string(),
    }
}

fn main() {
    let all_tag_maps = vec![codegen_tags_azureus(), codegen_tags_shadow()];

    println!("cargo:rerun-if-changed=build.rs");

    for tag_map in all_tag_maps {
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join(tag_map.filename);

        let mut file = BufWriter::new(File::create(&path).unwrap());

        writeln!(&mut file, "{}\n{};", tag_map.definition, tag_map.phf).unwrap();
    }
}
