# VST plugin with Rust and Csound

Implements Audio-FX plugin based on Csound reberbsc opcode.
It produces VST3 and CLAP plugins. 

To produce plugins run:

```
just run
```

It builds shared library and generates bundled plugins for vst3 and clap.
Plugins are built with clack framework and clap-wrapper-rs library.

### TODO

* implement csound audio to VST connection

* fine-tune the UI

* make VST build work again (somehow clap-wrapper does not work with Csound.)
  I guess it happens because of dynamic linking. Post an issue and try to investigate
  static linking for Csound.
  
#### Static linking for csound (cure VST3 build)

We can produce static linking (google/deepseek around).
But there is a problem that to be completley static it has to include recursive sub dependencies.
There is a script to do it for Mac on Csound repo. I wonder if it works or can be adapted for
linux and windows.


### Examples:

* clack with gui, official clack gain FX example:
  https://github.com/prokopyl/clack/tree/main/plugin/examples/gain-gui

* nih-plug example:
    https://github.com/steckes/rust-audio-plugin/tree/main

* clack example:
  https://github.com/Kwarf/crabhowler

* csound audio processing for VST example in cpp + juce:
  https://github.com/gogins/csound-vst3/blob/main/CsoundVST3/Source/PluginProcessor.cpp#L432b

* fast single producer single consumer queue
https://github.com/wryzxec/PikaQ

* ringbuffer - for education
https://dev.to/codeapprentice/low-latency-rust-building-a-cache-friendly-lock-free-spsc-ring-buffer-in-rust-ddm


Rust fast SPSC queues:

* rtrb - https://github.com/mgeier/rtrb
* ringbuffer - https://github.com/aodr3w/llt-rs/tree/main/src/ring_buffer
* fq - https://crates.io/crates/fq
* nexus::queue - https://github.com/Abso1ut3Zer0/nexus
* spsc - https://github.com/1rishuraj/low-latency-rust/tree/main/spsc
