# FMI 3.0 Interface in Rust

FMI 3.0 types and functions for Rust-based FMU implementations.

## Architecture

FMI 3.0 defines all functions in terms of function pointers. This crate provides:
- **Function type definitions** (in `crate::function_types`)
- **Platform types** (in `crate::platform_types`)
- **Constants** (in crate root)

## For FMU Exporters (Rust → FMU)

To export a Rust model as an FMU, implement the FMI 3.0 functions and expose them via `#[no_mangle]` extern "C" declarations.

### Example: Model Exchange FMU

A simple first-order system: `dx/dt = -x` with state serialization:

```rust
use fmi3::platform_types::*;
use fmi3::function_types::*;
use std::ffi::{c_char, CStr};
use std::collections::HashMap;

// Simple FMU model state
struct ModelState {
    x: FMI3Float64,  // continuous state
    time: FMI3Float64,
}

// Global instance storage (in production, use proper instance management)
static mut INSTANCES: HashMap<FMI3Instance, ModelState> = HashMap::new();
static mut NEXT_INSTANCE_ID: usize = 0;

#[no_mangle]
pub extern "C" fn fmi3GetVersion() -> *const c_char {
    b"3.0\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn fmi3SetDebugLogging(
    instance: FMI3Instance,
    logging_on: FMI3Boolean,
    n_categories: usize,
    categories: *const FMI3String,
) -> FMI3Status {
    FMI3Status::OK
}

#[no_mangle]
pub extern "C" fn fmi3EnterContinuousTimeMode(
    instance: FMI3Instance,
) -> FMI3Status {
    FMI3Status::OK
}

#[no_mangle]
pub extern "C" fn fmi3SetTime(
    instance: FMI3Instance,
    time: FMI3Float64,
) -> FMI3Status {
    unsafe {
        if let Some(state) = INSTANCES.get_mut(&instance) {
            state.time = time;
            FMI3Status::OK
        } else {
            FMI3Status::ERROR
        }
    }
}

#[no_mangle]
pub extern "C" fn fmi3SetContinuousStates(
    instance: FMI3Instance,
    continuous_states: &[FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status {
    unsafe {
        if let Some(state) = INSTANCES.get_mut(&instance) {
            if n_continuous_states >= 1 {
                state.x = continuous_states[0];
                FMI3Status::OK
            } else {
                FMI3Status::ERROR
            }
        } else {
            FMI3Status::ERROR
        }
    }
}

#[no_mangle]
pub extern "C" fn fmi3GetContinuousStateDerivatives(
    instance: FMI3Instance,
    derivatives: &mut [FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status {
    unsafe {
        if let Some(state) = INSTANCES.get(&instance) {
            if n_continuous_states >= 1 {
                // Simple model: dx/dt = -x
                derivatives[0] = -state.x;
                FMI3Status::OK
            } else {
                FMI3Status::ERROR
            }
        } else {
            FMI3Status::ERROR
        }
    }
}

#[no_mangle]
pub extern "C" fn fmi3GetContinuousStates(
    instance: FMI3Instance,
    continuous_states: &mut [FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status {
    unsafe {
        if let Some(state) = INSTANCES.get(&instance) {
            if n_continuous_states >= 1 {
                continuous_states[0] = state.x;
                FMI3Status::OK
            } else {
                FMI3Status::ERROR
            }
        } else {
            FMI3Status::ERROR
        }
    }
}
```

## For FMU Consumers (FMU → Rust)

To load and interact with external FMU shared libraries, you can represent a loaded FMU as a struct that holds all function pointers:

```rust
/// Represents a loaded FMU shared library with all function pointers
pub struct FMUInstance {
    get_version: FMI3GetVersionTYPE,
    set_debug_logging: FMI3SetDebugLoggingTYPE,
    instantiate_model_exchange: FMI3InstantiateModelExchangeTYPE,
    // ... other function pointers ...
}

impl FMUInstance {
    /// Load an FMU from a shared library file
    pub unsafe fn load(path: &str) -> Result<Self, String> {
        // Use libloading crate to load shared library
        // and bind all FMI 3.0 functions
        todo!()
    }
}
```

Then use it like:

```rust
let fmu = FMUInstance::load("path/to/fmu.so")?;
let instance = fmu.instantiate_model_exchange(...)?;
fmu.set_time(instance, 0.0)?;
```