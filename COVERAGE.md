# AudioUnit / AVFAudio logical-area coverage (v0.2.0)

Audited against the locally installed macOS SDK (`xcrun --sdk macosx --show-sdk-version`) while implementing the v0.2.0 expansion wave for `audiounit-rs`.

## Summary

| Area | Status | Notes |
| --- | --- | --- |
| `AUAudioUnit` | ✅ implemented | Direct instantiation, metadata snapshots, render-resource lifecycle, bus access, parameter-tree access, preset/channel/context setters, overview access, and V2-bridge detection. |
| `AUAudioUnitBus` | ✅ implemented | Format snapshots, standard-format setter, buffer-allocation policy, enabled flag, name, owner-unit lookup, channel-layout tags, and context latency. |
| `AUAudioUnitBusArray` | ✅ implemented | Count/count-changeable snapshots, bus enumeration, and `setBusCount` forwarding where supported. |
| `AUParameter` | ✅ implemented | Metadata snapshots, value get/set, host-time setters, automation-event setter, display-name variants, and string/value conversion. |
| `AUParameterTree` | ✅ implemented | Recursive JSON/tree snapshots, lookup by address, lookup by `(id, scope, element)`, and root-group access. |
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
- `AuAudioUnit::instantiate` bridges `AUAudioUnit.instantiate(with:options:completionHandler:)` through the Swift bridge.
- `AuAudioUnit::info()` snapshots component description/name/version, render-resource state, `maximumFramesToRender`, MIDI capabilities, preset state, channel-map state, bus counts, and V2-bridge availability.
- Mutating entry points are exposed for render resources, reset, context name, current preset, render quality, bypass, offline rendering, and channel maps.

### `AUAudioUnitBus` / `AUAudioUnitBusArray`
- `AudioFormatInfo` captures sample rate, channel count, common format, interleaving, and a settings-description string.
- Bus arrays expose both typed bus snapshots and handle-based traversal (`bus_at`).
- The bridge keeps owner-unit lookups explicit so Rust can round-trip from bus → unit safely.

### Parameters
- `AuParameterTree::to_json()` preserves a raw JSON representation for callers that prefer schema-free walking.
- `AuParameterNodeInfo` / `AuParameterInfo` provide typed snapshots for tree/group walking.
- `AuParameterAutomationEventType` mirrors the public event-type enum used by AU automation setters.

### `AUAudioUnitFactory` / `AUAudioUnitV2Bridge`
- The Swift bridge includes a small helper class that conforms to `AUAudioUnitFactory` and forwards `createAudioUnit(with:)` into synchronous `AUAudioUnit` instantiation.
- `AuAudioUnitV2Bridge` is exposed only when the instantiated AU actually subclasses the public bridge type.

### `AVAudioUnit` subclasses
- `AvAudioUnit` remains the generic synchronous wrapper and now exposes typed metadata snapshots.
- Effect/generator/MIDI-instrument wrappers each own their subclass handle and can clone the underlying generic `AvAudioUnit` when needed.
- `AVAudioUnitInstrument` remains a documented compatibility alias because the public SDK surface is `AVAudioUnitMIDIInstrument`.

### `AUVoiceIO`
- `AuVoiceIo` wraps Apple’s voice-processing output unit (`'vpio'`).
- Getter/setter coverage includes bypassed processing, AGC, mute state, and other-audio ducking configuration when the runtime exposes it.

## Validation

- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
