fn main() {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .build()
        .unwrap();
    let auctions = settings.get_array("content.auctions").unwrap();
    for v in auctions {
        println!("{}", v);
    }
}
