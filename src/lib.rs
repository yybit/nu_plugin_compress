mod compress;
mod decompress;
use compress::CompressCommand;
use decompress::DecompressCommand;
use nu_plugin::Plugin;

pub struct CompressPlugin;

impl Plugin for CompressPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(CompressCommand::Gzip),
            Box::new(CompressCommand::Zstd),
            Box::new(CompressCommand::Xz),
            Box::new(CompressCommand::Bzip2),
            Box::new(DecompressCommand::Gzip),
            Box::new(DecompressCommand::Zstd),
            Box::new(DecompressCommand::Xz),
            Box::new(DecompressCommand::Bzip2),
        ]
    }
}
