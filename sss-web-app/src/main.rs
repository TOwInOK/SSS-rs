pub fn trunk_launch() {
    use leptos::mount::mount_to_body;
    use sss_web_app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

fn main() {
    trunk_launch()
}
