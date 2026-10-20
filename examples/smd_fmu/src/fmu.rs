//! This module implements the FMU wrapper for the spring mass damper model,
//! with all the necessary functionality for the FMI 3.0 standard.

use crate::model;

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
    dimension: Vec<usize>,
    get: Box<dyn Fn(&model::SpringMassDamper) -> Vec<f64>>,
    set: Box<dyn Fn(&mut model::SpringMassDamper, &[f64])>,
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
                    dimension: vec![1],
                    get: Box::new(|model| vec![model.time]),
                    set: Box::new(|model, value| model.time = value[0]),
                },
                VariableDefinition {
                    name: "state".to_string(),
                    causality: Causality::Local,
                    variability: Variability::Continuous,
                    initial: Some(Initial::Exact),
                    dimension: vec![2],
                    get: Box::new(|model| model.x.to_vec()),
                    set: Box::new(|model, value| model.x.copy_from_slice(value)),
                },
                VariableDefinition {
                    name: "state_der".to_string(),
                    causality: Causality::Local,
                    variability: Variability::Continuous,
                    initial: Some(Initial::Calculated),
                    dimension: vec![2],
                    get: Box::new(|model| model.dx.to_vec()),
                    set: Box::new(|model, value| model.dx.copy_from_slice(value)),
                },
                VariableDefinition {
                    name: "input_force".to_string(),
                    causality: Causality::Input,
                    variability: Variability::Continuous,
                    initial: Some(Initial::Exact),
                    dimension: vec![1],
                    get: Box::new(|model| vec![model.force]),
                    set: Box::new(|model, value| model.force = value[0]),
                },
                VariableDefinition {
                    name: "mass".to_string(),
                    causality: Causality::Parameter,
                    variability: Variability::Fixed,
                    initial: Some(Initial::Exact),
                    dimension: vec![1],
                    get: Box::new(|model| vec![model.mass]),
                    set: Box::new(|model, value| model.mass = value[0]),
                },
                VariableDefinition {
                    name: "damping_constant".to_string(),
                    causality: Causality::Parameter,
                    variability: Variability::Fixed,
                    initial: Some(Initial::Exact),
                    dimension: vec![1],
                    get: Box::new(|model| vec![model.c]),
                    set: Box::new(|model, value| model.c = value[0]),
                },
                VariableDefinition {
                    name: "spring_constant".to_string(),
                    causality: Causality::Parameter,
                    variability: Variability::Fixed,
                    initial: Some(Initial::Exact),
                    dimension: vec![1],
                    get: Box::new(|model| vec![model.k]),
                    set: Box::new(|model, value| model.k = value[0]),
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

    /// Function to retrieve the value of a variable by its value reference.
    /// In this model, all variables are `f64`s, so only the GetFloat64 function calls this.
    /// All other Get functions are not used in this model.
    pub fn get_variable(
        &self,
        value_references: &[u32],
        n_value_references: usize,
        values: &mut [f64],
        n_values: usize,
    ) -> bool {
        let mut i_val: usize = 0;
        let model: &model::SpringMassDamper = &*self.model;

        for i in 0..n_value_references {
            let this_vr: usize = value_references[i] as usize;
            // Fail if the value reference is out of range
            if this_vr >= self.variables.len() {
                return false;
            }
            let this_var: &VariableDefinition = &self.variables[this_vr];
            let n_elems: usize = this_var.dimension.iter().product();
            let model_vals: Vec<f64> = (this_var.get.as_ref())(model);
            for j in 0..n_elems {
                // Fail if the number of values provided is less than the number of elements expected
                if i_val >= n_values {
                    return false;
                }
                values[i_val] = model_vals[j];
                i_val += 1;
            }
        }
        return true;
    }

    /// Fails if the variable is not settable in the current FMU state or the type does not match.
    pub fn set_variable(
        &mut self,
        value_references: &[u32],
        n_value_references: usize,
        values: &[f64],
        n_values: usize,
    ) -> bool {
        let mut i_val: usize = 0;
        let model: &mut model::SpringMassDamper = &mut *self.model;

        for i in 0..n_value_references {
            let this_vr: usize = value_references[i] as usize;
            // Fail if the value reference is out of range
            if this_vr >= self.variables.len() {
                return false;
            }
            let this_var: &VariableDefinition = &self.variables[this_vr];
            // Fail if setting this variable is not allowed in the current FMU state
            if !is_set_allowed(
                &self.state,
                &this_var.causality,
                &this_var.variability,
                &this_var.initial,
            ) {
                return false;
            }
            let n_elems: usize = this_var.dimension.iter().product();
            for _ in 0..n_elems {
                // Fail if the number of values provided is less than the number of elements expected
                if i_val >= n_values {
                    return false;
                }
                (this_var.set.as_ref())(model, &values[i_val..i_val + 1]);
                i_val += 1;
            }
        }
        return true;
    }
    // #endregion getters and setters
}

fn is_set_allowed(
    fmu_state: &FMUState,
    causality: &Causality,
    variability: &Variability,
    initial: &Option<Initial>,
) -> bool {
    match fmu_state {
        FMUState::Instantiated => {
            !matches!(variability, Variability::Constant)
                && matches!(initial, Some(Initial::Exact) | Some(Initial::Approx))
        }
        FMUState::InitializationMode => {
            !matches!(variability, Variability::Constant) && matches!(initial, Some(Initial::Exact))
        }
        FMUState::EventMode => {
            matches!(causality, Causality::Input)
                || (matches!(causality, Causality::Parameter)
                    && matches!(variability, Variability::Tunable))
        }
        FMUState::ContinuousTimeMode => {
            matches!(causality, Causality::Input) && matches!(variability, Variability::Continuous)
        }
        FMUState::ConfigurationMode => {
            matches!(causality, Causality::StructuralParameter)
                && matches!(variability, Variability::Fixed | Variability::Tunable)
        }
        FMUState::ReconfigurationMode => {
            matches!(causality, Causality::StructuralParameter)
                && matches!(variability, Variability::Tunable)
        }
        FMUState::Terminated => false,
    }
}
