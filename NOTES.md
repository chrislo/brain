* Todo list

- [ ] Allow the sampler to play different files
- [ ] Adopt the OSC message format used by Sonic Pi's [osmid](https://github.com/llloret/osmid)
- [ ] Pitch/rate control of samples
- [x] Make the sequencer do something more useful
- [ ] Message receive thread in sequencer to allow notes to be added to the pattern via OSC messages
- [ ] Correct for drift in main loop in sequencer. We know how many ticks have  been run and how long this tick has taken, so we should be able to sleep for the correct amount of time to avoid any drift
- [ ] Introduce Note type in sequencer to allow different pitches/samples to be played

* Notes

In [Tom Maisey's ADC talk](https://www.youtube.com/watch?v=yjri_jPLyU8) he introduces the idea of using immutable functions in a sequencer app. I think I could try something like that in Rust.
