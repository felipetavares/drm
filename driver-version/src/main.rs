mod gpu;

use std::error::Error;

use gpu::GPU;

/// Prints the kernel driver version and exits
fn main() -> Result<(), Box<dyn Error>> {
    let gpu = GPU::open()?;
    let version = gpu.driver_version()?;

    println!(
        "Driver {}.{}.{}",
        version.major, version.minor, version.patchlevel,
    );

    println!("{}", version.date);
    println!("{}", version.name);
    println!("{}", version.desc);

    Ok(())
}
