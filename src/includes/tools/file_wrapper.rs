
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufWriter};
use std::path::PathBuf;
use jni::JNIEnv;
use jni::objects::JObject;
use log::debug;

pub struct FileWrapper {
    path: PathBuf,
}

impl FileWrapper {
    pub fn new(env: &mut JNIEnv, context: JObject, relative_path: &str) -> Result<Self, String> {
        
        let files_dir_result = env.call_method(
            context,
            "getFilesDir",
            "()Ljava/io/File;",
            &[]
        ).map_err(|e| format!("Failed to call getFilesDir: {}", e))?;
        
        let file_object = files_dir_result.l()
            .map_err(|e| format!("Failed to convert to object: {}", e))?;
        
        let path_result = env.call_method(
            file_object,
            "getAbsolutePath",
            "()Ljava/lang/String;",
            &[]
        ).map_err(|e| format!("Failed to call getAbsolutePath: {}", e))?;
        
        let jpath = path_result.l()
            .map_err(|e| format!("Failed to convert to object: {}", e))?;
        
        let base_path: String = env.get_string((&jpath).into())
            .map_err(|e| format!("Failed to get string: {}", e))?
            .into();
        
        let mut full_path = PathBuf::from(base_path);
        full_path.push(relative_path);
        
        Ok(FileWrapper {
            path: full_path,
        })
    }

    pub fn file_exists(&self) -> bool {
        self.path.exists()
    }

    pub fn read_content(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn read_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(&self.path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    pub fn write_text(&self, content: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.path)?;
        file.write_all(content.as_bytes())?;
        debug!("FILE WROTE");
        Ok(())
    }

    pub fn write_text_force(&self, content: &str, force: bool) -> Result<usize, std::io::Error> {
        let file = if force {
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.path)?
        } else {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.path)?
        };

        let mut writer = BufWriter::new(file);
        let bytes = content.as_bytes();
        writer.write_all(bytes)?;
        writer.flush()?;
        
        debug!("Bytes Written->{}", bytes.len());
        Ok(bytes.len())
    }

    pub fn get_path(&self) -> &std::path::Path {
        &self.path
    }
}