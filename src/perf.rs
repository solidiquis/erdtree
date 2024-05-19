use ahash::HashMap;
use once_cell::sync::Lazy;
use std::{
    fmt, io,
    sync::{
        mpsc::{self, Sender},
        OnceLock,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

static mut GLOBAL_PERF_METRICS: OnceLock<PerfMetrics> = OnceLock::new();
static OUTPUT_NOTIFIER: OnceLock<Sender<Message>> = OnceLock::new();
static mut ONGOING_RECORDINGS: Lazy<HashMap<String, Recording>> = Lazy::new(HashMap::default);

#[derive(Debug)]
pub struct Recording {
    name: String,
    start: Instant,
    duration: Option<Duration>,
}

#[derive(Debug)]
pub struct PerfMetrics(Option<JoinHandle<Vec<Recording>>>);

enum Message {
    Recording(Recording),
    Terminate,
}

pub fn init_global() {
    let (tx, rx) = mpsc::channel();

    OUTPUT_NOTIFIER.set(tx).unwrap();

    let handle = thread::spawn(move || {
        let mut recordings = Vec::new();

        while let Ok(Message::Recording(recording)) = rx.recv() {
            recordings.push(recording);
        }

        recordings
    });

    unsafe {
        GLOBAL_PERF_METRICS.set(PerfMetrics(Some(handle))).unwrap();
    }
}

pub fn begin_recording(name: &str) {
    let recording = Recording {
        name: name.to_string(),
        start: Instant::now(),
        duration: None,
    };

    unsafe {
        if let Some(old_value) = ONGOING_RECORDINGS.insert(name.to_string(), recording) {
            panic!("Can't have two ongoing recordings with the same name: {old_value}");
        }
    }
}

pub fn finish_recording(name: &str) {
    let mut recording = unsafe {
        ONGOING_RECORDINGS
            .remove(name)
            .unwrap_or_else(|| panic!("No ongoing recording with name '{name}'"))
    };

    recording.duration = Some(recording.start.elapsed());

    OUTPUT_NOTIFIER
        .get()
        .cloned()
        .and_then(|tx| tx.send(Message::Recording(recording)).ok());
}

pub fn output<T: io::Write>(mut w: T) {
    OUTPUT_NOTIFIER
        .get()
        .cloned()
        .and_then(|tx| tx.send(Message::Terminate).ok());

    let mut recordings = unsafe {
        GLOBAL_PERF_METRICS
            .get_mut()
            .and_then(|PerfMetrics(join)| join.take().and_then(|h| h.join().ok()))
            .unwrap()
    };

    recordings.sort_by_cached_key(|a| a.duration.unwrap().as_nanos());

    let out = recordings
        .into_iter()
        .rev()
        .map(|r| format!("{r}"))
        .collect::<Vec<_>>()
        .join("\n");

    writeln!(w, "Performance metrics:\n{out}").unwrap();

    unsafe {
        if !ONGOING_RECORDINGS.is_empty() {
            let still_ongoing = ONGOING_RECORDINGS
                .keys()
                .map(|k| format!("- {k}"))
                .collect::<Vec<String>>()
                .join("\n");
            writeln!(w, "The following recordings are finished:\n{still_ongoing}").unwrap();
        }
    }
}

impl fmt::Display for Recording {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  {}:\n\t{}ms | {}ns",
            self.name,
            self.duration.unwrap().as_millis(),
            self.duration.unwrap().as_nanos(),
        )
    }
}
