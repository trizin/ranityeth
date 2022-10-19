use vanityeth::conf::config::get_config;

fn main() {
    let config = get_config();
    vanityeth::thread::run(config)
}
