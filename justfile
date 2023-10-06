fuzz:
    cargo +nightly fuzz run fuzz_parse -j 6

readme:
    cargo rdme
