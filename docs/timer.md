# Timer Module Reference

## TimerState (Enum)

The possible states a timer can be in:

| State      | Meaning                              |
|------------|--------------------------------------|
| `Running`  | Timer is actively counting down      |
| `Paused`   | Timer is paused, can be resumed      |
| `Reset`    | Timer is at its starting value       |
| `Finished` | Timer has reached zero               |

## Timer (Struct)

A countdown timer with three private fields:

| Field            | Type         | Description                          |
|------------------|--------------|--------------------------------------|
| `remaining_secs` | `u64`        | Seconds left on the timer            |
| `starting_secs`  | `u64`        | Original duration (used by reset)    |
| `state`          | `TimerState` | Current state of the timer           |

## Functions

### Constructor

| Function               | Returns | Description                                                     |
|------------------------|---------|-----------------------------------------------------------------|
| `Timer::new(minutes)`  | `Timer` | Creates a new timer with the given duration in minutes. Starts in the `Reset` state. |

### Getters

| Function      | Returns        | Description                        |
|---------------|----------------|------------------------------------|
| `state()`     | `&TimerState`  | Returns the current timer state    |
| `remaining()` | `u64`          | Returns the remaining seconds      |

### TimerControl Trait

| Function          | Returns              | Description                                                                                  |
|-------------------|----------------------|----------------------------------------------------------------------------------------------|
| `tick()`          | nothing              | Subtracts one second if the timer is `Running`. Changes state to `Finished` when it hits zero. |
| `get_remaining()` | `u64`                | Returns the remaining seconds.                                                                |
| `reset()`         | nothing              | Restores the timer to its original duration and sets state to `Reset`.                        |
| `pause()`         | `Result<(), String>` | Pauses the timer. Returns an error if the timer isn't currently `Running`.                    |
| `start()`         | `Result<(), String>` | Starts the timer. Returns an error if the timer is `Finished` or already `Running`.           |
