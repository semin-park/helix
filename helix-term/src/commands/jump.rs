pub(crate) mod annotate;
pub(crate) mod locations;
pub(crate) mod score;
pub(crate) mod sequencer;

pub use annotate::{apply_dimming, clear_dimming, show_key_annotations_with_callback, JUMP_KEYS};
pub use locations::{find_all_char_occurrences, find_all_identifiers_in_view};
pub use score::sort_jump_targets;
pub use sequencer::{JumpAnnotation, JumpSequence, JumpSequencer, TrieNode};
