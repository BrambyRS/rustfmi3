//! This module implements the FMU wrapper for the spring mass damper model,
//! with all the necessary functionality for the FMI 3.0 standard.

use crate::model;
use fmi3::platform_types::*;

/// Enumeration of the different data types that can be set or retrieved in the FMI 3.0 interface.
#[allow(dead_code)]
pub enum FMI3DataType {
    FMI3Boolean(Vec<FMI3Boolean>),
    FMI3Float32(Vec<FMI3Float32>),
    FMI3Float64(Vec<FMI3Float64>),
    FMI3Int8(Vec<FMI3Int8>),
    FMI3Int16(Vec<FMI3Int16>),
    FMI3Int32(Vec<FMI3Int32>),
    FMI3Int64(Vec<FMI3Int64>),
    FMI3UInt8(Vec<FMI3UInt8>),
    FMI3UInt16(Vec<FMI3UInt16>),
    FMI3UInt32(Vec<FMI3UInt32>),
    FMI3UInt64(Vec<FMI3UInt64>),
    FMI3Binary(Vec<FMI3Binary>),
    FMI3String(Vec<FMI3String>),
    FMI3Clock(Vec<FMI3Clock>),
}

impl std::cmp::PartialEq for FMI3DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FMI3DataType::FMI3Boolean(_), FMI3DataType::FMI3Boolean(_)) => true,
            (FMI3DataType::FMI3Float32(_), FMI3DataType::FMI3Float32(_)) => true,
            (FMI3DataType::FMI3Float64(_), FMI3DataType::FMI3Float64(_)) => true,
            (FMI3DataType::FMI3Int8(_), FMI3DataType::FMI3Int8(_)) => true,
            (FMI3DataType::FMI3Int16(_), FMI3DataType::FMI3Int16(_)) => true,
            (FMI3DataType::FMI3Int32(_), FMI3DataType::FMI3Int32(_)) => true,
            (FMI3DataType::FMI3Int64(_), FMI3DataType::FMI3Int64(_)) => true,
            (FMI3DataType::FMI3UInt8(_), FMI3DataType::FMI3UInt8(_)) => true,
            (FMI3DataType::FMI3UInt16(_), FMI3DataType::FMI3UInt16(_)) => true,
            (FMI3DataType::FMI3UInt32(_), FMI3DataType::FMI3UInt32(_)) => true,
            (FMI3DataType::FMI3UInt64(_), FMI3DataType::FMI3UInt64(_)) => true,
            (FMI3DataType::FMI3Binary(_), FMI3DataType::FMI3Binary(_)) => true,
            (FMI3DataType::FMI3String(_), FMI3DataType::FMI3String(_)) => true,
            (FMI3DataType::FMI3Clock(_), FMI3DataType::FMI3Clock(_)) => true,
            _ => false,
        }
    }
}

/// Represents the different states an FMU can be in according to the FMI 3.0 standard.
/// The enum does not itself enforce any state transitions; that is left to the FMU implementation.
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum FMUState {
    Instantiated,
    ConfigurationMode,
    InitializationMode,
    ContinuousTimeMode,
    ReconfigurationMode,
    EventMode,
    Terminated,
}

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

#[allow(dead_code)]
enum Variability {
    Constant,
    Fixed,
    Tunable,
    Discrete,
    Continuous,
}

#[allow(dead_code)]
enum Initial {
    Exact,
    Approx,
    Calculated,
}

/// Represents the definition of a variable in the FMU, including its causality, value reference, and data type.
struct VariableDefinition {
    name: String,
    causality: Causality,
    variability: Variability,
    initial: Option<Initial>,
    data_type: FMI3DataType,
    dimension: Vec<usize>,
    get: Box<dyn Fn(&model::SpringMassDamper) -> FMI3DataType>,
    set: Box<dyn Fn(&mut model::SpringMassDamper, &FMI3DataType)>,
}

/// Represents the FMU instance for the spring mass damper model, encapsulating its state and variable definitions.
pub struct FMU {
    model: Box<model::SpringMassDamper>,
    pub state: FMUState,
    variables: Vec<VariableDefinition>,
}

