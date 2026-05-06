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

* make local version of params (see clack example)

* implement csound update of the params

* implement csound audio to VST connection

* fine-tune the UI

### Examples:

* clack with gui, official clack gain FX example:
  https://github.com/prokopyl/clack/tree/main/plugin/examples/gain-gui

* nih-plug example:
    https://github.com/steckes/rust-audio-plugin/tree/main

* clack example:
  https://github.com/Kwarf/crabhowler

* csound audio processing for VST example in cpp + juce:
  https://github.com/gogins/csound-vst3/blob/main/CsoundVST3/Source/PluginProcessor.cpp#L432b

