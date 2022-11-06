use ranityeth_lib::conf::config::get_config;

fn main() {
    let config = get_config();
    ranityeth_lib::thread::run(config)
}
