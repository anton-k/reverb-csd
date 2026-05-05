# VST plugin with Rust and Csound

Implements Audio-FX plugin based on Csound reberbsc opcode.
It produces VST3 and CLAP plugins. 

To produce plugins run:

```
just run
```

It builds shared library and generates bundled plugins for vst3 and clap.
Plugins are built with clack framework and clap-wrapper-rs library.

### Examples:

* nih-plug example:
    https://github.com/steckes/rust-audio-plugin/tree/main

* clack example:
  https://github.com/Kwarf/crabhowler

* csound audio processing for VST example in cpp + juce:
  https://github.com/gogins/csound-vst3/blob/main/CsoundVST3/Source/PluginProcessor.cpp#L432b

