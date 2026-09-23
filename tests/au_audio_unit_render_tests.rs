use std::cell::RefCell;

use audiounit::prelude::*;

const FRAMES: u32 = 256;
const LEVEL: f32 = 0.25;
const RENDERS: u32 = 6;
const PRE_RENDER: u32 = 1 << 2;
const POST_RENDER: u32 = 1 << 3;

fn delay_unit() -> Result<AuAudioUnit, Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        AudioComponentDescription::apple(AUDIO_UNIT_TYPE_EFFECT, AUDIO_UNIT_SUBTYPE_DELAY),
        InstantiationOptions::InProcess,
    )?;
    let input = unit
        .input_busses()
        .bus_at(0)
        .ok_or("the delay has no input bus")?;
    input.set_standard_format(44_100.0, 1)?;
    input.set_enabled(true);
    unit.output_busses()
        .bus_at(0)
        .ok_or("the delay has no output bus")?
        .set_standard_format(44_100.0, 1)?;
    unit.allocate_render_resources()?;
    Ok(unit)
}

fn render_mono(
    unit: &AuAudioUnit,
    output: &mut [f32],
    render: u32,
    pull_input: Option<&mut AuPullInput<'_>>,
) -> Result<(), AuError> {
    let time_stamp = AudioTimeStamp::with_sample_time(f64::from(render * FRAMES));
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
            pull_input,
        )
    }
}

#[test]
fn render_pulls_effect_input_and_bounds_the_observer() -> Result<(), Box<dyn std::error::Error>> {
    let unit = delay_unit()?;
    let render_observer = unit.add_render_observer_capture(4)?;
    let mut input = vec![LEVEL; FRAMES as usize];
    let mut output = vec![0.0_f32; FRAMES as usize];
    let pulls = RefCell::new(Vec::new());
    let mut pull = |_flags: &mut u32,
                    _time_stamp: &AudioTimeStamp,
                    frame_count: u32,
                    bus: isize,
                    data: *mut AudioBufferList|
     -> i32 {
        pulls.borrow_mut().push((frame_count, bus));
        let Some(buffers) = (unsafe { data.as_mut() }) else {
            return -50;
        };
        let buffer = &mut buffers.buffers[0];
        if buffer.data.is_null() {
            buffer.data = input.as_mut_ptr().cast();
        } else {
            unsafe {
                std::slice::from_raw_parts_mut(buffer.data.cast::<f32>(), frame_count as usize)
            }
            .fill(LEVEL);
        }
        buffer.data_byte_size = frame_count * 4;
        0
    };

    for render in 0..RENDERS {
        render_mono(&unit, &mut output, render, Some(&mut pull))?;
    }

    assert_eq!(pulls.borrow().len(), RENDERS as usize);
    assert!(pulls
        .borrow()
        .iter()
        .all(|&(frames, bus)| frames == FRAMES && bus == 0));
    assert!(
        output.iter().any(|sample| sample.abs() > 0.01),
        "no input reached the output"
    );

    let events = render_observer.take_events();
    let newest: Vec<(u32, f64)> = events
        .iter()
        .map(|event| (event.action_flags, event.sample_time))
        .collect();
    let final_time = f64::from((RENDERS - 1) * FRAMES);
    let penultimate_time = f64::from((RENDERS - 2) * FRAMES);
    assert_eq!(
        newest,
        [
            (PRE_RENDER, penultimate_time),
            (POST_RENDER, penultimate_time),
            (PRE_RENDER, final_time),
            (POST_RENDER, final_time),
        ]
    );
    assert!(events.iter().all(|event| event.frame_count == FRAMES));
    assert_eq!(render_observer.dropped_events(), u64::from(RENDERS * 2 - 4));
    assert!(render_observer.take_events().is_empty());

    drop(render_observer);
    render_mono(&unit, &mut output, RENDERS, Some(&mut pull))?;
    assert_eq!(pulls.borrow().len(), RENDERS as usize + 1);
    Ok(())
}

#[test]
fn render_without_input_reports_an_error_for_an_effect() -> Result<(), Box<dyn std::error::Error>> {
    let unit = delay_unit()?;
    let mut output = vec![0.0_f32; FRAMES as usize];
    assert!(render_mono(&unit, &mut output, 0, None).is_err());
    Ok(())
}
