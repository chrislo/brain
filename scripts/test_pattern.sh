#!/usr/bin/env bash

right () {
    sendosc 127.0.0.1 57120 /midi/atom/1/1/control_change i 102 i 127
}

note_on() {
    sendosc 127.0.0.1 57120 /midi/atom/1/10/note_on i $1
}

pad() {
    note=$(($1 + 35))
    note_on $note
}

# Kick
pad 1
pad 5
pad 9
pad 13


# Switch to closed hat
right
right

pad 2
pad 8
pad 10

# Switch to open hat
right

pad 3
pad 7
pad 11
pad 15

# Switch to clap
right

pad 5
pad 13
