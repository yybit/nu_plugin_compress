use std::io::{BufReader, Cursor, Read};

use crate::CompressPlugin;
use bzip2::bufread::BzEncoder;
use nu_plugin::PluginCommand;
use nu_protocol::{
    ByteStream, ByteStreamType, Category, LabeledError, PipelineData, Signature, Span, SyntaxShape,
    Type, Value,
};
use xz2::bufread::XzEncoder;

pub enum CompressCommand {
    Gzip,
    Zstd,
    Xz,
    Bzip2,
}

impl CompressCommand {
    fn compress(
        &self,
        r: impl Read + Send + 'static,
        span: Span,
        engine: &nu_plugin::EngineInterface,
        level: i32,
    ) -> Result<ByteStream, LabeledError> {
        let reader = BufReader::new(r);
        let stream = match self {
            CompressCommand::Gzip => ByteStream::read(
                flate2::read::GzEncoder::new(reader, flate2::Compression::new(level as u32)),
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
            CompressCommand::Zstd => ByteStream::read(
                zstd::stream::read::Encoder::new(reader, level)
                    .map_err(|e| LabeledError::new(e.to_string()))?,
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
            CompressCommand::Xz => ByteStream::read(
                XzEncoder::new(reader, level as u32),
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
            CompressCommand::Bzip2 => ByteStream::read(
                BzEncoder::new(reader, bzip2::Compression::new(level as u32)),
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
        };

        Ok(stream)
    }
}

impl PluginCommand for CompressCommand {
    type Plugin = CompressPlugin;

    fn name(&self) -> &str {
        match self {
            CompressCommand::Gzip => "to gz",
            CompressCommand::Zstd => "to zst",
            CompressCommand::Xz => "to xz",
            CompressCommand::Bzip2 => "to bz2",
        }
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .named("level", SyntaxShape::Int, "compress level.", Some('l'))
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::Any, Type::Binary)])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        match self {
            CompressCommand::Gzip => "Compress with gzip.",
            CompressCommand::Zstd => "Compress with zstd.",
            CompressCommand::Xz => "Compress with xz.",
            CompressCommand::Bzip2 => "Compress with bzip2.",
        }
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let level = match call.get_flag_value("level") {
            Some(Value::Int { val, .. }) => val as i32,
            Some(_) | None => 3,
        };

        match input {
            PipelineData::Value(value, pipeline_metadata) => {
                let data = value.coerce_binary()?.to_vec();
                let stream = self.compress(Cursor::new(data), value.span(), engine, level)?;
                Ok(PipelineData::ByteStream(stream, pipeline_metadata))
            }
            PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
                let span = byte_stream.span();
                let stream = self.compress(
                    byte_stream
                        .reader()
                        .ok_or(LabeledError::new("empty input"))?,
                    span,
                    engine,
                    level,
                )?;
                Ok(PipelineData::ByteStream(stream, pipeline_metadata))
            }
            v => {
                return Err(LabeledError::new(format!(
                    "require binary|string input, got {}",
                    v.get_type()
                ))
                .with_label("Expected binary from pipeline", call.head))
            }
        }
    }
}
