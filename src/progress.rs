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

/// Responsible for displying the progress indicator. This struct will be owned by a separate
/// thread that is responsible for displaying the progress text whereas the [`IndicatorHandle`]
/// is how the outside world will interact with it.
pub struct Indicator {
    count: u64,
    stdout: io::Stdout,
    state: IndicatorState,
}

/// This struct is how the outside world will inform the [`Indicator`] about the progress of the
/// program. The `join_handle` returns the handle to the thread that owns the [`Indicator`] and the
/// `mailbox` is the [`Sender`] channel that allows [`Message`]s to be sent to [`Indicator`].
pub struct IndicatorHandle {
    pub join_handle: thread::JoinHandle<Result<(), Error>>,
    mailbox: Sender<Message>,
}

/// The different messages that could be sent to the thread that owns the [`Indicator`].
#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    /// Message that indicates that we are currently reading from disk and that a file was indexed.
    Index,

    /// Message that indicates that we are done reading from disk and are preparing the output.
    DoneIndexing,

    /// Message that indicates that the output is ready to be flushed and that we should cleanup
    /// the [`Indicator`] as well as the screen.
    RenderReady,
}

/// All of the different states the [`Indicator`] can be in during its life cycle.
#[derive(Default, PartialEq)]
enum IndicatorState {
    /// We are currently reading from disk.
    #[default]
    Indexing,

    /// No longer reading from disk; preparing output.
    Rendering,

    /// Output is prepared and the [`Indicator`] is ready to be torn down.
    Done,
}

/// Errors associated with [`crossterm`];
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] io::Error);

impl Default for Indicator {
    /// Default constructor for [`Indicator`].
    fn default() -> Self {
        Self {
            count: u64::default(),
            stdout: io::stdout(),
            state: IndicatorState::default(),
        }
    }
}

impl IndicatorHandle {
    /// The constructor for an [`IndicatorHandle`].
    pub fn new(
        join_handle: thread::JoinHandle<Result<(), Error>>,
        mailbox: Sender<Message>,
    ) -> Self {
        Self {
            join_handle,
            mailbox,
        }
    }

    /// Getter for a cloned `mailbox` wherewith to send [`Message`]s to the [`Indicator`].
    pub fn mailbox(&self) -> Sender<Message> {
        self.mailbox.clone()
    }
}

impl Indicator {
    /// Initializes a worker thread that owns [`Indicator`] that awaits on [`Message`]s to traverse
    /// through its internal states. An [`IndicatorHandle`] is returned as a mechanism to allow the
    /// outside world to send messages to the worker thread and ultimately to the [`Indicator`].
    pub fn measure() -> IndicatorHandle {
        let (tx, rx) = mpsc::channel();

        let join_handle = thread::spawn(move || {
            let mut indicator = Self::default();

            indicator.stdout.execute(cursor::SavePosition)?;
            indicator.stdout.execute(cursor::Hide)?;

            while let Ok(msg) = rx.recv() {
                if indicator.state == IndicatorState::Indexing {
                    match msg {
                        Message::Index => indicator.index()?,
                        Message::DoneIndexing => {
                            indicator.update_state(IndicatorState::Rendering)?;
                        }
                        Message::RenderReady => {}
                    }
                }

                if indicator.state == IndicatorState::Rendering && msg == Message::RenderReady {
                    indicator.update_state(IndicatorState::Done)?;
                    break;
                }
                indicator.stdout.execute(cursor::RestorePosition)?;
            }

            Ok(())
        });

        IndicatorHandle::new(join_handle, tx)
    }

    /// Updates the `state` of the [`Indicator`] to `new_state`, immediately running an associated
    /// side effect if applicable.
    #[inline]
    fn update_state(&mut self, new_state: IndicatorState) -> Result<(), Error> {
        use IndicatorState::{Done, Indexing, Rendering};

        match (&self.state, &new_state) {
            (Indexing, Rendering) => {
                let stdout = &mut self.stdout;
                stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
                stdout.execute(cursor::RestorePosition)?;
                self.rendering();
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

    /// The user-facing output when the `state` of the [`Indicator`] is `Indexing`.
    #[inline]
    fn index(&mut self) -> Result<(), Error> {
        self.count += 1;
        write!(self.stdout, "Indexing {} files...", self.count)?;
        Ok(())
    }

    /// The user-facing output when the `state` of the [`Indicator`] is `Rendering`.
    #[inline]
    fn rendering(&mut self) {
        write!(self.stdout, "Preparing output...").unwrap();
    }
}
