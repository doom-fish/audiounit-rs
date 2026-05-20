//! Executor-agnostic async streams for capture-backed `AUAudioUnit` /
//! `AUParameterTree` callback surfaces.
//!
//! Enable this module with the `async` Cargo feature:
//!
//! ```toml
//! [dependencies]
//! audiounit = { version = "0.3", features = ["async"] }
//! ```
//!
//! These wrappers intentionally target the capture-backed, event-style surfaces
//! already exposed by the synchronous API:
//!
//! - `AUAudioUnit` render-observer capture
//! - `AUAudioUnit` MIDI output capture
//! - `AUAudioUnit` MIDI event-list capture
//! - `AUParameterTree` observer / recording / automation capture
//!
//! Internally each stream owns a retained framework handle plus a lightweight
//! polling thread that drains the existing capture buffers into a bounded,
//! executor-agnostic async stream.
//!
//! # Example
//!
//! ```rust,no_run
//! use audiounit::prelude::*;
//!
//! # async fn run() -> Result<(), audiounit::AuError> {
//! let unit = AuAudioUnit::instantiate(
//!     AudioComponentDescription::apple(
//!         AUDIO_UNIT_TYPE_OUTPUT,
//!         AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT,
//!     ),
//!     InstantiationOptions::InProcess,
//! )?;
//! let stream = unit.render_observer_stream(128)?;
//!
//! while let Some(event) = stream.next().await {
//!     println!("{:?}", event?);
//! }
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::{
    AuAudioUnit, AuError, AuMidiOutputEvent, AuMidiOutputEventListSummary,
    AuParameterAutomationEventInfo, AuParameterTree, AuParameterValueEvent,
    AuRecordedParameterEventInfo, AuRenderObserverEvent,
};

/// Item type emitted by the async capture streams.
pub type AsyncCaptureResult<T> = std::result::Result<T, AuError>;

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(10);

type CaptureStream<T> = BoundedAsyncStream<AsyncCaptureResult<T>>;
type CaptureSender<T> = AsyncStreamSender<AsyncCaptureResult<T>>;

struct PollingWorker {
    stop: Arc<AtomicBool>,
    join: Option<JoinHandle<()>>,
}

impl Drop for PollingWorker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(join) = self.join.take() {
            join.thread().unpark();
            let _ = join.join();
        }
    }
}

fn validate_stream_config(
    operation: &'static str,
    capacity: usize,
    poll_interval: Duration,
) -> Result<(), AuError> {
    if capacity == 0 {
        return Err(AuError::InvalidArgument(format!(
            "{operation}: async stream capacity must be > 0"
        )));
    }
    if poll_interval.is_zero() {
        return Err(AuError::InvalidArgument(format!(
            "{operation}: poll interval must be > 0"
        )));
    }
    Ok(())
}

fn spawn_capture_stream<T, F>(capacity: usize, worker: F) -> (CaptureStream<T>, PollingWorker)
where
    T: Send + 'static,
    F: FnOnce(CaptureSender<T>, Arc<AtomicBool>) + Send + 'static,
{
    let (inner, sender) = BoundedAsyncStream::new(capacity);
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = Arc::clone(&stop);
    let join = thread::spawn(move || worker(sender, stop_thread));
    (
        inner,
        PollingWorker {
            stop,
            join: Some(join),
        },
    )
}

fn drive_capture_loop<T, F>(
    sender: &CaptureSender<T>,
    stop: &Arc<AtomicBool>,
    poll_interval: Duration,
    mut drain: F,
) where
    T: Send + 'static,
    F: FnMut() -> Result<Vec<T>, AuError>,
{
    while !stop.load(Ordering::Acquire) {
        match drain() {
            Ok(events) => {
                if events.is_empty() {
                    thread::park_timeout(poll_interval);
                } else {
                    for event in events {
                        sender.push(Ok(event));
                    }
                }
            }
            Err(error) => {
                sender.push(Err(error));
                break;
            }
        }
    }
}

