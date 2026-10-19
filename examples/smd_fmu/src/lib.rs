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
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Float32],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Float32(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFloat64(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Float64],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Float64(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt8(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int8],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Int8(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt8(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt8],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3UInt8(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt16(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int16],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Int16(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt16(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt16],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3UInt16(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt32(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int32],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Int32(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt32(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt32],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3UInt32(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInt64(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int64],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Int64(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUInt64(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt64],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3UInt64(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoolean(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Boolean],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Boolean(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetString(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3String],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3String(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBinary(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    value_sizes: &mut [usize],
    values: &mut [FMI3Binary],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Binary(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

// #region setting variable values
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetClock(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Clock],
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            let value = &mut values[i];
            match fmu_instance.get_variable(vr as usize) {
                Ok(fmu::FMI3DataType::FMI3Clock(v)) => *value = v,
                _ => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFloat32(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Float32],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Float32(values[i]))
            {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFloat64(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Float64],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Float64(values[i]))
            {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt8(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int8],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Int8(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt8(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt8],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3UInt8(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt16(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int16],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Int16(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt16(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt16],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3UInt16(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt32(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int32],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Int32(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt32(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt32],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3UInt32(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInt64(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int64],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Int64(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUInt64(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt64],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3UInt64(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoolean(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Boolean],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Boolean(values[i]))
            {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetString(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3String],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3String(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBinary(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    value_sizes: &[usize],
    values: &[FMI3Binary],
    n_values: usize,
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Binary(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetClock(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Clock],
) -> FMI3Status {
    unsafe {
        let fmu_instance: &mut fmu::FMU = &mut *(instance as *mut fmu::FMU);
        for i in 0..n_value_references {
            let vr = value_references[i];
            match fmu_instance.set_variable(vr as usize, fmu::FMI3DataType::FMI3Clock(values[i])) {
                Ok(()) => {}
                Err(()) => return FMI3Status::ERROR,
            }
        }
        FMI3Status::OK
    }
}
// #endregion setting variable values
// #endregion getting and setting variable values
