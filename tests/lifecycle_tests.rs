mod support;

use std::sync::{Arc, Barrier};
use std::thread;

use audiounit::legacy;
use audiounit::prelude::*;

const THREADS: usize = 8;

type Work = fn(&Barrier) -> Result<(), AuError>;

const fn status_error(code: i32) -> AuError {
    AuError::Unknown {
        code,
        message: String::new(),
    }
}

fn on_parallel_threads(work: Work) -> Result<(), AuError> {
    let barrier = Arc::new(Barrier::new(THREADS));
    let threads: Vec<_> = (0..THREADS)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || work(&barrier))
        })
        .collect();
    for thread in threads {
        thread.join().expect("worker thread")?;
    }
    Ok(())
}

fn synth() -> Result<AuAudioUnit, AuError> {
    AuAudioUnit::instantiate(
        support::dls_synth_description(),
        InstantiationOptions::InProcess,
    )
}

#[test]
fn synth_render_resources_allocate_on_parallel_threads() -> Result<(), AuError> {
    on_parallel_threads(|barrier| {
        let unit = synth()?;
        barrier.wait();
        unit.allocate_render_resources()?;
        unit.deallocate_render_resources();
        Ok(())
    })
}

#[test]
fn legacy_synths_initialize_on_parallel_threads() -> Result<(), AuError> {
    on_parallel_threads(|barrier| {
        let unit = synth()?;
        unsafe {
            let raw =
                legacy::audio_component_instance_new(unit.component_ptr()).map_err(status_error)?;
            barrier.wait();
            legacy::audio_unit_initialize(raw).map_err(status_error)?;
            legacy::audio_unit_uninitialize(raw).map_err(status_error)?;
            legacy::audio_component_instance_dispose(raw).map_err(status_error)?;
        }
        Ok(())
    })
}

#[test]
fn midi_instruments_instantiate_on_parallel_threads() -> Result<(), AuError> {
    on_parallel_threads(|barrier| {
        barrier.wait();
        drop(AvAudioUnitMidiInstrument::new(
            support::dls_synth_description(),
        )?);
        drop(AvAudioUnitSampler::new()?);
        Ok(())
    })
}
