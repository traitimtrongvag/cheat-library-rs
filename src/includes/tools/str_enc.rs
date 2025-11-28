pub struct StrEnc {
    data: Vec<u8>,
}

impl StrEnc {
    pub fn new(str_data: &[u8], key: &[u8]) -> Self {
        let len = str_data.len().min(key.len());
        let mut data = Vec::with_capacity(len);
        
        for i in 0..len {
            data.push(str_data[i] ^ key[i]);
        }
        
        StrEnc { data }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.data)
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

impl Drop for StrEnc {
    fn drop(&mut self) {
        for byte in &mut self.data {
            *byte = 0;
        }
    }
}

#[macro_export]
macro_rules! obfuscate {
    ($s:expr) => {{
        const KEY: &[u8] = b"simple_key_123";
        $crate::includes::tools::str_enc::StrEnc::new($s.as_bytes(), KEY)
    }};
}