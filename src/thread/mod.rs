use anyhow::{anyhow, Result};
use log::debug;
use rig::message::Message;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

static GLOBAL_LOCK: LazyLock<Mutex<HashMap<String, Arc<Mutex<()>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait Thread {
    fn push_message(&self, message: &Message) -> Result<()>;
    fn get_messages(&self) -> Result<Vec<Message>>;
}

pub fn allocate_thread(thread_id: &str) -> Result<Box<dyn Thread>> {
    let mut thread_file = temp_dir();
    thread_file.push(thread_id);
    debug!("allocate file for thread {}: {:?}", thread_id, thread_file);
    let Ok(mut global_lock) = GLOBAL_LOCK.lock() else {
        return Err(anyhow!("can not get lock of GLOBAL_LOCK"));
    };
    if !global_lock.contains_key(thread_id) {
        global_lock.insert(thread_id.to_string(), Arc::new(Mutex::new(())));
    }
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
    fn new(thread_id: &str, file_path: PathBuf) -> Result<FileBasedThread> {
        let thread = FileBasedThread {
            thread_id: thread_id.to_string(),
            thread_file: file_path,
        };
        Ok(thread)
    }

    fn serialize_message(&self, message: &Message) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(message)?)
    }

    fn deserialize_message(&self, raw_data: &[u8]) -> Result<Message> {
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
    fn push_message(&self, message: &Message) -> Result<()> {
        let file_lock = {
            let global_lock = GLOBAL_LOCK
                .lock()
                .map_err(|_| anyhow!("can not get lock of GLOBAL_LOCK"))?;
            global_lock
                .get(&self.thread_id)
                .cloned()
                .expect("no file lock for current thread")
        };
        let _guard = file_lock
            .lock()
            .map_err(|_| anyhow!("can not get lock for thread file"))?;
        let raw_data = self.serialize_message(message)?;
        let mut file = self.write_thread_file()?;
        file.write_all(&raw_data)?;
        file.write_all(b"\n")?;
        file.flush()?;
        Ok(())
    }

    fn get_messages(&self) -> Result<Vec<Message>> {
        let file_lock = {
            let global_lock = GLOBAL_LOCK
                .lock()
                .map_err(|_| anyhow!("can not get lock of GLOBAL_LOCK"))?;
            global_lock
                .get(&self.thread_id)
                .cloned()
                .expect("no file lock for current thread")
        };
        let _guard = file_lock
            .lock()
            .map_err(|_| anyhow!("can not get lock for thread file"))?;
        let mut messages = Vec::new();
        let Ok(buf_rdr) = self.read_thread_file() else {
            return Ok(Vec::new());
        };
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

#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    fn setup_logger() {
        stderrlog::new().verbosity(3).init().unwrap_or_default();
    }

    #[test]
    fn test_write_then_read_messages() {
        setup_logger();
        let thread_id = format!("{}", rand::random::<u64>());
        let thread = allocate_thread(&thread_id).unwrap();
        let message = Message::user("test");
        thread
            .push_message(&message)
            .expect("save 1st message failed");
        let expect_one_message = thread.get_messages().expect("unable to get 1st message");
        assert!(
            expect_one_message.len() == 1,
            "the len of message is not 1 after saved first message"
        );
        thread
            .push_message(&message)
            .expect("save 2nd message failed");
        let expect_two_messages = thread.get_messages().expect("unable to get 2 messages");
        assert!(
            expect_two_messages.len() == 2,
            "the len of message is not 2 after saved two messages"
        );
    }

    #[test]
    fn test_read_empty_file() {
        setup_logger();
        let thread_id = format!("{}", rand::random::<u64>());
        let thread = allocate_thread(&thread_id).unwrap();
        let messages = thread
            .get_messages()
            .expect("should not have error from an empty thread file");
        assert!(messages.is_empty(), "no messages should be in the list");
    }

    #[test]
    fn test_concurrent_writes() {
        setup_logger();
        const NUM_THREADS: usize = 1000;
        let thread_id_rc = Arc::new(format!("{}", rand::random::<u64>()));

        let mut handles = Vec::new();

        for i in 0..NUM_THREADS {
            let thread_id = thread_id_rc.clone();
            let handle = thread::spawn(move || -> Result<()> {
                let thread = allocate_thread(&thread_id).unwrap();
                let message = Message::user(format!("test in thread-{}", i));
                thread.push_message(&message)?;
                Ok(())
            });
            handles.push(handle);
        }

        for handle in handles {
            let res = handle.join().expect("thread panicked");
            res.expect("thread returned an error");
        }

        let thread = allocate_thread(&thread_id_rc).unwrap();
        let messages = thread.get_messages().expect("can not get all messages.");
        assert!(NUM_THREADS == messages.len());
    }
}
