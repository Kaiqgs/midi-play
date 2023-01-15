#[cfg(test)]
pub mod common;
#[cfg(test)]
pub mod components;
#[cfg(test)]
pub mod controllers;
#[cfg(test)]
pub mod midiplay;
#[cfg(test)]
pub mod models;

/*
Tests for MIDI Input;
    Test that the class can correctly read input from all 88 keys on a piano.
    Test that the class can handle multiple keys being pressed at the same time.
    Test that the class can correctly identify when a key is released.
    Test that the class can handle rapid successive presses of the same key.
    Test that the class can correctly handle input from a sustain pedal.
    Test that the class can correctly handle input from a damper pedal.
    Test that the class can handle input from a piano with weighted keys.
    Test that the class can handle input from a piano with unweighted keys.
    Test that the class can handle input from a digital piano with simulated weighting.
    Test that the class can handle input from a MIDI keyboard.

Tests for displaying:
    Test that the class correctly displays sheet music on the screen in realtime as the user plays the piano.
    Test that the class correctly highlights the current measure being played.
    Test that the class correctly displays notes and their corresponding durations (e.g. whole notes, half notes, quarter notes, etc.).
    Test that the class correctly displays accidentals (sharp, flat, natural) and key signatures.
    Test that the class can handle multiple voices and correctly displays them on separate staffs.
    Test that the class can handle time signature changes during a piece of music.
    Test that the class can handle fermatas and other musical notations.
    Test that the class can handle and display repeat signs, first and second endings, and other measures that require repeating.
    Test that the class can handle and display dynamics and other performance instructions.
    Test that the class can handle and display page turns during a piece of music.

Tests for loading midi and converting:
    Test that the class can successfully parse a MIDI file and extract the information about the notes and their timing.
    Test that the class can convert the MIDI data to a 2D space using the correct mapping of pitches to y-coordinates and times to x-coordinates.
    Test that the class can handle different tempos and resolutions in the MIDI data.
    Test that the class can handle different types of MIDI events, such as note on/off events, pitch bend events, and control change events.
    Test that the class can handle MIDI files with multiple tracks and channels.
    Test that the class can handle MIDI files with different time signatures and meter changes.
    Test that the class can handle MIDI files with different note durations, including legato and staccato notes.
    Test that the class can handle MIDI files with different velocity values, including loud and soft notes.
 */
