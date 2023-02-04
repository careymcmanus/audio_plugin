use nih_plug::prelude::nih_export_standalone;

use AudioPlugin;

fn main() {
    nih_export_standalone::<AudioPlugin>();
}
