//! Defining types for the FMI 3.0 interface
#![allow(dead_code)]

use crate::platform_types::*;
use std::ffi::c_char;

// #region enums
/// Status codes for FMI 3.0
pub enum FMI3Status {
    OK = 0,
    WARNING = 1,
    DISCARD = 2,
    ERROR = 3,
    FATAL = 4,
}

/// Dependency types for FMI 3.0
pub enum FMI3DependencyKind {
    Independent,
    Constant,
    Fixed,
    Tunable,
    Discrete,
    Dependent,
}

/// Interval qualifiers for FMI 3.0
pub enum FMI3IntervalQualifier {
    NotYetKnown,
    Unchanged,
    Changed,
}
// #endregion enums

// #region callback functions
pub type FMI3LogMessageCallback = fn(
    instance_environment: FMI3InstanceEnvironment,
    status: FMI3Status,
    category: FMI3String,
    message: FMI3String,
);

pub type FMI3ClockUpdateCallback = fn(instance_environment: FMI3InstanceEnvironment);

pub type FMI3IntermediateUpdateCallback = fn(
    instance_environment: FMI3InstanceEnvironment,
    intermediate_update_time: FMI3Float64,
    intermediate_variable_set_requested: FMI3Boolean,
    intermediate_variable_get_allowed: FMI3Boolean,
    intermediate_step_finished: FMI3Boolean,
    can_return_early: FMI3Boolean,
    early_return_requested: *mut FMI3Boolean,
    early_return_time: *mut FMI3Float64,
);

pub type FMI3LockPreemptionCallback = fn();
pub type FMI3UnlockPreemptionCallback = fn();
// #endregion callback functions

// #region common functions
pub type FMI3GetVersionTYPE = fn() -> *const c_char;

pub type FMI3SetDebugLoggingTYPE = fn(
    instance: FMI3Instance,
    logging_on: FMI3Boolean,
    n_categories: usize,
    categories: *const FMI3String,
);
// #endregion common functions

// #region creation and destruction of FMU
pub type FMI3InstantiateModelExchangeTYPE = fn(
    instance_name: FMI3String,
    instantiation_token: FMI3String,
    resource_path: FMI3String,
    visible: FMI3Boolean,
    logging_on: FMI3Boolean,
    instance_environment: FMI3InstanceEnvironment,
    log_message: FMI3LogMessageCallback,
) -> FMI3Instance;

pub type FMI3InstantiateCoSimulationTYPE = fn(
    instance_name: FMI3String,
    instantiation_token: FMI3String,
    resource_path: FMI3String,
    visible: FMI3Boolean,
    logging_on: FMI3Boolean,
    event_mode_used: FMI3Boolean,
    early_return_allowed: FMI3Boolean,
    required_intermediate_variables: *const FMI3ValueReference,
    n_required_intermediate_variables: usize,
    instance_environment: FMI3InstanceEnvironment,
    log_message: FMI3LogMessageCallback,
    intermediate_update: FMI3IntermediateUpdateCallback,
) -> FMI3Instance;

pub type FMI3InstantiateScheduledExecutionTYPE = fn(
    instance_name: FMI3String,
    instantiation_token: FMI3String,
    resource_path: FMI3String,
    visible: FMI3Boolean,
    logging_on: FMI3Boolean,
    instance_environment: FMI3InstanceEnvironment,
    log_message: FMI3LogMessageCallback,
    clock_update: FMI3ClockUpdateCallback,
    lock_preemption: FMI3LockPreemptionCallback,
    unlock_preemption: FMI3UnlockPreemptionCallback,
) -> FMI3Instance;

pub type FMI3FreeInstanceTYPE = fn(instance: FMI3Instance);
// #endregion creation and destruction of FMU

// #region enter/exit initialization mode, enter event mode, terminate, and reset
pub type FMI3EnterInitializationModeTYPE = fn(
    instance: FMI3Instance,
    tolerance_defined: FMI3Boolean,
    tolerance: FMI3Float64,
    start_time: FMI3Float64,
    stop_time_defined: FMI3Boolean,
    stop_time: FMI3Float64,
) -> FMI3Status;

pub type FMI3ExitInitializationModeTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3EnterEventModeTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3TerminateTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3ResetTYPE = fn(instance: FMI3Instance) -> FMI3Status;
// #endregion enter/exit initialization mode, enter event mode, terminate, and reset

