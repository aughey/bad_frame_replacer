use eyre::Result;
use std::{
    io::Read,
    process::{Command, Stdio},
};

fn main() -> Result<()> {
    // split cmd into space delimited outputs
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg("/c/jha/8.mov")
        .arg("-f")
        .arg("rawvideo")
        .arg("-vcodec")
        .arg("rawvideo")
        .arg("-pix_fmt")
        .arg("rgb24")
        .arg("-");
    cmd.stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| eyre::eyre!("failed to get stdout"))?;

    const WIDTH: usize = 3840;
    const HEIGHT: usize = 2160;
    let mut frame = vec![0u8; WIDTH * HEIGHT * 3];
    const FIRSTLINE_RANGE: std::ops::Range<usize> = 0..WIDTH * 3;

    let mut framecount = 0;
    println!("DFSP");
    loop {
        // read data to fill frame
        if let Err(_) = stdout.read_exact(&mut frame) {
            break;
        }

        let first_line = &frame[FIRSTLINE_RANGE];
        let average_color = first_line.chunks(3).fold((0, 0, 0), |acc, pixel| {
            (
                acc.0 + pixel[0] as u32,
                acc.1 + pixel[1] as u32,
                acc.2 + pixel[2] as u32,
            )
        });
        let average_color = (
            average_color.0 / WIDTH as u32,
            average_color.1 / WIDTH as u32,
            average_color.2 / WIDTH as u32,
        );

        let search_for = (89u32, 5u32, 16u32);
        let diff = (average_color.0 as i32 - search_for.0 as i32).abs()
            + (average_color.1 as i32 - search_for.1 as i32).abs()
            + (average_color.2 as i32 - search_for.2 as i32).abs();
        let level = if diff < 10 { 1 } else { 0 };
        println!("{framecount} {level}");

        framecount += 1;
    }
    Ok(())
}
