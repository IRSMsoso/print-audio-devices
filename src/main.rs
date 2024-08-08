use std::fmt::Write;

use cpal::{
    available_hosts, Device, host_from_id, SupportedBufferSize, SupportedStreamConfigRange,
};
use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> anyhow::Result<()> {
    println!("Finding device info...");

    let mut info_string = String::new();

    let host_ids = available_hosts();

    for host_id in host_ids {
        let host = host_from_id(host_id)?;

        let devices: Vec<Device> = host.devices()?.collect();

        info_string.write_fmt(format_args!(
            "Host: {} [{} device(s)]\n",
            host_id.name(),
            devices.len()
        ))?;

        for device in devices {
            info_string.write_fmt(format_args!("    {}\n", device.name()?))?;

            info_string.write_fmt(format_args!("        Input:\n"))?;
            let Ok(input_configs) = device.supported_input_configs() else {
                info_string.write_fmt(format_args!("            Disconnected"))?;
                continue;
            };
            let input_configs: Vec<SupportedStreamConfigRange> = input_configs.collect();

            for input_config in input_configs {
                info_string.write_fmt(format_args!(
                    "            [{} channel(s)] [{}Hz -> {}Hz] [{} samples] [format: {}]\n",
                    input_config.channels(),
                    input_config.min_sample_rate().0,
                    input_config.max_sample_rate().0,
                    match input_config.buffer_size() {
                        SupportedBufferSize::Range { min, max } => {
                            format!("{} -> {}", min, max)
                        }
                        SupportedBufferSize::Unknown => "Unknown".to_owned(),
                    },
                    input_config.sample_format()
                ))?;
            }
        }
    }

    println!("{}", info_string);

    Ok(())
}