macro_rules! impl_capture_stream_methods {
    ($stream:ident, $item:ty) => {
        impl $stream {
            pub const fn next(&self) -> NextItem<'_, AsyncCaptureResult<$item>> {
                self.inner.next()
            }

            pub fn try_next(&self) -> Option<AsyncCaptureResult<$item>> {
                self.inner.try_next()
            }

            pub fn buffered_count(&self) -> usize {
                self.inner.buffered_count()
            }

            pub fn is_closed(&self) -> bool {
                self.inner.is_closed()
            }
        }
    };
}

/// Async stream over `AUAudioUnit` render-observer capture events.
pub struct AuRenderObserverStream {
    inner: CaptureStream<AuRenderObserverEvent>,
    _worker: PollingWorker,
}

impl AuRenderObserverStream {
    pub fn subscribe(unit: &AuAudioUnit, capacity: usize) -> Result<Self, AuError> {
        Self::subscribe_with_poll_interval(unit, capacity, DEFAULT_POLL_INTERVAL)
    }

    pub fn subscribe_with_poll_interval(
        unit: &AuAudioUnit,
        capacity: usize,
        poll_interval: Duration,
    ) -> Result<Self, AuError> {
        validate_stream_config(
            "AuRenderObserverStream::subscribe_with_poll_interval",
            capacity,
            poll_interval,
        )?;

        let unit = unit.retained()?;
        let token = unit.add_render_observer_capture()?;
        let (inner, worker) = spawn_capture_stream(capacity, move |sender, stop| {
            drive_capture_loop(&sender, &stop, poll_interval, || {
                unit.take_render_observer_events(token)
            });
            unit.remove_render_observer(token);
        });

        Ok(Self {
            inner,
            _worker: worker,
        })
    }
}

impl_capture_stream_methods!(AuRenderObserverStream, AuRenderObserverEvent);

/// Async stream over captured `MIDIOutputEventBlock` callbacks.
pub struct AuMidiOutputEventStream {
    inner: CaptureStream<AuMidiOutputEvent>,
    _worker: PollingWorker,
}

impl AuMidiOutputEventStream {
    pub fn subscribe(unit: &AuAudioUnit, capacity: usize) -> Result<Self, AuError> {
        Self::subscribe_with_poll_interval(unit, capacity, DEFAULT_POLL_INTERVAL)
    }

    pub fn subscribe_with_poll_interval(
        unit: &AuAudioUnit,
        capacity: usize,
        poll_interval: Duration,
    ) -> Result<Self, AuError> {
        validate_stream_config(
            "AuMidiOutputEventStream::subscribe_with_poll_interval",
            capacity,
            poll_interval,
        )?;

        let unit = unit.retained()?;
        unit.set_midi_output_event_capture_enabled(true);
        let (inner, worker) = spawn_capture_stream(capacity, move |sender, stop| {
            drive_capture_loop(&sender, &stop, poll_interval, || {
                unit.take_captured_midi_output_events()
            });
            unit.set_midi_output_event_capture_enabled(false);
        });

        Ok(Self {
            inner,
            _worker: worker,
        })
    }
}

impl_capture_stream_methods!(AuMidiOutputEventStream, AuMidiOutputEvent);

/// Async stream over captured `MIDIOutputEventListBlock` summaries.
pub struct AuMidiOutputEventListStream {
    inner: CaptureStream<AuMidiOutputEventListSummary>,
    _worker: PollingWorker,
}

impl AuMidiOutputEventListStream {
    pub fn subscribe(unit: &AuAudioUnit, capacity: usize) -> Result<Self, AuError> {
        Self::subscribe_with_poll_interval(unit, capacity, DEFAULT_POLL_INTERVAL)
    }

    pub fn subscribe_with_poll_interval(
        unit: &AuAudioUnit,
        capacity: usize,
        poll_interval: Duration,
    ) -> Result<Self, AuError> {
        validate_stream_config(
            "AuMidiOutputEventListStream::subscribe_with_poll_interval",
            capacity,
            poll_interval,
        )?;

        let unit = unit.retained()?;
        unit.set_midi_output_event_list_capture_enabled(true);
        let (inner, worker) = spawn_capture_stream(capacity, move |sender, stop| {
            drive_capture_loop(&sender, &stop, poll_interval, || {
                unit.take_captured_midi_output_event_lists()
            });
            unit.set_midi_output_event_list_capture_enabled(false);
        });

        Ok(Self {
            inner,
            _worker: worker,
        })
    }
}

