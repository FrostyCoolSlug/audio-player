use crate::audio::AudioConfiguration;
use crate::audio::get_configuration;

//#[cfg(target_os = "linux")]
mod pulse;

//#[cfg(not(target_os = "linux"))]
mod cpal;

mod audio;

fn main() {
    let mut configuration = get_configuration();
    println!("{:?}", configuration.get_outputs());
    println!("{:?}", configuration.get_inputs());
}
