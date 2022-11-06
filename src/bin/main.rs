use ranityeth::conf::config::get_config;

fn main() {
    let config = get_config();
    ranityeth::thread::run(config)
}
