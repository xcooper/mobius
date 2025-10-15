use anyhow::{anyhow, Result};
use rig::message::Message;
use std::env::temp_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub trait Thread {
    fn push_message(&self, message: Message) -> Result<()>;
    fn get_messages(&self) -> Result<Vec<Message>>;
}

pub fn allocate_thread(thread_id: String) -> Result<Box<dyn Thread>> {
    let mut thread_file = temp_dir();
    thread_file.push(&thread_id);
    let new_thread = FileBasedThread::new(thread_id, thread_file)?;
    Ok(Box::from(new_thread))
}

pub fn destroy_thread(thread: Box<dyn Thread>) {
    todo!()
}

struct FileBasedThread {
    thread_id: String,
    thread_file: PathBuf,
}

impl FileBasedThread {
    fn new(thread_id: String, file_path: PathBuf) -> Result<FileBasedThread> {
        let thread = FileBasedThread {
            thread_id,
            thread_file: file_path,
        };
        Ok(thread)
    }

    fn serialize_message(&self, message: &Message) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(message)?)
    }

    fn deserialize_message(&self, raw_data: &Vec<u8>) -> Result<Message> {
        Ok(serde_json::from_slice(raw_data)?)
    }

    fn read_thread_file(&self) -> Result<BufReader<File>> {
        let file = File::open(self.thread_file.as_path())?;
        Ok(BufReader::new(file))
    }

    fn write_thread_file(&self) -> Result<BufWriter<File>> {
        let file = File::options()
            .create(true)
            .append(true)
            .open(self.thread_file.as_path())?;
        Ok(BufWriter::new(file))
    }
}

impl Thread for FileBasedThread {
    fn push_message(&self, message: Message) -> Result<()> {
        let raw_data = self.serialize_message(&message);
        let mut file = self.write_thread_file()?;
        file.write_all(&raw_data?)?;
        file.write_all(b"\n")?;
        file.flush()?;
        Ok(())
    }

    fn get_messages(&self) -> Result<Vec<Message>> {
        let mut messages = Vec::new();
        let buf_rdr = self.read_thread_file()?;
        for line in buf_rdr.lines() {
            let Ok(line_str) = line else {
                return Err(anyhow!("can not read data from {:?}", self.thread_file));
            };
            let line_bytes = line_str.into_bytes();
            let Ok(message) = self.deserialize_message(&line_bytes) else {
                return Err(anyhow!(
                    "can not deserialize data from {:?}: {:?}...",
                    self.thread_file,
                    line_bytes.get(..30).unwrap_or(&line_bytes)
                ));
            };
            messages.push(message);
        }
        Ok(messages)
    }
}
