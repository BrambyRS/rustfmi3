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

// #region getting and setting variable values
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFloat32(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Float32,
    _: usize,
) -> FMI3Status {
    // There are no Float32 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFloat64(
    instance: FMI3Instance,
    value_references: *const FMI3ValueReference,
    n_value_references: usize,
    values: *mut FMI3Float64,
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        let vr_slice = std::slice::from_raw_parts(value_references, n_value_references);
        let val_slice = std::slice::from_raw_parts_mut(values, n_values);
        match fmu_instance.get_variable(vr_slice, n_value_references, val_slice, n_values) {
            true => FMI3Status::OK,
            false => FMI3Status::ERROR,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt8(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Int8,
    _: usize,
) -> FMI3Status {
    // There are no Int8 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt8(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3UInt8,
    _: usize,
) -> FMI3Status {
    // There are no UInt8 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt16(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Int16,
    _: usize,
) -> FMI3Status {
    // There are no Int16 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt16(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3UInt16,
    _: usize,
) -> FMI3Status {
    // There are no UInt16 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt32(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Int32,
    _: usize,
) -> FMI3Status {
    // There are no Int32 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt32(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3UInt32,
    _: usize,
) -> FMI3Status {
    // There are no UInt32 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt64(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Int64,
    _: usize,
) -> FMI3Status {
    // There are no Int64 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt64(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3UInt64,
    _: usize,
) -> FMI3Status {
    // There are no UInt64 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoolean(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Boolean,
    _: usize,
) -> FMI3Status {
    // There are no Boolean variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetString(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3String,
    _: usize,
) -> FMI3Status {
    // There are no String variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBinary(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut usize,
    _: *mut FMI3Binary,
    _: usize,
) -> FMI3Status {
    // There are no Binary variables in this model.
    return FMI3Status::ERROR;
}

// #region setting variable values
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetClock(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *mut FMI3Clock,
) -> FMI3Status {
    // There are no Clock variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFloat32(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Float32,
    _: usize,
) -> FMI3Status {
    // There are no Float32 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFloat64(
    instance: FMI3Instance,
    value_references: *const FMI3ValueReference,
    n_value_references: usize,
    values: *const FMI3Float64,
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        let vr_slice = std::slice::from_raw_parts(value_references, n_value_references);
        let val_slice = std::slice::from_raw_parts(values, n_values);
        match fmu_instance.set_variable(
            vr_slice,
            n_value_references,
            &mut val_slice.to_vec(),
            n_values,
        ) {
            true => FMI3Status::OK,
            false => FMI3Status::ERROR,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt8(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Int8,
    _: usize,
) -> FMI3Status {
    // There are no Int8 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt8(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3UInt8,
    _: usize,
) -> FMI3Status {
    // There are no UInt8 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt16(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Int16,
    _: usize,
) -> FMI3Status {
    // There are no Int16 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt16(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3UInt16,
    _: usize,
) -> FMI3Status {
    // There are no UInt16 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt32(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Int32,
    _: usize,
) -> FMI3Status {
    // There are no Int32 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt32(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3UInt32,
    _: usize,
) -> FMI3Status {
    // There are no UInt32 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt64(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Int64,
    _: usize,
) -> FMI3Status {
    // There are no Int64 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt64(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3UInt64,
    _: usize,
) -> FMI3Status {
    // There are no UInt64 variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoolean(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Boolean,
    _: usize,
) -> FMI3Status {
    // There are no Boolean variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetString(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3String,
    _: usize,
) -> FMI3Status {
    // There are no String variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBinary(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const usize,
    _: *const FMI3Binary,
    _: usize,
) -> FMI3Status {
    // There are no Binary variables in this model.
    return FMI3Status::ERROR;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetClock(
    _: FMI3Instance,
    _: *const FMI3ValueReference,
    _: usize,
    _: *const FMI3Clock,
) -> FMI3Status {
    // There are no Clock variables in this model.
    return FMI3Status::ERROR;
}
// #endregion setting variable values
// #endregion getting and setting variable values
