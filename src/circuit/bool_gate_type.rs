use crate::circuit::{BoolCircuit, BoolGateType};

#[derive(Debug, Clone)]
pub enum BoolGateTypeId {
    Input = 0,
    Output = 1,
    Xor = 2,
    And = 3,
    Or = 4,
    Not = 5,
}

#[derive(Debug, Clone)]
pub struct InputGateType {}

impl BoolGateType for InputGateType {
    /// [WARNING] An evaluator of [`BoolCircuit`] should not call `eval` function when the gate type is [`InputGateType`].
    fn eval(&self, input: &[bool]) -> Vec<bool> {
        unimplemented!("An evaluator of BoolCircuit should not call `eval` function when the gate type is InputGateType")
    }

    fn num_input(&self) -> usize {
        0
    }

    fn num_output(&self) -> usize {
        1
    }

    fn gate_type_id(&self) -> u64 {
        0
    }
}

impl InputGateType {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct OutputGateType {}

impl BoolGateType for OutputGateType {
    /// [WARNING] An evaluator of [`BoolCircuit`] should not call `eval` function when the gate type is [`InputGateType`].
    fn eval(&self, input: &[bool]) -> Vec<bool> {
        vec![input[0]]
    }

    fn num_input(&self) -> usize {
        1
    }

    fn num_output(&self) -> usize {
        1
    }

    fn gate_type_id(&self) -> u64 {
        1
    }
}

impl OutputGateType {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct XorGateType {}

impl BoolGateType for XorGateType {
    fn eval(&self, input: &[bool]) -> Vec<bool> {
        vec![input[0] ^ input[1]]
    }

    fn num_input(&self) -> usize {
        2
    }

    fn num_output(&self) -> usize {
        1
    }

    fn gate_type_id(&self) -> u64 {
        2
    }
}

impl XorGateType {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct AndGateType {}

impl BoolGateType for AndGateType {
    fn eval(&self, input: &[bool]) -> Vec<bool> {
        vec![input[0] & input[1]]
    }

    fn num_input(&self) -> usize {
        2
    }

    fn num_output(&self) -> usize {
        1
    }

    fn gate_type_id(&self) -> u64 {
        3
    }
}

impl AndGateType {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct OrGateType {}

impl BoolGateType for OrGateType {
    fn eval(&self, input: &[bool]) -> Vec<bool> {
        vec![input[0] | input[1]]
    }

    fn num_input(&self) -> usize {
        2
    }

    fn num_output(&self) -> usize {
        1
    }

    fn gate_type_id(&self) -> u64 {
        4
    }
}

impl OrGateType {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct NotGateType {}

impl BoolGateType for NotGateType {
    fn eval(&self, input: &[bool]) -> Vec<bool> {
        vec![!input[0]]
    }

    fn num_input(&self) -> usize {
        1
    }

    fn num_output(&self) -> usize {
        1
    }

    fn gate_type_id(&self) -> u64 {
        5
    }
}

impl NotGateType {
    pub fn new() -> Self {
        Self {}
    }
}