// #region getting and setting variable values
pub type FMI3GetFloat32TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Float32],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetFloat64TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Float64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetInt8TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int8],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetUInt8TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt8],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetInt16TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int16],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetUInt16TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt16],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetInt32TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int32],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetUInt32TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt32],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetInt64TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Int64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetUInt64TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3UInt64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetBooleanTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Boolean],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetStringTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3String],
    n_values: usize,
) -> FMI3Status;

pub type FMI3GetBinaryTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    value_sizes: &mut [usize],
    values: &mut [FMI3Binary],
    n_values: usize,
) -> FMI3Status;

// #region setting variable values
pub type FMI3GetClockTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &mut [FMI3Clock],
) -> FMI3Status;

pub type FMI3SetFloat32TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Float32],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetFloat64TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Float64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetInt8TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int8],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetUInt8TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt8],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetInt16TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int16],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetUInt16TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt16],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetInt32TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int32],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetUInt32TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt32],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetInt64TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Int64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetUInt64TYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3UInt64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetBooleanTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Boolean],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetStringTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3String],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetBinaryTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    value_sizes: &[usize],
    values: &[FMI3Binary],
    n_values: usize,
) -> FMI3Status;

pub type FMI3SetClockTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    values: &[FMI3Clock],
) -> FMI3Status;
// #endregion setting variable values
// #endregion getting and setting variable values

// #region variable dependency information
pub type FMI3GetNumberOfVariableDependenciesTYPE = fn(
    instance: FMI3Instance,
    value_reference: FMI3ValueReference,
    n_dependencies: *mut usize,
) -> FMI3Status;

pub type FMI3GetVariableDependenciesTYPE = fn(
    instance: FMI3Instance,
    dependent: FMI3ValueReference,
    element_indices_of_dependent: &[usize],
    independents: &[FMI3ValueReference],
    element_indices_of_independents: &[usize],
    dependency_kinds: &[FMI3DependencyKind],
    n_dependencies: usize,
) -> FMI3Status;
// #endregion variable dependency information

// #region FMU state
pub type FMI3GetFMUStateTYPE =
    fn(instance: FMI3Instance, fmu_state: *mut FMI3FMUState) -> FMI3Status;

pub type FMI3SetFMUStateTYPE = fn(instance: FMI3Instance, fmu_state: FMI3FMUState) -> FMI3Status;

pub type FMI3FreeFMUStateTYPE =
    fn(instance: FMI3Instance, fmu_state: *mut FMI3FMUState) -> FMI3Status;

pub type FMI3SerializedFMUStateSizeTYPE =
    fn(instance: FMI3Instance, fmu_state: FMI3FMUState, size: *mut usize) -> FMI3Status;

pub type FMI3SerializeFMUStateTYPE = fn(
    instance: FMI3Instance,
    fmu_state: FMI3FMUState,
    serialized_state: &[FMI3Byte],
    size: usize,
) -> FMI3Status;

pub type FMI3DeserializeFMUStateTYPE = fn(
    instance: FMI3Instance,
    serialized_state: &[FMI3Byte],
    size: usize,
    fmu_state: *mut FMI3FMUState,
) -> FMI3Status;
// #endregion FMU state

// #region partial derivatives
pub type FMI3GetDirectionalDerivativeTYPE = fn(
    instance: FMI3Instance,
    unknowns: &[FMI3ValueReference],
    n_unknowns: usize,
    knowns: &[FMI3ValueReference],
    n_knowns: usize,
    seed: &[FMI3Float64],
    n_seed: usize,
    sensitivity: &mut [FMI3Float64],
    n_sensitivity: usize,
) -> FMI3Status;

pub type FMI3GetAdjointDerivativeTYPE = fn(
    instance: FMI3Instance,
    unknowns: &[FMI3ValueReference],
    n_unknowns: usize,
    knowns: &[FMI3ValueReference],
    n_knowns: usize,
    seed: &[FMI3Float64],
    n_seed: usize,
    sensitivity: &mut [FMI3Float64],
    n_sensitivity: usize,
) -> FMI3Status;
// #endregion partial derivatives

// #region configuration mode and intervals
pub type FMI3EnterConfigurationModeTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3ExitConfigurationModeTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3GetIntervalDecimalTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    intervals: &mut [FMI3Float64],
    qualifiers: &mut [FMI3IntervalQualifier],
) -> FMI3Status;

pub type FMI3GetIntervalFractionTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    counters: &mut [FMI3UInt64],
    resolutions: &mut [FMI3UInt64],
    qualifiers: &mut [FMI3IntervalQualifier],
) -> FMI3Status;

