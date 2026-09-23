# AudioUnit / AVFAudio logical-area coverage

"Implemented" below means the area has working wrappers exercised by the tests, not that every SDK member of the area is wrapped; see "Not wrapped". The symbol-level scoreboard in `COVERAGE_AUDIT.md` covers a hand-picked sample of 178 symbols (168 wrapped, 5 known gaps, 5 exempt). It is not a full enumeration of the frameworks.

## Summary

| Area | Status | Notes |
| --- | --- | --- |
| `AUAudioUnit` | ✅ implemented | Direct instantiation with a configurable timeout, metadata snapshots, render-resource lifecycle, render invocation with pull input, bounded render-observer and MIDI-output captures, MIDI scheduling helpers, host-context and MIDI-CI APIs, message channels, device selection, hardware control, bus access, parameter-tree access, preset/channel/context setters, overview access, and V2-bridge detection. |
| `AUAudioUnitBus` | ✅ implemented | Format snapshots, standard-format setter, buffer-allocation policy, enabled flag, name, owner-unit lookup, channel-layout tags, and context latency. |
| `AUAudioUnitBusArray` | ✅ implemented | Count/count-changeable snapshots, bus enumeration, and `setBusCount` forwarding where supported. |
| `AUParameter` | ✅ implemented | Metadata snapshots, value get/set, host-time setters, automation-event setter, display-name variants, and string/value conversion. |
| `AUParameterTree` | ✅ implemented | Recursive JSON/tree snapshots, lookup by address, lookup by `(id, scope, element)`, root-group access, and bounded parameter/recording/automation observer captures with owning tokens. |
| `AUParameterGroup` | ✅ implemented | Group snapshots plus `children` and `allParameters` access. |
| `AUAudioUnitFactory` | ✅ implemented | Bridge-backed helper conforming to `AUAudioUnitFactory` and exposing `createAudioUnit(with:)` semantics. |
| `AUAudioUnitV2Bridge` | ✅ implemented | Cast from `AUAudioUnit`, inspect metadata, and access the underlying v2 `AudioUnit` pointer. |
| `AVAudioUnitEffect` | ✅ implemented | AUv2 initializer, metadata snapshots, and `bypass`. |
| `AVAudioUnitGenerator` | ✅ implemented | AUv2 initializer, metadata snapshots, and `bypass`. |
| `AVAudioUnitMIDIInstrument` | ✅ implemented | AUv2 initializer, metadata snapshots, note/controller/program/pitch-bend/pressure/SysEx helpers, and raw MIDI event-list forwarding. |
| `AUVoiceIO` | ✅ implemented | `kAudioUnitSubType_VoiceProcessingIO` instantiation plus bypass, AGC, mute, and other-audio ducking accessors. |
| `AVAudioUnitInstrument` | ⏭️ skipped | Current public macOS SDKs do not expose a separate public `AVAudioUnitInstrument` class; the crate provides a compatibility alias to `AvAudioUnitMidiInstrument` for this logical area. |

## Area notes

### `AUAudioUnit`
- `AuAudioUnit::instantiate` bridges `AUAudioUnit.instantiate(with:options:completionHandler:)` through the Swift bridge; `instantiate_with_timeout` sets how long to wait.
- `AuAudioUnit::info()` snapshots component description/name/version, render-resource state, `maximumFramesToRender`, MIDI capabilities, preset state, channel-map state, bus counts, and V2-bridge availability.
- Mutating entry points are exposed for render resources, reset, context name, current preset, render quality, bypass, offline rendering, and channel maps.
- Advanced coverage includes `renderBlock` with an optional pull-input callback, `scheduleParameterBlock`, render-observer capture, MIDI scheduling and output capture, fixed musical/transport context blocks, MIDI-CI profile helpers, `AUMessageChannel`, and I/O hardware helpers.
- Captures copy plain values into a bounded lock-free ring on the render thread (no allocation or locking), drop the oldest event when full, and unregister when their token is dropped. MIDI-output captures chain the block that was already installed instead of replacing it.
- MIDI event-list captures record only the protocol and packet count of each list.

### `AUAudioUnitBus` / `AUAudioUnitBusArray`
- `AudioFormatInfo` captures sample rate, channel count, common format, interleaving, and a settings-description string.
- Bus arrays expose both typed bus snapshots and handle-based traversal (`bus_at`).
- The bridge keeps owner-unit lookups explicit so Rust can round-trip from bus → unit safely.

### Parameters
- `AuParameterTree::to_json()` preserves a raw JSON representation for callers that prefer schema-free walking.
- `AuParameterNodeInfo` / `AuParameterInfo` provide typed snapshots for tree/group walking.
- `AuParameterAutomationEventType` mirrors the public event-type enum used by AU automation setters.
- Parameter, recording, and automation observer captures return owning tokens that unregister on drop and drain a bounded ring. The framework delivers these notifications asynchronously.

### `AUAudioUnitFactory` / `AUAudioUnitV2Bridge`
- The Swift bridge includes a small helper class that conforms to `AUAudioUnitFactory` and forwards `createAudioUnit(with:)` into synchronous `AUAudioUnit` instantiation.
- `AuAudioUnitV2Bridge` is exposed only when the instantiated AU actually subclasses the public bridge type.

### `AVAudioUnit` subclasses
- `AvAudioUnit` remains the generic synchronous wrapper and now exposes typed metadata snapshots plus real preset loading.
- `ComponentManager` also covers predicate/test enumeration and richer `AudioUnitComponent` metadata, including user tags, all tags, architectures, configuration dictionaries, and channel-compatibility probes.
- Effect/generator/MIDI-instrument wrappers each own their subclass handle and can clone the underlying generic `AvAudioUnit` when needed.
- Additional concrete wrappers now cover `AVAudioUnitSampler`, `AVAudioUnitEQ`, `AVAudioUnitDelay`, `AVAudioUnitDistortion`, `AVAudioUnitReverb`, `AVAudioUnitTimeEffect`, `AVAudioUnitTimePitch`, and `AVAudioUnitVarispeed`.
- `AVAudioUnitInstrument` remains a documented compatibility alias because the public SDK surface is `AVAudioUnitMIDIInstrument`.

### `AUVoiceIO`
- `AuVoiceIo` wraps Apple’s voice-processing output unit (`'vpio'`).
- Getter/setter coverage includes bypassed processing, AGC, mute state, and other-audio ducking configuration when the runtime exposes it.

## Not wrapped

- `AUAudioUnit.outputProvider` and `inputHandler`.
- `AUAudioUnit.saveUserPreset:error:`, `deleteUserPreset:error:`, and `presetStateFor:error:`.
- `requestViewControllerWithCompletionHandler:` (CoreAudioKit).
- `AUAudioUnit` subclassing (`internalRenderBlock`) for implementing audio units in Rust.

## Validation

- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
