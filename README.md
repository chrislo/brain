# Brain

Electronic musicians, particularly those who use mostly hardware rather than software, usually have a "brain" in their setup. The brain is responsible for controlling the rest of the equipment and typically has at least a (MIDI/CV) sequencer and often sampling, synthesis and effects built in.

This is my attempt to turn a Raspberry Pi into the brain of my music setup.

The [sequencer](/sequencer) is written in Rust and is controlled using [OSC messages](http://opensoundcontrol.org/).

Because I don't have a MIDI-controllable sampler, the project also contains a simple [sampler](/sampler) written using Supercollider.

I have a [PreSonus ATOM](https://www.presonus.com/products/ATOM) pad controller, and a [QuNexus](https://www.keithmcmillen.com/products/qunexus/) keyboard controller, so I want to be able to talk to the sequencer using MIDI. To do that, I'm using [osmid](https://github.com/llloret/osmid) the MIDI<->OSC bridge used in [Sonic Pi](https://github.com/samaaron/sonic-pi).

## Status

I'm currently working towards something "minimally viable" for making music. For me that means:

- A simple step sequencer that can control the internal sampler
- A sampler that can load a bunch of pre-prepared samples from a folder and play them back
- A way to receive step input from the PreSonus ATOM
- A basic "UI" to give me some indication what is happening. This might be lighting up the keys on the ATOM or something else
- Some way of controlling external gear. Perhaps as simple as outputting MIDI clock to that I can sync the sequencer of my [Korg Volca Keys](https://www.korg.com/us/products/dj/volca_keys/).

I could obviously take this idea much further, but it's useful to have something to focus on for version 1. I'm particularly inspired by the [Squarp Pyramid](https://squarp.net/pyramid) sequencer with its performance tools, MIDI effects and so on.

## Why?

Mostly as an excuse to teach myself Rust, learn more about real-time programming and to do something creative with computers. I'm really inspired by other Raspberry Pi-based music hardware projects such as the [monome norns](https://market.monome.org/products/norns-shield-kit) or , [OTTO](https://github.com/OTTO-project/OTTO/). These projects involve some custom hardware though, and I'm quite interested to see how far I can push a "stock" Raspberry Pi talking to my existing MIDI devices.
