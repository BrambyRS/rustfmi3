//! The content of this module is means to help as a starting point for implementing FMI 3.0
//! either when exporting or importing FMUs. It implements structs that may be useful, but they
//! are not required to be used.

/// Represents the possible states an FMU can be in accoring to the FMI 3.0 standard.
/// There are restrictions on which states an FMU can transition to from a given state,
/// but it is left to the implementer to enforce these restrictions.
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum FMUState {
    ConfigurationMode,
    ContinuousTimeMode,
    EventMode,
    InitializationMode,
    Instantiated,
    ReconfigurationMode,
    Terminated,
}

// #region FMU variable attributes
/// Represents the causality of a variable in the FMU according to the FMI 3.0 standard.
#[derive(PartialEq)]
#[allow(dead_code)]
enum Causality {
    Parameter,
    CalculatedParameter,
    StructuralParameter,
    Input,
    Output,
    Local,
    Independent,
}

/// Represents the variability of a variable in the FMU according to the FMI 3.0 standard.
#[derive(PartialEq)]
#[allow(dead_code)]
enum Variability {
    Constant,
    Fixed,
    Tunable,
    Discrete,
    Continuous,
}

/// Represents the initial attribute of a variable in the FMU according to the FMI 3.0 standard.
#[derive(PartialEq)]
#[allow(dead_code)]
enum Initial {
    Exact,
    Approx,
    Calculated,
}

// The Dependency kinds already exist in `function_types`
// #endregion FMU variable attributes
