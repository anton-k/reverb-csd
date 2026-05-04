build:
    cargo build

run:
    cargo build --release
    cargo bundle target/release/libreverb_csd.so
    cp -r target/release/ReverbCsd.vst3/ ~/music/vst/

    
