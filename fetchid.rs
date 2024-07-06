use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("page_source.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Extract the captcha ID
    if let Some(start) = contents.find("embeddedSessionID text") {
        let id_start = contents[start..].find('>').unwrap() + start + 1;
        let id_end = contents[id_start..].find('<').unwrap() + id_start;
        let captcha_id = &contents[id_start..id_end];
        println!("Captcha ID: {}", captcha_id);
    } else {
        println!("Captcha ID not found in the page source.");
    }

    Ok(())
}
