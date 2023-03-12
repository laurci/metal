mod fs;
mod config;
mod error;
mod processor;

pub use error::{MetalError};
pub use config::{MetalConfig};

use processor::{should_read_file, should_process_file, Processor};

pub fn process_teleports(dir: &str, config: MetalConfig) -> Result<(), MetalError> {
    let source_file_paths = fs::discover_source_files(dir)?;

    let mut processor = Processor::new(config);

    for source_file_path in source_file_paths {
        if !should_read_file(&source_file_path) {
            continue;
        }

        let source_file_content = fs::read_source_file(&source_file_path)?;

        if !should_process_file(&source_file_path, &source_file_content) {
            continue;
        }

        processor.process(&source_file_path, &source_file_content)?;
    }

    processor.write_output()?;

    Ok(())
}