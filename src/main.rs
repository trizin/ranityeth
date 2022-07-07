use vanityeth::conf::config::get_config;

static THREADS_COUNT: u32 = 20;

fn main() {
    let config = get_config();

    vanityeth::thread::run(THREADS_COUNT, config)
}