impl FMU {
    pub fn new(mass: f64, damping: f64, stiffness: f64) -> Self {
        Self {
            model: Box::new(model::SpringMassDamper::new(mass, damping, stiffness)),
            state: FMUState::Instantiated,
            variables: vec![
                VariableDefinition {
                    name: "time".to_string(),
                    causality: Causality::Independent,
                    variability: Variability::Continuous,
                    initial: None,
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    dimension: vec![1],
                    get: Box::new(|model| FMI3DataType::FMI3Float64(vec![model.time])),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if let Some(&time) = v.get(0) {
                                model.time = time;
                            }
                        }
                    }),
                },
                VariableDefinition {
                    name: "state".to_string(),
                    causality: Causality::Local,
                    variability: Variability::Continuous,
                    initial: Some(Initial::Exact),
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    dimension: vec![2],
                    get: Box::new(|model| FMI3DataType::FMI3Float64(vec![model.x[0], model.x[1]])),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if v.len() == 2 {
                                model.x[0] = v[0];
                                model.x[1] = v[1];
                            }
                        }
                    }),
                },
                VariableDefinition {
                    name: "state_der".to_string(),
                    causality: Causality::Local,
                    variability: Variability::Continuous,
                    initial: Some(Initial::Calculated),
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    dimension: vec![2],
                    get: Box::new(|model| {
                        FMI3DataType::FMI3Float64(vec![model.dx[0], model.dx[1]])
                    }),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if v.len() == 2 {
                                model.dx[0] = v[0];
                                model.dx[1] = v[1];
                            }
                        }
                    }),
                },
                VariableDefinition {
                    name: "input_force".to_string(),
                    causality: Causality::Input,
                    variability: Variability::Continuous,
                    initial: Some(Initial::Exact),
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    dimension: vec![1],
                    get: Box::new(|model| FMI3DataType::FMI3Float64(vec![model.force])),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if v.len() == 1 {
                                model.force = v[0];
                            }
                        }
                    }),
                },
                VariableDefinition {
                    name: "mass".to_string(),
                    causality: Causality::Parameter,
                    variability: Variability::Fixed,
                    initial: Some(Initial::Exact),
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    get: Box::new(|model| FMI3DataType::FMI3Float64(vec![model.mass])),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if v.len() == 1 {
                                model.mass = v[0];
                            }
                        }
                    }),
                    dimension: vec![1],
                },
                VariableDefinition {
                    name: "damping_constant".to_string(),
                    causality: Causality::Parameter,
                    variability: Variability::Fixed,
                    initial: Some(Initial::Exact),
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    get: Box::new(|model| FMI3DataType::FMI3Float64(vec![model.c])),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if v.len() == 1 {
                                model.c = v[0];
                            }
                        }
                    }),
                    dimension: vec![1],
                },
                VariableDefinition {
                    name: "spring_constant".to_string(),
                    causality: Causality::Parameter,
                    variability: Variability::Fixed,
                    initial: Some(Initial::Exact),
                    data_type: FMI3DataType::FMI3Float64(vec![]),
                    get: Box::new(|model| FMI3DataType::FMI3Float64(vec![model.k])),
                    set: Box::new(|model, value| {
                        if let FMI3DataType::FMI3Float64(v) = value {
                            if v.len() == 1 {
                                model.k = v[0];
                            }
                        }
                    }),
                    dimension: vec![1],
                },
            ],
        }
    }

    // #region Initialization Mode
    pub fn enter_initialization_mode(&mut self) {
        self.state = FMUState::InitializationMode;
    }

    pub fn exit_initialization_mode(&mut self) {
        self.state = FMUState::ContinuousTimeMode;
    }
    // #endregion Initialization Mode

    // #region getters and setters

    /// Fails if the variable has no start value (initial = Exact/Approx) in Instantiated state.
    pub fn get_variable(&self, vr: usize) -> Result<FMI3DataType, ()> {
        let var = self.variables.get(vr).ok_or(())?;

        // If we're in the Instantiated state, only variables with a defined start value are readable.
        // If we're in the Terminated state, no variables are readable.
        if self.state == FMUState::Instantiated
            && !matches!(var.initial, Some(Initial::Exact) | Some(Initial::Approx))
            || self.state == FMUState::Terminated
        {
            return Err(());
        }

        return Ok((var.get)(&self.model));
    }

    /// Fails if the variable is not settable in the current FMU state or the type does not match.
    pub fn set_variable(&mut self, vr: usize, value: FMI3DataType) -> Result<(), ()> {
        let allowed = {
            let var = self.variables.get(vr).ok_or(())?;

            if std::mem::discriminant(&var.data_type) != std::mem::discriminant(&value) {
                return Err(());
            }

            match self.state {
                // variability ≠ Constant AND (initial = Exact OR Approx)
                FMUState::Instantiated => {
                    !matches!(var.variability, Variability::Constant)
                        && matches!(var.initial, Some(Initial::Exact) | Some(Initial::Approx))
                }
                // variability ≠ Constant AND initial = Exact
                FMUState::InitializationMode => {
                    !matches!(var.variability, Variability::Constant)
                        && matches!(var.initial, Some(Initial::Exact))
                }
                // causality = Input OR (causality = Parameter AND variability = Tunable)
                FMUState::EventMode => {
                    matches!(var.causality, Causality::Input)
                        || (matches!(var.causality, Causality::Parameter)
                            && matches!(var.variability, Variability::Tunable))
                }
                // causality = Input AND variability = Continuous
                FMUState::ContinuousTimeMode => {
                    matches!(var.causality, Causality::Input)
                        && matches!(var.variability, Variability::Continuous)
                }
                // causality = StructuralParameter AND (variability = Fixed OR Tunable)
                FMUState::ConfigurationMode => {
                    matches!(var.causality, Causality::StructuralParameter)
                        && matches!(var.variability, Variability::Fixed | Variability::Tunable)
                }
                // causality = StructuralParameter AND variability = Tunable only
                FMUState::ReconfigurationMode => {
                    matches!(var.causality, Causality::StructuralParameter)
                        && matches!(var.variability, Variability::Tunable)
                }
                FMUState::Terminated => false,
            }
        };

        if !allowed {
            return Err(());
        }

        // self.variables and self.model are distinct fields; split borrow is safe
        let model = &mut *self.model;
        self.variables[vr].set.as_ref()(model, &value);
        Ok(())
    }
    // #endregion getters and setters
}
