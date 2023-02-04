use nih_plug::prelude::*;

use audio_plugin::AudioPlugin;

fn main() {
    nih_export_standalone::<AudioPlugin>();
}
