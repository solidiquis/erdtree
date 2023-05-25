use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write},
    sync::mpsc::{self, Sender},
    thread,
};

pub struct Indicator {
    count: u64,
    stdout: io::Stdout,
    state: IndicatorState,
}

pub struct IndicatorHandle {
    pub join_handle: thread::JoinHandle<Result<(), Error>>,
    mailbox: Sender<Message>,
}

#[derive(Debug)]
pub enum Message {
    Index,
    DoneIndexing,
    RenderReady,
}

#[derive(Default)]
enum IndicatorState {
    #[default]
    Indexing,
    Rendering,
    Done,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Crossterm(#[from] io::Error),
}

impl Default for Indicator {
    fn default() -> Self {
        Self {
            count: u64::default(),
            stdout: io::stdout(),
            state: IndicatorState::default(),
        }
    }
}

impl IndicatorHandle {
    pub fn new(
        join_handle: thread::JoinHandle<Result<(), Error>>,
        mailbox: Sender<Message>,
    ) -> Self {
        Self {
            join_handle,
            mailbox,
        }
    }

    pub fn mailbox(&self) -> Sender<Message> {
        self.mailbox.clone()
    }
}

impl Indicator {
    pub fn measure() -> IndicatorHandle {
        let (tx, rx) = mpsc::channel::<Message>();

        let join_handle = thread::spawn(move || -> Result<(), Error> {
            let mut indicator = Self::default();

            indicator.stdout.execute(cursor::SavePosition)?;
            indicator.stdout.execute(cursor::Hide)?;

            while let Ok(msg) = rx.recv() {
                if matches!(indicator.state, IndicatorState::Indexing) {
                    match msg {
                        Message::Index => indicator.index()?,
                        Message::DoneIndexing => {
                            indicator.update_state(IndicatorState::Rendering)?
                        }
                        _ => (),
                    }
                }

                if matches!(indicator.state, IndicatorState::Rendering) {
                    if matches!(msg, Message::RenderReady) {
                        indicator.update_state(IndicatorState::Done)?;
                        break;
                    }
                }
                indicator.stdout.execute(cursor::RestorePosition)?;
            }

            Ok(())
        });

        IndicatorHandle::new(join_handle, tx)
    }

    #[inline]
    fn update_state(&mut self, new_state: IndicatorState) -> Result<(), Error> {
        use IndicatorState::{Done, Indexing, Rendering};

        match (&self.state, &new_state) {
            (Indexing, Rendering) => {
                let stdout = &mut self.stdout;
                stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
                stdout.execute(cursor::RestorePosition)?;
                self.rendering()
            }

            (Rendering, Done) => {
                let stdout = &mut self.stdout;
                stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
                stdout.execute(cursor::RestorePosition)?;
                stdout.execute(cursor::Show)?;
            }
            _ => (),
        }

        self.state = new_state;

        Ok(())
    }

    #[inline]
    fn index(&mut self) -> Result<(), Error> {
        self.count += 1;
        write!(self.stdout, "Indexing {} files...", self.count)?;
        Ok(())
    }

    #[inline]
    fn rendering(&mut self) {
        write!(self.stdout, "Preparing output...").unwrap();
    }
}
