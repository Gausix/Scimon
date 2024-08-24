use walkdir::WalkDir;

use std::{
    fs::File,
    path::Path,

    io::{
        Read,
        Write,
        Result as IoResult,
    },
};

use zip::{
    CompressionMethod,

    write::{
        FileOptions,
        ExtendedFileOptions
    },
};

use crate::{
    syntax::vars::Vars,
    configs::settings::Settings,

    ui::{
        ui_base::UI,
        panic_alerts::PanicAlerts,
        compress_alerts::CompressAlerts,
    },
};

pub struct Compress {
    contents: String,
}

impl Compress {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    fn compress_level() -> u8 {
        let value = Settings::get("general.level_compress", "INT");
        let level = value.as_i64().expect("Invalid level_compress value. Must be an integer.") as u8;

        if level > 9 {
            PanicAlerts::compress_level();
            return 0
        }
        
        level
    }
    
    pub fn get(&self) -> IoResult<()> {
        if let Some(zip_file) = Vars::get_compress(&self.contents) {
            UI::section_header("Compressing files", "normal");

            let folder_path = Vars::get_path(&self.contents);
            let compress_level = Self::compress_level();
            
            let output_path = Path::new(&zip_file);
            let output_file = File::create(output_path)?;
            let mut zip = zip::ZipWriter::new(output_file);
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .compression_level(Some(compress_level.into())) // Max compression level
                .unix_permissions(0o755);

            for entry in WalkDir::new(&folder_path) {
                let entry = entry?;
                let path = entry.path();
                let name = path.strip_prefix(Path::new(&folder_path)).unwrap();

                if path.extension().map_or(false, |ext| ext == "pdf") {
                    zip.start_file(name.to_str().unwrap(), options.clone())?;
                    let mut f = File::open(path)?;
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;

                    let file = path.to_str().unwrap();
                    CompressAlerts::added(&file, &zip_file);
                }
            }

            zip.finish()?;
            CompressAlerts::completed(&zip_file);
        }

        Ok(())
    }

}
