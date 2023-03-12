use std::fs;

use crate::error::MetalError;

pub fn discover_source_files(dir: &str) -> Result<Vec<String>, MetalError> {
    fn read_dir(path: &str) -> Result<Vec<String>, MetalError> {
        let Ok(entries) = fs::read_dir(path) else {
            return Err(MetalError::FailedToOpenDir(path.to_owned()))
        };

        let mut result = vec![];

        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };

            let path = entry.path();

            if path.is_dir() {
                let Ok(mut sub_result) = read_dir(path.to_str().unwrap()) else {
                    continue;
                };

                result.append(&mut sub_result);
            } else {
                result.push(path.to_str().unwrap().to_owned());
            }
        }

        Ok(result)
    }

    let result = read_dir(dir)?;
    Ok(result)
}

pub fn read_source_file(path: &str) -> Result<String, MetalError> {
    let Ok(content) = fs::read_to_string(path) else {
        return Err(MetalError::FailedToLoadFile(path.to_owned()))
    };

    Ok(content)
}

pub fn write_output(path: &str, content: &str) -> Result<(), MetalError> {
    let Ok(_) = fs::write(path, content) else {
        return Err(MetalError::FailedToWriteFile(path.to_owned()))
    };

    Ok(())
}