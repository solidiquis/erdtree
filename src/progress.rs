use crate::{context::Context, tty};
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write},
    sync::mpsc::{self, SendError, SyncSender},
    thread::{self, JoinHandle},
};

/// Responsible for displying the progress indicator. This struct will be owned by a separate
/// thread that is responsible for displaying the progress text whereas the [`IndicatorHandle`]
/// is how the outside world will interact with it.
pub struct Indicator<'a> {
    count: u64,
    stdout: io::StdoutLock<'a>,
    state: IndicatorState,
}

/// This struct is how the outside world will inform the [`Indicator`] about the progress of the
/// program. The `join_handle` returns the handle to the thread that owns the [`Indicator`] and the
/// `mailbox` is the [`SyncSender`] channel that allows [`Message`]s to be sent to [`Indicator`].
pub struct IndicatorHandle {
    pub join_handle: Option<JoinHandle<Result<(), Error>>>,
    mailbox: SyncSender<Message>,
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

    /// Tear down the progress indicator for whatever reason.
    Finish,
}

/// All of the different states the [`Indicator`] can be in during its life cycle.
#[derive(Debug, Default, PartialEq)]
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
pub enum Error {
    #[error("#{0}")]
    Io(#[from] io::Error),

    #[error("#{0}")]
    Send(#[from] SendError<Message>),
}

impl Default for Indicator<'_> {
    /// Default constructor for [`Indicator`].
    fn default() -> Self {
        Self {
            count: u64::default(),
            stdout: io::stdout().lock(),
            state: IndicatorState::default(),
        }
    }
}

impl IndicatorHandle {
    /// The constructor for an [`IndicatorHandle`].
    pub fn new(
        join_handle: Option<JoinHandle<Result<(), Error>>>,
        mailbox: SyncSender<Message>,
    ) -> Self {
        Self {
            join_handle,
            mailbox,
        }
    }

    /// Getter for a cloned `mailbox` wherewith to send [`Message`]s to the [`Indicator`].
    pub fn mailbox(&self) -> SyncSender<Message> {
        self.mailbox.clone()
    }

    /// Analogous to [`Self::try_terminate`] but panics if failure.
    pub fn terminate(this: Option<Self>) {
        Self::try_terminate(this).expect("Failed to properly terminate the progress indicator");
    }

    /// Attempts to terminate the [`Indicator`] with cleanup.
    pub fn try_terminate(this: Option<Self>) -> Result<(), Error> {
        if let Some(mut handle) = this {
            // This is allowed to fail silently. If user administers interrupt then the `Indicator`
            // will be dropped along with the receiving end of the `mailbox`.
            //
            // If user does not administer interrupt but file-system traversal fails for whatever
            // reason then this will proceed as normal.
            let _ = handle.mailbox().send(Message::Finish);

            handle
                .join_handle
                .take()
                .map(|h| h.join().unwrap())
                .transpose()?;
        }

        Ok(())
    }
}

impl<'a> Indicator<'a> {
    /// Initializes an [`Indicator`] returning an atomic reference counter of an [`IndicatorHandle`] if
    /// a progress indicator is enabled via [`Context`]. Upon initialization an interrupt handler is
    /// also registered. Sources of panic can come from [`IndicatorHandle::terminate`] or
    /// [`ctrlc::set_handler`].
    pub fn maybe_init(ctx: &Context) -> Option<IndicatorHandle> {
        (ctx.stdout_is_tty && !ctx.no_progress)
            .then(Indicator::measure)
            .map(|indicator| {
                let mailbox = indicator.mailbox();

                let int_handler = move || {
                    let _ = mailbox.try_send(Message::Finish);
                    tty::restore_tty();
                };

                ctrlc::set_handler(int_handler).expect("Failed to set interrupt handler");

                indicator
            })
    }

    /// Initializes a worker thread that owns [`Indicator`] that awaits on [`Message`]s to traverse
    /// through its internal states. An [`IndicatorHandle`] is returned as a mechanism to allow the
    /// outside world to send messages to the worker thread and ultimately to the [`Indicator`].
    pub fn measure() -> IndicatorHandle {
        let (tx, rx) = mpsc::sync_channel(1024);

        let join_handle = thread::spawn(move || {
            let mut indicator = Self::default();

            indicator.stdout.execute(cursor::SavePosition)?;
            indicator.stdout.execute(cursor::Hide)?;

            while let Ok(msg) = rx.recv() {
                match msg {
                    Message::Index => indicator.index()?,
                    Message::DoneIndexing => {
                        indicator.update_state(IndicatorState::Rendering)?;
                    },
                    Message::RenderReady | Message::Finish => {
                        indicator.update_state(IndicatorState::Done)?;
                        return Ok(());
                    },
                }

                indicator.stdout.execute(cursor::RestorePosition)?;
            }

            Ok(())
        });

        IndicatorHandle::new(Some(join_handle), tx)
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
            },

            (Rendering | Indexing, Done) => {
                let stdout = &mut self.stdout;
                stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
                stdout.execute(cursor::RestorePosition)?;
                stdout.execute(cursor::Show)?;
            },
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
