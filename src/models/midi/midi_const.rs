pub(crate) const NOTE_COUNT: u32 = 12;

pub(crate) const NATURALS: [u32; 7] = [
    0,  // A
    2,  // B
    3,  // C
    5,  // D
    7,  // E
    8,  // F
    10, // G
];

pub(crate) const UNSHARPABLE_INDEX: [u32; 2] = [
    1, // B
    4, // E
];

pub(crate) const UNFLATTABLE_INDEX: [u32; 2] = [
    2, // C
    5, // F
];

pub(crate) const ACCIDENTS: [u32; 5] = [
    1,  // As/Bb
    4,  // Cs/Db
    6,  // Ds/Eb
    9,  // Fs/Gb
    11, // Gs/Ab
];

pub(crate) const NOTES: [u32; 12] = [
    0,  // A
    1,  // As/Bb
    2,  // B
    3,  // C
    4,  // Cs/Db
    5,  // D
    6,  // Ds/Eb
    7,  // E
    8,  // F
    9,  // Fs/Gb
    10, // G
    11, // Gs/Ab
];

pub(crate) const MIDI_OFFSET: u32 = 21;

pub fn get_id(natural: u32, octave: u32, accidental: bool) -> Result<u32, ()> {
    let mut uppable = true;
    let mut downable = true;

    if UNSHARPABLE_INDEX.contains(&natural) {
        uppable = false;
    } else if UNFLATTABLE_INDEX.contains(&natural) {
        downable = false;
    }
    let natural = NATURALS[natural as usize];
    if accidental && !(uppable || downable) {
        return Err(());
    }
    Ok(natural + octave * NOTE_COUNT + if accidental { 1 } else { 0 } + MIDI_OFFSET)
}
