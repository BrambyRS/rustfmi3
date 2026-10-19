mod fmu;
mod model;

use fmi3::{function_types::*, platform_types::*};
use std::ffi::{c_char, c_void};

// #region common functions
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3GetVersion() -> *const c_char {
    std::ffi::CStr::from_bytes_with_nul(b"3.0\0")
        .unwrap()
        .as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3SetDebugLogging(
    _: FMI3Instance,
    _: FMI3Boolean,
    _: usize,
    _: *const FMI3String,
) {
    // Do nothing for now
}
// #endregion common functions

// #region creation and destruction of FMU
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3InstantiateModelExchange(
    instance_name: FMI3String,
    instantiation_token: FMI3String,
    resource_path: FMI3String,
    visible: FMI3Boolean,
    logging_on: FMI3Boolean,
    instance_environment: FMI3InstanceEnvironment,
    log_message: FMI3LogMessageCallback,
) -> FMI3Instance {
    let fmu = Box::new(fmu::FMU::new(1.0, 1.0, 1.0));
    return Box::into_raw(fmu) as FMI3FMUState;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3FreeInstance(instance: FMI3Instance) {
    unsafe {
        // Take ownership of the Box and drop it out of scope
        if !instance.is_null() {
            let _ = Box::from_raw(instance as *mut fmu::FMU);
        }
    }
}
// #endregion creation and destruction of FMU

// #region enter/exit initialization mode, enter event mode, terminate, and reset

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3EnterInitializationMode(
    instance: FMI3Instance,
    tolerance_defined: FMI3Boolean,
    tolerance: FMI3Float64,
    start_time: FMI3Float64,
    stop_time_defined: FMI3Boolean,
    stop_time: FMI3Float64,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        if fmu_instance.state != fmu::FMUState::Instantiated {
            fmu_instance.state = fmu::FMUState::Terminated;
            return FMI3Status::ERROR;
        }
        fmu_instance.state = fmu::FMUState::InitializationMode;
        return FMI3Status::OK;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3ExitInitializationMode(instance: FMI3Instance) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        if fmu_instance.state != fmu::FMUState::InitializationMode {
            fmu_instance.state = fmu::FMUState::Terminated;
            return FMI3Status::ERROR;
        }
        fmu_instance.state = fmu::FMUState::ContinuousTimeMode;
        return FMI3Status::OK;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3EnterEventMode(instance: FMI3Instance) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        if fmu_instance.state != fmu::FMUState::ContinuousTimeMode {
            fmu_instance.state = fmu::FMUState::Terminated;
            return FMI3Status::ERROR;
        }
        fmu_instance.state = fmu::FMUState::EventMode;
        return FMI3Status::OK;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3EnterContinuousTimeMode(instance: FMI3Instance) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        if fmu_instance.state != fmu::FMUState::EventMode {
            fmu_instance.state = fmu::FMUState::Terminated;
            return FMI3Status::ERROR;
        }
        fmu_instance.state = fmu::FMUState::ContinuousTimeMode;
        return FMI3Status::OK;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3Terminate(instance: FMI3Instance) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        fmu_instance.state = fmu::FMUState::Terminated;
        return FMI3Status::OK;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3Reset(instance: FMI3Instance) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        fmu_instance.state = fmu::FMUState::Instantiated;
        return FMI3Status::OK;
    }
}
// #endregion enter/exit initialization mode, enter event mode, terminate, and reset
