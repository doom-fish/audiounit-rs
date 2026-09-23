use core::ffi::c_void;
use core::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicUsize, Ordering};

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::spsc::{SpscConsumer, SpscProducer, SpscRing};

use crate::au_audio_unit::{
    AuMidiOutputEvent, AuMidiOutputEventListSummary, AuRenderObserverEvent,
};
use crate::au_parameter_tree::{
    AuParameterAutomationEventInfo, AuParameterValueEvent, AuRecordedParameterEventInfo,
};
use crate::error::AuError;

pub const CAPTURE_CAPACITY_MAX: usize = 65_536;
pub const MIDI_OUTPUT_INLINE_BYTES: usize = 128;

pub(crate) struct CaptureSink<T> {
    producer: AtomicPtr<SpscProducer<T, CAPTURE_CAPACITY_MAX>>,
    in_flight: AtomicUsize,
    dropped: AtomicU64,
    _producer: PhantomData<SpscProducer<T, CAPTURE_CAPACITY_MAX>>,
}

impl<T> CaptureSink<T> {
    fn new(producer: SpscProducer<T, CAPTURE_CAPACITY_MAX>) -> Self {
        Self {
            producer: AtomicPtr::new(Box::into_raw(Box::new(producer))),
            in_flight: AtomicUsize::new(0),
            dropped: AtomicU64::new(0),
            _producer: PhantomData,
        }
    }

    fn push(&self, event: T) {
        self.in_flight.fetch_add(1, Ordering::SeqCst);
        let producer = self.producer.load(Ordering::SeqCst);
        if let Some(producer) = unsafe { producer.as_ref() } {
            if producer.push_overwrite(event).is_some() {
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
    }

    fn detach(&self) {
        let producer = self.producer.swap(core::ptr::null_mut(), Ordering::SeqCst);
        while self.in_flight.load(Ordering::SeqCst) != 0 {
            std::thread::yield_now();
        }
        if !producer.is_null() {
            drop(unsafe { Box::from_raw(producer) });
        }
    }
}

impl<T> Drop for CaptureSink<T> {
    fn drop(&mut self) {
        let producer = *self.producer.get_mut();
        if !producer.is_null() {
            drop(unsafe { Box::from_raw(producer) });
        }
    }
}

pub(crate) type CaptureContext<T> = CallbackContext<CaptureSink<T>>;

pub(crate) struct Capture<T: Send + 'static> {
    context: CaptureContext<T>,
    consumer: SpscConsumer<T, CAPTURE_CAPACITY_MAX>,
}

impl<T: Send + 'static> Capture<T> {
    pub(crate) fn new(capacity: usize) -> Result<Self, AuError> {
        if capacity == 0 || capacity > CAPTURE_CAPACITY_MAX {
            return Err(AuError::InvalidArgument(format!(
                "capture capacity must be between 1 and {CAPTURE_CAPACITY_MAX}, got {capacity}"
            )));
        }
        let (producer, consumer) = SpscRing::with_capacity(capacity);
        Ok(Self {
            context: CallbackContext::new(CaptureSink::new(producer)),
            consumer,
        })
    }

    pub(crate) fn retained_context(&self) -> *mut c_void {
        self.context.retained_ptr()
    }

    pub(crate) fn drain(&self) -> Vec<T> {
        std::iter::from_fn(|| self.consumer.pop()).collect()
    }

    pub(crate) fn dropped(&self) -> u64 {
        self.context.get().dropped.load(Ordering::Relaxed)
    }

    pub(crate) fn capacity(&self) -> usize {
        self.consumer.capacity()
    }

    pub(crate) fn close(&self) {
        self.context.deactivate();
        self.context.get().detach();
    }
}

impl<T: Send + 'static> Drop for Capture<T> {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MidiOutputRecord {
    event_sample_time: i64,
    cable: u8,
    length: usize,
    bytes: [u8; MIDI_OUTPUT_INLINE_BYTES],
}

