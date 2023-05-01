use cpal::{Sample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let supported_config = device.default_output_config().unwrap();

    println!("Device: {}, Using config: {:?}", device.name().expect("flute"), supported_config);

    let config = supported_config.into();

    let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));
}

fn write_silence(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    let mut counter = 0;
    for sample in data.iter_mut() {
        let s = if (counter / 20) % 2 == 0 { &1.0 } else { &0.0 };
        counter = counter + 1;
        *sample = Sample::from(s);
    }
    println!("{:?}", data);
}
