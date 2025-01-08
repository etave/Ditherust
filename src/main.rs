
use ditherust::handlers::DitherustArgs;
use image::ImageError;

fn main() -> Result<(), ImageError>{
    let args: DitherustArgs = argh::from_env();
    println!("{:?}", args);
    let path_in = args.input;
    return Ok(())
}