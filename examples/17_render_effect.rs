use std::f32::consts::TAU;

use audiounit::prelude::*;

const FRAMES: u32 = 512;
const SAMPLE_RATE: f32 = 44_100.0;
const PHASE_STEP: f32 = TAU * 440.0 / SAMPLE_RATE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        AudioComponentDescription::apple(AUDIO_UNIT_TYPE_EFFECT, AUDIO_UNIT_SUBTYPE_DELAY),
        InstantiationOptions::InProcess,
    )?;
    let input_bus = unit.input_busses().bus_at(0).ok_or("no input bus")?;
    input_bus.set_standard_format(f64::from(SAMPLE_RATE), 1)?;
    input_bus.set_enabled(true);
    unit.output_busses()
        .bus_at(0)
        .ok_or("no output bus")?
        .set_standard_format(f64::from(SAMPLE_RATE), 1)?;
    let observer = unit.add_render_observer_capture(64)?;
    unit.allocate_render_resources()?;

    let mut phase = 0.0_f32;
    let mut tone = vec![0.0_f32; FRAMES as usize];
    let mut pull = |_: &mut u32,
                    _: &AudioTimeStamp,
                    frames: u32,
                    _: isize,
                    data: *mut AudioBufferList|
     -> i32 {
        let Some(list) = (unsafe { data.as_mut() }) else {
            return -50;
        };
        let frames = frames.min(FRAMES) as usize;
        for sample in &mut tone[..frames] {
            *sample = 0.5 * phase.sin();
            phase = (phase + PHASE_STEP) % TAU;
        }
        let buffer = &mut list.buffers[0];
        if buffer.data.is_null() {
            buffer.data = tone.as_mut_ptr().cast();
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(tone.as_ptr(), buffer.data.cast::<f32>(), frames);
            }
        }
        buffer.data_byte_size = u32::try_from(frames * 4).unwrap_or(0);
        0
    };

    let mut output = vec![0.0_f32; FRAMES as usize];
    for block in 0..8_u32 {
        let time_stamp = AudioTimeStamp::with_sample_time(f64::from(block * FRAMES));
        let mut flags = 0;
        let mut list = AudioBufferList {
            number_buffers: 1,
            buffers: [AudioBuffer {
                number_channels: 1,
                data_byte_size: FRAMES * 4,
                data: output.as_mut_ptr().cast(),
            }],
        };
        unsafe {
            unit.render(
                &raw mut flags,
                &raw const time_stamp,
                FRAMES,
                0,
                &raw mut list,
                Some(&mut pull),
            )
        }?;
        let peak = output
            .iter()
            .fold(0.0_f32, |peak, sample| peak.max(sample.abs()));
        println!("block {block}: output peak {peak:.3}");
    }

    let events = observer.take_events();
    println!(
        "render observer saw {} callbacks ({} dropped)",
        events.len(),
        observer.dropped_events()
    );
    unit.deallocate_render_resources();
    Ok(())
}
