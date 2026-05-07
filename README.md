# VST plugin with Rust and Csound

Implements Audio-FX plugin based on Csound reberbsc opcode.
It produces VST3 and CLAP plugins. 

To produce plugins run:

```
just run
```

It builds shared library and generates bundled plugins for vst3 and clap.
Plugins are built with clack-plugin framework and clap-wrapper-rs library.

### TODO

* implement csound audio to VST connection

* fine-tune the UI

* move CSD-bindings to separate repo.

* implement a simple csd_clack_bridge repo to simplify
  development of similiar plugins.

* fix bug: plugin UI does not opens on first close and re-open in the DAW (Reaper).

* what about presets?
 
### Examples:

* clack with gui, official clack gain FX example:
  https://github.com/prokopyl/clack/tree/main/plugin/examples/gain-gui

* clack example:
  https://github.com/Kwarf/crabhowler

* csound audio processing for VST example in cpp + juce:
  https://github.com/gogins/csound-vst3/blob/main/CsoundVST3/Source/PluginProcessor.cpp#L432b

* CLAP plugin tutorial:
  https://nakst.gitlab.io/tutorial/clap-part-1.html

### SPSC Queues (for Csound to Plugin audio communication)

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
* heapless:spsc

CPP queues:

* and doc: https://wryzxec.github.io/lockfree_spsc.html
  https://github.com/wryzxec/PikaQ

* cpp vs rust comparison
  https://nordvarg.com/blog/low-latency-cpp-rust

* SPSC tutorials:

  https://www.youtube.com/watch?v=K3P_Lmq6pw0&t=38s
  https://www.youtube.com/watch?v=8uAW5FQtcvE&t=2821s
