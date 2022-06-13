use std::fs;
use std::net::SocketAddr;
use std::error::Error;
use std::process::Command;

fn main()
{
    let command = std::env::args().nth(1).expect("no argument given");

    let result = match command.as_str()
    {
        "temperature" => temp(),
        "cpu_freq" => cpu_freq(),
        _ => panic!("Unknown argument {0}", command)
    };

    result.expect("");
}

fn temp() -> Result<(), Box<dyn Error>>
{
    let value = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?
        .trim()
        .to_string()
        .parse::<f32>()?;

    let temp = value / 1000.0;

    println!("{:.2}", temp);
    Ok(())
}

fn cpu_freq() -> Result<(), Box<dyn Error>>
{
    let output = Command::new("/usr/bin/vcgencmd")
        .arg("measure_clock")
        .arg("arm")
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    let freq = output_str
        .split("=")
        .nth(1)
        .ok_or("vcgencmd unparseable output")?
        .trim()
        .parse::<f64>()?
        ;

    println!("{:.2}", freq / 1000000.0);

    Ok(())
}