impl From<MidiOutputRecord> for AuMidiOutputEvent {
    fn from(record: MidiOutputRecord) -> Self {
        let kept = record.length.min(MIDI_OUTPUT_INLINE_BYTES);
        Self {
            event_sample_time: record.event_sample_time,
            cable: record.cable,
            bytes: record.bytes[..kept].to_vec(),
            truncated: record.length > MIDI_OUTPUT_INLINE_BYTES,
        }
    }
}

unsafe fn push<T: Send + 'static>(context: *mut c_void, site: &str, event: T) {
    let _ = unsafe { CaptureContext::<T>::with(context, site, |sink| sink.push(event)) };
}

pub(crate) unsafe extern "C" fn push_render_observer(
    context: *mut c_void,
    action_flags: u32,
    sample_time: f64,
    host_time: u64,
    frame_count: u32,
    output_bus_number: isize,
) {
    unsafe {
        push(
            context,
            "audiounit::render_observer",
            AuRenderObserverEvent {
                action_flags,
                sample_time,
                host_time,
                frame_count,
                output_bus_number,
            },
        );
    }
}

pub(crate) unsafe extern "C" fn push_midi_output(
    context: *mut c_void,
    event_sample_time: i64,
    cable: u8,
    length: isize,
    bytes: *const u8,
) {
    let length = usize::try_from(length).unwrap_or(0);
    let mut record = MidiOutputRecord {
        event_sample_time,
        cable,
        length,
        bytes: [0; MIDI_OUTPUT_INLINE_BYTES],
    };
    let kept = length.min(MIDI_OUTPUT_INLINE_BYTES);
    if kept > 0 && !bytes.is_null() {
        unsafe { std::ptr::copy_nonoverlapping(bytes, record.bytes.as_mut_ptr(), kept) };
    }
    unsafe { push(context, "audiounit::midi_output", record) };
}

pub(crate) unsafe extern "C" fn push_midi_output_event_list(
    context: *mut c_void,
    event_sample_time: i64,
    cable: u8,
    protocol: i32,
    num_packets: u32,
) {
    unsafe {
        push(
            context,
            "audiounit::midi_output_event_list",
            AuMidiOutputEventListSummary {
                event_sample_time,
                cable,
                protocol,
                num_packets,
            },
        );
    }
}

pub(crate) unsafe extern "C" fn push_parameter_value(
    context: *mut c_void,
    address: u64,
    value: f32,
) {
    unsafe {
        push(
            context,
            "audiounit::parameter_observer",
            AuParameterValueEvent { address, value },
        );
    }
}

pub(crate) unsafe extern "C" fn push_recorded_parameter(
    context: *mut c_void,
    host_time: u64,
    address: u64,
    value: f32,
) {
    unsafe {
        push(
            context,
            "audiounit::parameter_recording_observer",
            AuRecordedParameterEventInfo {
                host_time,
                address,
                value,
            },
        );
    }
}

