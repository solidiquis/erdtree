use crossterm::{
    cursor,
    ExecutableCommand,
    terminal::{self, ClearType},
};
use std::{
    io::{self, Write},
    fmt,
    sync::{
        mpsc::{self, Receiver, Sender},
        OnceLock,
    },
    thread,
};

/// For progress indicator to throttle printin.
pub const RENDER_INTERVAL_MS: u64 = 33;

/// To notify the progress indicator
static NOTIFIER: OnceLock<Sender<Message>> = OnceLock::new();

pub struct Indicator {
    counter: usize,
    state: IndicatorState,
    mailbox: Receiver<Message>,
}

pub enum Message {
    Indexing(usize),
    DoneIndexing,
}

#[derive(Default)]
enum IndicatorState {
    #[default]
    Indexing,
    PreparingOutput,
}

#[derive(Default)]
pub struct FileCounter {
    num_file: usize,
    num_dir: usize,
    num_link: usize,
}

impl Indicator {
    pub fn init() -> Self {
        let (tx, rx) = mpsc::channel();

        NOTIFIER.set(tx).unwrap();

        Self {
            counter: 0,
            state: IndicatorState::default(),
            mailbox: rx,
        }
    }

    pub fn use_notifier() -> Option<Sender<Message>> {
        NOTIFIER.get().cloned()
    }

    fn update_state(&mut self, new_state: IndicatorState) {
        self.state = new_state;
    }

    pub fn notify(notifier: &Option<Sender<Message>>, count: usize) {
        if let Some(n) = notifier.as_ref() {
            let _ = n.send(Message::Indexing(count));
        }
    }

    pub fn finish(notifier: &Option<Sender<Message>>) {
        if let Some(n) = notifier.as_ref() {
            let _ = n.send(Message::DoneIndexing);
        }
    }

    pub fn show_progress<F, T>(mut self, op: F) -> T
    where F: FnOnce() -> T
    {
        let _ = io::stdout().execute(cursor::Hide);

        let comp_result = thread::scope(move |s| {
            let handle = s.spawn(move || {
                let threshold = std::time::Duration::from_millis(RENDER_INTERVAL_MS);
                let mut time_last_print = std::time::Instant::now();
                let mut stdout = io::stdout();
                let _ = stdout.execute(cursor::SavePosition);
                
                while let Ok(Message::Indexing(count)) = self.mailbox.recv() {
                    self.counter = count;

                    if time_last_print.elapsed() < threshold {
                        continue;
                    }
                    let _ = stdout.execute(terminal::Clear(ClearType::CurrentLine));
                    let _ = write!(stdout, "{self}");
                    let _ = stdout.execute(cursor::RestorePosition);
                    time_last_print = std::time::Instant::now();
                }
                self.update_state(IndicatorState::PreparingOutput);
                let _ = stdout.execute(terminal::Clear(ClearType::CurrentLine));
                let _ = write!(stdout, "{self}");
            });

            let out = op();
            handle.join().unwrap();

            out
        });

        let _ = io::stdout().execute(cursor::Show);

        comp_result
    }
}

impl fmt::Display for Indicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            IndicatorState::Indexing => {
                write!(f, "Indexing {} files...", self.counter)
            }
            IndicatorState::PreparingOutput => write!(f, "Preparing output..."),
        }
    }
}
