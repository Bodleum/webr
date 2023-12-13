#![recursion_limit = "512"]

use webr::{create_footer, get_page, init, prelude::*};

fn main() -> Result<()> {
    init()?;

    let socials = [
        ("Github", "https://github.com/Bodleum"),
        ("Gitlab", "https://gitlab.com/Bodleum"),
        ("Instagram", "https://www.instagram.com/_thebakerdan"),
    ];
    let f = create_footer("contact@daniellaing.com", socials);
    println!("{f}");

    let page = get_page();
    println!("{page}");

    Ok(())
}
