use self::bool_gate_type::{
    AndGateType, InputGateType, NotGateType, OrGateType, OutputGateType, XorGateType,
};
use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::{clone_trait_object, DynClone};
use itertools::*;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::{
    any::Any,
    collections::HashMap,
    error::Error,
    ops::{Add, AddAssign, Sub, SubAssign},
    sync::Arc,
};
pub mod bool_gate_type;
pub mod builder;
pub mod encode;
pub mod evaluator;
use bincode;
use bool_gate_type::*;
use builder::*;
use encode::*;
use evaluator::*;

pub trait BoolGateType: std::fmt::Debug + DynClone + Downcast + Send + Sync {
    fn eval(&self, input: &[bool]) -> Vec<bool>;
    fn num_input(&self) -> usize;
    fn num_output(&self) -> usize;
    fn gate_type_id(&self) -> u64;
}

clone_trait_object!(BoolGateType);
impl_downcast!(BoolGateType);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GateId {
    pub id: usize,
}

impl Zero for GateId {
    fn zero() -> Self {
        Self { id: 0 }
    }

    fn is_zero(&self) -> bool {
        self.id == 0
    }
}

impl Add for GateId {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            id: self.id + rhs.id,
        }
    }
}

impl AddAssign for GateId {
    fn add_assign(&mut self, rhs: Self) {
        self.id += rhs.id;
    }
}

impl Sub for GateId {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            id: self.id - rhs.id,
        }
    }
}

impl SubAssign for GateId {
    fn sub_assign(&mut self, rhs: Self) {
        self.id -= rhs.id;
    }
}

