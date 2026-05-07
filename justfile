build:
    cargo build

run:
    cargo build --release
    cargo bundle target/release/libreverb_csd.so
    cp -r target/release/ReverbCsd.vst3/ ~/music/vst/
    # cp target/release/ReverbCsd.clap ~/music/clap/
    # cp target/release/libreverb_csd.so target/release/ReverbCsd.clap 
    clack-host-cpal --file-path target/release/ReverbCsd.clap 
    

    
