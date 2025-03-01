use crate::extract::ExtractedData;
use anyhow::Result;
use ron::ser::PrettyConfig;
use std::{fs::File, io::Write, path::Path};

pub fn save_extracted_data(extracted: ExtractedData, output_file_path: &Path) -> Result<()> {
    // Serialize to RON and save to file
    let ron_string = ron::ser::to_string_pretty(&extracted_data, PrettyConfig::default())?;

    let mut file = File::create(&output_file_path)?;
    file.write_all(ron_string.as_bytes())?;
    Ok(())
}
