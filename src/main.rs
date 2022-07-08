use std::fs;
use std::error::Error;
use std::process::Command;

fn main()
{
    let command = std::env::args().nth(1).expect("no argument given");

    let result = match command.as_str()
    {
        "temperature" => temp(),
        "cpu_freq" => cpu_freq(),
        "is_throttled" => is_throttled(),
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

fn is_throttled() -> Result<(), Box<dyn Error>>
{
    let output = Command::new("/usr/bin/vcgencmd")
        .arg("get_throttled")
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    let bits = output_str
        .split("=0x")
        .nth(1)
        .ok_or("vcgencmd unparseable output")?
        .trim();
    let bits = u32::from_str_radix(bits, 16)?;

    let under_volt = (bits & 0x1) != 0;
    let throttled = (bits & 0x4) != 0;
    let over_temp = (bits & 0x8) != 0;

    let message = if !throttled {
        ""
    } else if over_temp {
        "THROTTLE TMP!"
    } else if under_volt {
        "THROTTLE PWR!"
    } else {
        "THROTTLE ???!"
    };

    println!("{}", message);
    Ok(())
}