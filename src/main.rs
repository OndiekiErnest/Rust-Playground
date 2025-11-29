struct Excerpt<'a, 'b> {
    part: &'a str,
    link: &'b str,
}

fn main() {
    let x = Excerpt {
        part: "The good part...",
        link: "link.to",
    };
    println!("{}: {}", x.link, x.part);
}
