use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
use std::fs;
use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
//First cli app that i made for screen shotting a website and saving it to a file
//You can use a crate that does this for you but i wanted to make my ownp
fn main() {
    let mut s = String::new();
    let mut x_cords = String::new();
    let mut y_cords = String::new();
    println!("Enter URL: ");
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    println!("Enter X size pixles: ");

    stdin()
        .read_line(&mut x_cords)
        .expect("Did not enter a correct string");

    println!("Enter Y size pixles: ");

    stdin()
        .read_line(&mut y_cords)
        .expect("Did not enter a correct string");
    take_screenshot(s, x_cords, y_cords).unwrap();

    println!("Done");
}

fn take_screenshot(
    url: String,
    x_size: String,
    y_size: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Taking screenshot of {}", url);
    println!("X size: {}", x_size);
    println!("Y size: {}", y_size);

    let x_size: u32 = x_size.trim().parse().unwrap();
    let y_size: u32 = y_size.trim().parse().unwrap();
    let options = LaunchOptionsBuilder::default()
        .window_size(Some((x_size, y_size)))
        .build()?;
    let browser = Browser::new(options)?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(&url)?;

    tab.wait_until_navigated()?;

    sleep(Duration::from_secs(3));

    let png_data = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;

    fs::write("screenshot.png", png_data)?;

    Ok(())
}