pub type FMI3GetShiftDecimalTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    shifts: &mut [FMI3Float64],
) -> FMI3Status;

pub type FMI3GetShiftFractionTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    counters: &mut [FMI3UInt64],
    resolutions: &mut [FMI3UInt64],
) -> FMI3Status;

pub type FMI3SetIntervalDecimalTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    intervals: &[FMI3Float64],
) -> FMI3Status;

pub type FMI3SetIntervalFractionTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    counters: &[FMI3UInt64],
    resolutions: &[FMI3UInt64],
) -> FMI3Status;

pub type FMI3SetShiftDecimalTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    shifts: &[FMI3Float64],
) -> FMI3Status;

pub type FMI3SetShiftFractionTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    counters: &[FMI3UInt64],
    resolutions: &[FMI3UInt64],
) -> FMI3Status;

pub type FMI3EvaluateDiscreteStatesTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3UpdateDiscreteStatesTYPE = fn(
    instance: FMI3Instance,
    discrete_states_need_update: *mut FMI3Boolean,
    terminate_simulation: *mut FMI3Boolean,
    nominals_of_continuous_states_changed: *mut FMI3Boolean,
    values_of_continuous_states_changed: *mut FMI3Boolean,
    next_event_time_defined: *mut FMI3Boolean,
    next_event_time: *mut FMI3Float64,
) -> FMI3Status;
// #endregion configuration mode and intervals

// #region model exchange functions
pub type FMI3EnterContinuousTimeModeTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3CompletedIntegratorStepTYPE = fn(
    instance: FMI3Instance,
    no_set_fmu_state_prior_to_current_point: FMI3Boolean,
    enter_event_mode: *mut FMI3Boolean,
    terminate_simulation: *mut FMI3Boolean,
) -> FMI3Status;

pub type FMI3SetTimeTYPE = fn(instance: FMI3Instance, time: FMI3Float64) -> FMI3Status;

pub type FMI3SetContinuousStatesTYPE = fn(
    instance: FMI3Instance,
    continuous_states: &[FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status;

pub type FMI3GetContinuousStateDerivativesTYPE = fn(
    instance: FMI3Instance,
    derivatives: &mut [FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status;

pub type FMI3GetEventIndicatorsTYPE = fn(
    instance: FMI3Instance,
    event_indicators: &mut [FMI3Float64],
    n_event_indicators: usize,
) -> FMI3Status;

pub type FMI3GetContinuousStatesTYPE = fn(
    instance: FMI3Instance,
    continuous_states: &mut [FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status;

pub type FMI3GetNominalsOfContinuousStatesTYPE = fn(
    instance: FMI3Instance,
    nominals: &mut [FMI3Float64],
    n_continuous_states: usize,
) -> FMI3Status;

pub type FMI3GetNumberOfEventIndicatorsTYPE =
    fn(instance: FMI3Instance, n_event_indicators: *mut usize) -> FMI3Status;

pub type FMI3GetNumberOfContinuousStatesTYPE =
    fn(instance: FMI3Instance, n_continuous_states: *mut usize) -> FMI3Status;
// #endregion model exchange functions

// #region co-simulation functions
pub type FMI3EnterStepModeTYPE = fn(instance: FMI3Instance) -> FMI3Status;

pub type FMI3GetOutputDerivativesTYPE = fn(
    instance: FMI3Instance,
    value_references: &[FMI3ValueReference],
    n_value_references: usize,
    orders: &[FMI3Int32],
    values: &mut [FMI3Float64],
    n_values: usize,
) -> FMI3Status;

pub type FMI3DoStepTYPE = fn(
    instance: FMI3Instance,
    current_communication_point: FMI3Float64,
    communication_step_size: FMI3Float64,
    no_set_fmu_state_prior_to_current_point: FMI3Boolean,
    event_handling_needed: *mut FMI3Boolean,
    terminate_simulation: *mut FMI3Boolean,
    early_return: *mut FMI3Boolean,
    last_successful_time: *mut FMI3Float64,
) -> FMI3Status;
// #endregion co-simulation functions

// #region scheduled execution functions
pub type FMI3ActivateModelPartitionTYPE = fn(
    instance: FMI3Instance,
    clock_reference: FMI3ValueReference,
    activation_time: FMI3Float64,
) -> FMI3Status;
// #endregion scheduled execution functions