impl_capture_stream_methods!(AuMidiOutputEventListStream, AuMidiOutputEventListSummary);

/// Async stream over `AUParameterTree` value-observer capture events.
pub struct AuParameterObserverStream {
    inner: CaptureStream<AuParameterValueEvent>,
    _worker: PollingWorker,
}

impl AuParameterObserverStream {
    pub fn subscribe(tree: &AuParameterTree, capacity: usize) -> Result<Self, AuError> {
        Self::subscribe_with_poll_interval(tree, capacity, DEFAULT_POLL_INTERVAL)
    }

    pub fn subscribe_with_poll_interval(
        tree: &AuParameterTree,
        capacity: usize,
        poll_interval: Duration,
    ) -> Result<Self, AuError> {
        validate_stream_config(
            "AuParameterObserverStream::subscribe_with_poll_interval",
            capacity,
            poll_interval,
        )?;

        let tree = tree.retained()?;
        let token = tree.add_parameter_observer_capture()?;
        let (inner, worker) = spawn_capture_stream(capacity, move |sender, stop| {
            drive_capture_loop(&sender, &stop, poll_interval, || {
                tree.take_parameter_observer_events(token)
            });
            tree.remove_parameter_observer(token);
        });

        Ok(Self {
            inner,
            _worker: worker,
        })
    }
}

impl_capture_stream_methods!(AuParameterObserverStream, AuParameterValueEvent);

/// Async stream over `AUParameterTree` recording-observer capture events.
pub struct AuParameterRecordingStream {
    inner: CaptureStream<AuRecordedParameterEventInfo>,
    _worker: PollingWorker,
}

impl AuParameterRecordingStream {
    pub fn subscribe(tree: &AuParameterTree, capacity: usize) -> Result<Self, AuError> {
        Self::subscribe_with_poll_interval(tree, capacity, DEFAULT_POLL_INTERVAL)
    }

    pub fn subscribe_with_poll_interval(
        tree: &AuParameterTree,
        capacity: usize,
        poll_interval: Duration,
    ) -> Result<Self, AuError> {
        validate_stream_config(
            "AuParameterRecordingStream::subscribe_with_poll_interval",
            capacity,
            poll_interval,
        )?;

        let tree = tree.retained()?;
        let token = tree.add_parameter_recording_observer_capture()?;
        let (inner, worker) = spawn_capture_stream(capacity, move |sender, stop| {
            drive_capture_loop(&sender, &stop, poll_interval, || {
                tree.take_parameter_recording_events(token)
            });
            tree.remove_parameter_observer(token);
        });

        Ok(Self {
            inner,
            _worker: worker,
        })
    }
}

impl_capture_stream_methods!(AuParameterRecordingStream, AuRecordedParameterEventInfo);

/// Async stream over `AUParameterTree` automation-observer capture events.
pub struct AuParameterAutomationStream {
    inner: CaptureStream<AuParameterAutomationEventInfo>,
    _worker: PollingWorker,
}

impl AuParameterAutomationStream {
    pub fn subscribe(tree: &AuParameterTree, capacity: usize) -> Result<Self, AuError> {
        Self::subscribe_with_poll_interval(tree, capacity, DEFAULT_POLL_INTERVAL)
    }

    pub fn subscribe_with_poll_interval(
        tree: &AuParameterTree,
        capacity: usize,
        poll_interval: Duration,
    ) -> Result<Self, AuError> {
        validate_stream_config(
            "AuParameterAutomationStream::subscribe_with_poll_interval",
            capacity,
            poll_interval,
        )?;

        let tree = tree.retained()?;
        let token = tree.add_parameter_automation_observer_capture()?;
        let (inner, worker) = spawn_capture_stream(capacity, move |sender, stop| {
            drive_capture_loop(&sender, &stop, poll_interval, || {
                tree.take_parameter_automation_events(token)
            });
            tree.remove_parameter_observer(token);
        });

        Ok(Self {
            inner,
            _worker: worker,
        })
    }
}

impl_capture_stream_methods!(AuParameterAutomationStream, AuParameterAutomationEventInfo);
