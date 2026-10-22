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