pub(crate) unsafe extern "C" fn push_parameter_automation(
    context: *mut c_void,
    host_time: u64,
    address: u64,
    value: f32,
    event_type: u32,
) {
    unsafe {
        push(
            context,
            "audiounit::parameter_automation_observer",
            AuParameterAutomationEventInfo {
                host_time,
                address,
                value,
                event_type,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{
        push, push_midi_output, push_parameter_value, push_render_observer, Capture,
        MidiOutputRecord, CAPTURE_CAPACITY_MAX, MIDI_OUTPUT_INLINE_BYTES,
    };
    use crate::au_audio_unit::{AuMidiOutputEvent, AuRenderObserverEvent};
    use crate::au_parameter_tree::AuParameterValueEvent;
    use crate::capture::CaptureContext;

    #[test]
    fn capacity_is_bounded() {
        assert!(Capture::<AuParameterValueEvent>::new(0).is_err());
        assert!(Capture::<AuParameterValueEvent>::new(CAPTURE_CAPACITY_MAX + 1).is_err());
        assert_eq!(
            Capture::<AuParameterValueEvent>::new(8)
                .expect("capture")
                .capacity(),
            8
        );
    }

    #[test]
    fn full_ring_drops_the_oldest_events_and_counts_them() {
        let capture = Capture::<AuParameterValueEvent>::new(4).expect("capture");
        let context = capture.retained_context();
        for address in 0..10_u64 {
            unsafe { push_parameter_value(context, address, 0.5) };
        }
        let events = capture.drain();
        assert_eq!(
            events.iter().map(|event| event.address).collect::<Vec<_>>(),
            vec![6, 7, 8, 9]
        );
        assert_eq!(capture.dropped(), 6);
        assert!(capture.drain().is_empty());
        unsafe { (CaptureContext::<AuParameterValueEvent>::RELEASE)(context) };
    }

    #[test]
    fn pushes_after_the_capture_is_dropped_are_ignored() {
        let capture = Capture::<AuRenderObserverEvent>::new(4).expect("capture");
        let context = capture.retained_context();
        unsafe { push_render_observer(context, 1, 0.0, 0, 256, 0) };
        assert_eq!(capture.drain().len(), 1);
        drop(capture);
        unsafe {
            push_render_observer(context, 1, 0.0, 0, 256, 0);
            (CaptureContext::<AuRenderObserverEvent>::RELEASE)(context);
        }
    }

    #[test]
    fn dropping_a_capture_frees_buffered_events_while_its_context_is_still_retained() {
        static LIVE: AtomicUsize = AtomicUsize::new(0);

        struct Tracked;

        impl Tracked {
            fn new() -> Self {
                LIVE.fetch_add(1, Ordering::SeqCst);
                Self
            }
        }

        impl Drop for Tracked {
            fn drop(&mut self) {
                LIVE.fetch_sub(1, Ordering::SeqCst);
            }
        }

        let capture = Capture::<Tracked>::new(4).expect("capture");
        let context = capture.retained_context();
        for _ in 0..6 {
            unsafe { push(context, "test", Tracked::new()) };
        }
        assert_eq!(LIVE.load(Ordering::SeqCst), 4);
        assert_eq!(capture.dropped(), 2);
        drop(capture);
        assert_eq!(LIVE.load(Ordering::SeqCst), 0);
        unsafe { push(context, "test", Tracked::new()) };
        assert_eq!(LIVE.load(Ordering::SeqCst), 0);
        unsafe { (CaptureContext::<Tracked>::RELEASE)(context) };
    }

    #[test]
    fn midi_output_keeps_short_messages_and_flags_long_ones() {
        let capture = Capture::<MidiOutputRecord>::new(4).expect("capture");
        let context = capture.retained_context();
        let note_on = [0x90_u8, 60, 100];
        let sysex = [0x42_u8; MIDI_OUTPUT_INLINE_BYTES + 10];
        unsafe {
            push_midi_output(context, 7, 1, 3, note_on.as_ptr());
            push_midi_output(
                context,
                8,
                0,
                isize::try_from(sysex.len()).unwrap(),
                sysex.as_ptr(),
            );
            push_midi_output(context, 9, 0, -1, note_on.as_ptr());
        }
        let events: Vec<AuMidiOutputEvent> = capture.drain().into_iter().map(Into::into).collect();
        assert_eq!(events[0].bytes, note_on);
        assert_eq!(events[0].event_sample_time, 7);
        assert_eq!(events[0].cable, 1);
        assert!(!events[0].truncated);
        assert_eq!(events[1].bytes.len(), MIDI_OUTPUT_INLINE_BYTES);
        assert!(events[1].truncated);
        assert!(events[2].bytes.is_empty());
        unsafe { (CaptureContext::<MidiOutputRecord>::RELEASE)(context) };
    }
}
