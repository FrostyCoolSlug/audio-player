use std::any::Any;
use cpal::traits::{DeviceTrait, HostTrait};
use crate::AudioConfiguration;

pub struct CpalConfiguration {

}

impl CpalConfiguration {
    fn new() -> Self {
        Self {}
    }
}

impl AudioConfiguration for CpalConfiguration {
    fn get_outputs(&mut self) -> Vec<String> {
        let mut list: Vec<String> = vec![];

        let available_hosts = cpal::available_hosts();
        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id).unwrap();
            let devices = host.devices().unwrap();
            for (_device_index, device) in devices.enumerate() {
                if device.supported_output_configs().is_err() {
                    println!("No output configs? {}", device.name().unwrap());
                    continue;
                }
                list.push(format!("{}*{}", host_id.name(), device.name().unwrap()));
            }
        }
        list
    }

    fn get_inputs(&mut self) -> Vec<String> {
        let mut list: Vec<String> = vec![];

        let available_hosts = cpal::available_hosts();
        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id).unwrap();
            let devices = host.devices().unwrap();
            for (_device_index, device) in devices.enumerate() {
                if device.supported_input_configs().is_err() {
                    println!("No Input Configs? {}", device.name().unwrap());
                    continue;
                }
                list.push(format!("{}*{}", host_id.name(), device.name().unwrap()));
            }
        }
        list
    }
}

pub fn get_configuration() -> CpalConfiguration {
    CpalConfiguration::new()
}