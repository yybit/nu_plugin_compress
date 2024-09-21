use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_compress::CompressPlugin;

fn main() {
    serve_plugin(&CompressPlugin {}, MsgPackSerializer {})
}