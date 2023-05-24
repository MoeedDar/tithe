use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

#[derive(thiserror::Error, Debug)]
pub enum AppendLogError {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub struct AppendLog {
    data_file: File,
    index_file: File,
}

impl AppendLog {
    pub fn new(dir: PathBuf) -> Result<AppendLog, AppendLogError> {
        let data_path = dir.join("al.data");
        let index_path = dir.join("al.idx");

        let mut binding = OpenOptions::new();
        let file_options = binding
            .create(true)
            .append(true)
            .read(true);

        let data_file = file_options.open(data_path)?;
        let index_file = file_options.open(index_path)?;

        let append_log = AppendLog {
            data_file,
            index_file,
        };

        Ok(append_log)
    }

    pub fn append(&mut self, data: &[u8]) -> Result<(), AppendLogError> {
        let top = self.top()?;

        self.data_file.write(data)?;

        self.index_file
            .write(&top.to_be_bytes())
            .map_err(|err| {
                self.data_file.set_len(top).unwrap();
                err
            })?;

        Ok(())
    }

    pub fn at(&mut self, index: u64) -> Result<Vec<u8>, AppendLogError> {
        let mut start_bytes = [0u8; 8];
        let mut end_bytes = [0u8; 8];

        self.index_file.seek(SeekFrom::Start(index * 8))?;
        self.index_file.read_exact(&mut start_bytes)?;

        self.index_file
            .read_exact(&mut end_bytes)
            .or_else(|err| {
                if err.kind() == ErrorKind::UnexpectedEof {
                    let file_size = self.data_file.seek(SeekFrom::End(0))?;
                    end_bytes.copy_from_slice(&file_size.to_be_bytes())
                }
                Err(err)
            })?;

        let start = u64::from_be_bytes(start_bytes);
        let end = u64::from_be_bytes(end_bytes);        
        let size = end - start;
        let mut data = vec![0u8; size as usize];

        self.data_file.seek(SeekFrom::Start(start))?;
        self.data_file.read_exact(&mut data)?;

        Ok(data)
    }

    pub fn top(&self) -> Result<u64, AppendLogError> {
        self.data_file
            .metadata()
            .map_err(Into::into)
            .and_then(|metadata| Ok(metadata.len()))
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use super::*;

    #[test]
    fn test_append_log() {
        let temp_dir = temp_dir();

        let mut append_log = AppendLog::new(temp_dir).unwrap();

        append_log.append(b"hello").unwrap();
        append_log.append(b"world").unwrap();

        assert_eq!(append_log.at(0).unwrap(), b"hello");
        assert_eq!(append_log.at(1).unwrap(), b"world");
    }
}