impl GateId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        (self.id as u32).to_le_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            id: u32::from_le_bytes(bytes.try_into().unwrap()) as usize,
        }
    }

    pub fn to_bits(&self) -> Vec<bool> {
        self.to_bytes()
            .into_iter()
            .flat_map(|byte| (0..8).map(move |idx| (byte >> idx) & 1 == 1))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct BoolGate {
    pub gate_id: GateId,
    pub gate_type: Box<dyn BoolGateType>,
    pub inputs: Vec<Arc<BoolGate>>,
}

impl BoolGate {
    pub fn new(
        gate_id: GateId,
        gate_type: Box<dyn BoolGateType>,
        inputs: Vec<Arc<BoolGate>>,
    ) -> Self {
        Self {
            gate_id,
            gate_type,
            inputs,
        }
    }

    pub fn get_input(&self, idx: usize) -> &BoolGate {
        &self.inputs[idx]
    }
}

#[derive(Debug, Clone)]
pub struct BoolCircuit {
    pub num_input: usize,
    pub num_gates: usize,
    pub output_gates: Vec<Arc<BoolGate>>,
}

impl BoolCircuit {
    pub fn new(output_gates: Vec<Arc<BoolGate>>, num_input: usize) -> Self {
        let num_gates = output_gates
            .iter()
            .map(|gate| gate.gate_id.id)
            .max()
            .unwrap_or(0)
            + 1;
        Self {
            output_gates,
            num_gates,
            num_input,
        }
    }

    pub fn num_input(&self) -> usize {
        self.num_input
    }

    pub fn num_output(&self) -> usize {
        self.output_gates.len()
    }

    pub fn num_gates(&self) -> usize {
        self.num_gates
    }

    // pub fn circuit_digest<H: NativeHasher>(&self) -> [bool; DIGEST_BIT_SIZE] {
    //     let mut gates = sort_gates(&self).into_iter().collect_vec();
    //     gates.sort_by_key(|(gate_id, _)| *gate_id);
    //     let sorted_gates = gates.into_iter().map(|(_, gate)| gate).collect_vec();

    //     let mut circuit_digest = vec![];
    //     for gate in sorted_gates.into_iter() {
    //         let mut digest_input = circuit_digest.clone();
    //         let gate_bytes = gate.to_bytes();
    //         digest_input.append(&mut bytes2bits_le(&gate_bytes, gate_bytes.len()));
    //         let mut hasher = H::new();
    //         circuit_digest = hasher.digest(&digest_input);
    //     }
    //     circuit_digest.try_into().unwrap()
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_not_gate() {
        let mut circuit_builder = BoolCircuitBuilder::new();
        let input = circuit_builder.input(1);
        let not = circuit_builder.not(&input[0]);
        let circuit = Arc::new(circuit_builder.output(vec![not]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true]);
        assert_eq!(output[0], false);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false]);
        assert_eq!(output[0], true);
    }

    #[test]
    fn test_xor_gate() {
        let mut circuit_builder = BoolCircuitBuilder::new();
        let input = circuit_builder.input(2);
        let xor = circuit_builder.xor(&input[0], &input[1]);
        let circuit = Arc::new(circuit_builder.output(vec![xor]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false, false]);
        assert_eq!(output[0], false);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false, true]);
        assert_eq!(output[0], true);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true, false]);
        assert_eq!(output[0], true);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true, true]);
        assert_eq!(output[0], false);
    }

    #[test]
    fn test_and_gate() {
        let mut circuit_builder = BoolCircuitBuilder::new();
        let input = circuit_builder.input(2);
        let and = circuit_builder.and(&input[0], &input[1]);
        let circuit = Arc::new(circuit_builder.output(vec![and]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false, false]);
        assert_eq!(output[0], false);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false, true]);
        assert_eq!(output[0], false);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true, false]);
        assert_eq!(output[0], false);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true, true]);
        assert_eq!(output[0], true);
    }

    #[test]
    fn test_or_gate() {
        let mut circuit_builder = BoolCircuitBuilder::new();
        let input = circuit_builder.input(2);
        let or = circuit_builder.or(&input[0], &input[1]);
        let circuit = Arc::new(circuit_builder.output(vec![or]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false, false]);
        assert_eq!(output[0], false);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[false, true]);
        assert_eq!(output[0], true);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true, false]);
        assert_eq!(output[0], true);

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let output = evaluator.eval(&[true, true]);
        assert_eq!(output[0], true);
    }

    #[test]
    fn test_multi_layer() {
        let mut circuit_builder = BoolCircuitBuilder::new();
        let input = circuit_builder.input(3);
        let or = circuit_builder.or(&input[0], &input[1]);
        let and = circuit_builder.and(&input[1], &input[2]);
        let xor = circuit_builder.xor(&or, &and);
        let not = circuit_builder.not(&xor);
        let circuit = Arc::new(circuit_builder.output(vec![not]));

        let expected_output = |a: bool, b: bool, c: bool| {
            let or = a || b;
            let and = b && c;
            let xor = or ^ and;
            !xor
        };

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, false, false];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, false, true];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, true, false];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, true, true];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, false, false];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, false, true];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, true, false];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, true, true];
        let output = evaluator.eval(&input);
        assert_eq!(output[0], expected_output(input[0], input[1], input[2]));
    }

    #[test]
    fn test_multi_bit_output() {
        let mut circuit_builder = BoolCircuitBuilder::new();
        let input = circuit_builder.input(3);
        let or = circuit_builder.or(&input[0], &input[1]);
        let and = circuit_builder.and(&input[1], &input[2]);
        let xor = circuit_builder.xor(&or, &and);
        let not = circuit_builder.not(&xor);
        let circuit = Arc::new(circuit_builder.output(vec![xor, not]));

        let expected_output = |a: bool, b: bool, c: bool| {
            let or = a || b;
            let and = b && c;
            let xor = or ^ and;
            vec![xor, !xor]
        };

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, false, false];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, false, true];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, true, false];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![false, true, true];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, false, false];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, false, true];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, true, false];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));

        let mut evaluator = PlainBoolCircuitEvaluator::new(circuit.clone());
        let input = vec![true, true, true];
        let output = evaluator.eval(&input);
        assert_eq!(output, expected_output(input[0], input[1], input[2]));
    }
}
