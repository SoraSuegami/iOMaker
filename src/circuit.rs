use crate::gate::*;
use num_traits::{One, Zero};
use std::{collections::HashMap, fmt, hash::Hash, usize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BooleanCircuitError {
    #[error("The output of the wire id {0} is unknown.")]
    UnknownOutput(WireId),
    #[error("The gate of id {0} is unknown.")]
    UnknownGate(GateId),
    #[error("The evaled bit of the gate id {0} is unknown.")]
    UnknownEvaledBit(GateId),
    #[error("The number of given inputs is {0}, but the input length is {1}.")]
    InvalidInputLen(usize, usize),
    #[error("The evaled bit of the gate id {0} is constrained to {1} in the const gate, but its value is {2}")]
    InvalidBitOfConstGate(GateId, bool, bool),
    #[error("The gate of the gate id {0} is not supported.")]
    NotSupportedGate(GateId),
}

pub trait BooleanCircuit<G: Gate> {
    fn input_len(&self) -> usize;
    fn output_len(&self) -> usize;
    fn depth_whole(&self) -> usize;
    fn depth_of_output(&self, output_wire_id: &WireId) -> Result<usize, BooleanCircuitError>;
    fn input(&mut self) -> Result<GateId, BooleanCircuitError>;
    fn output(&mut self, gate_id: GateId) -> Result<WireId, BooleanCircuitError>;
    fn constant(&mut self, gate_id: &GateId, bit: bool) -> Result<GateId, BooleanCircuitError>;
    fn not(&mut self, gate_id: &GateId) -> Result<GateId, BooleanCircuitError>;
    fn and(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError>;
    fn or(&mut self, gate_l_id: &GateId, gate_r_id: &GateId)
        -> Result<GateId, BooleanCircuitError>;
    fn xor(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError>;
    fn nand(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError>;
    fn eval_output(
        &self,
        inputs: &[bool],
        output_wire_id: &WireId,
        evaled_map: Option<&mut HashMap<GateId, bool>>,
    ) -> Result<bool, BooleanCircuitError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NandBasedCircuit {
    pub output_map: HashMap<WireId, GateId>,
    pub gate_map: HashMap<GateId, BooleanGate>,
    pub input_len: usize,
    pub output_len: usize,
    pub num_gate: usize,
    pub num_nand: usize,
    pub num_const: usize,
}

impl BooleanCircuit<BooleanGate> for NandBasedCircuit {
    fn input_len(&self) -> usize {
        self.input_len
    }

    fn output_len(&self) -> usize {
        self.output_len
    }

    fn depth_whole(&self) -> usize {
        let mut max: usize = 0;
        for i in 0..self.output_len {
            let wire_id = WireId(i as u64);
            let depth = self.depth_of_output(&wire_id).expect(&format!(
                "Fail to get the depth of the {}th output circuit",
                i
            ));
            if depth > max {
                max = depth;
            }
        }
        max
    }

    fn depth_of_output(&self, output_wire_id: &WireId) -> Result<usize, BooleanCircuitError> {
        let output_id = self.output_to_gate_id(output_wire_id)?;
        let gate: &BooleanGate = self.get_gate(&output_id)?;
        Ok(gate.depth())
    }

    fn input(&mut self) -> Result<GateId, BooleanCircuitError> {
        let new_wire_id = WireId(self.input_len as u64);
        let input_gate = InputGate {
            wire_id: new_wire_id,
            value: None,
        };
        self.input_len += 1;
        let new_gate_id = GateId(self.num_gate as u64);
        self.gate_map
            .insert(new_gate_id, BooleanGate::Input(input_gate));
        self.num_gate += 1;
        Ok(new_gate_id)
    }

    fn output(&mut self, gate_id: GateId) -> Result<WireId, BooleanCircuitError> {
        if self.gate_map.get(&gate_id).is_none() {
            return Err(BooleanCircuitError::UnknownGate(gate_id));
        }
        let new_wire_id = WireId(self.output_len as u64);
        self.output_map.insert(new_wire_id, gate_id);
        self.output_len += 1;
        Ok(new_wire_id)
    }

    fn constant(&mut self, gate_id: &GateId, bit: bool) -> Result<GateId, BooleanCircuitError> {
        let input_gate = self.get_gate(gate_id)?;
        let constant_gate = ConstGate {
            id: *gate_id,
            value: bit,
            depth: input_gate.depth() + 1,
        };
        let new_gate_id = GateId(self.num_gate as u64);
        self.gate_map
            .insert(new_gate_id, BooleanGate::Const(constant_gate));
        self.num_gate += 1;
        self.num_const += 1;
        Ok(new_gate_id)
    }

    fn not(&mut self, gate_id: &GateId) -> Result<GateId, BooleanCircuitError> {
        let not = self.nand(gate_id, gate_id)?;
        Ok(not)
    }

    fn and(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError> {
        let nand = self.nand(gate_l_id, gate_r_id)?;
        let and = self.not(&nand)?;
        Ok(and)
    }

    fn or(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError> {
        let not_l = self.not(gate_l_id)?;
        let not_r = self.not(gate_r_id)?;
        let or = self.nand(&not_l, &not_r)?;
        Ok(or)
    }

    fn xor(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError> {
        let not_l = self.not(gate_l_id)?;
        let not_r = self.not(gate_r_id)?;
        let and1 = self.and(gate_l_id, &not_r)?;
        let and2 = self.and(&not_l, gate_r_id)?;
        let xor = self.or(&and1, &and2)?;
        Ok(xor)
    }

    fn nand(
        &mut self,
        gate_l_id: &GateId,
        gate_r_id: &GateId,
    ) -> Result<GateId, BooleanCircuitError> {
        let input_l_gate = self.get_gate(gate_l_id)?;
        let input_r_gate = self.get_gate(gate_r_id)?;
        let new_depth = if input_l_gate.depth() >= input_r_gate.depth() {
            input_l_gate.depth() + 1
        } else {
            input_r_gate.depth() + 1
        };
        let nand_gate = NandGate {
            left_id: *gate_l_id,
            right_id: *gate_r_id,
            depth: new_depth,
        };
        let new_gate_id = GateId(self.num_gate as u64);
        self.gate_map
            .insert(new_gate_id, BooleanGate::Nand(nand_gate));
        self.num_gate += 1;
        self.num_nand += 1;
        Ok(new_gate_id)
    }

    fn eval_output(
        &self,
        inputs: &[bool],
        output_wire_id: &WireId,
        evaled_map: Option<&mut HashMap<GateId, bool>>,
    ) -> Result<bool, BooleanCircuitError> {
        if inputs.len() != self.input_len {
            return Err(BooleanCircuitError::InvalidInputLen(
                inputs.len(),
                self.input_len,
            ));
        }
        if output_wire_id.0 >= (self.output_len() as u64) {
            return Err(BooleanCircuitError::UnknownOutput(*output_wire_id));
        }
        let mut new_evaled_map = HashMap::<GateId, bool>::new();
        let mut evaled_map = match evaled_map {
            Some(m) => m,
            None => &mut new_evaled_map,
        };
        let output_gate_id = self.output_to_gate_id(output_wire_id)?;
        let output_gate = self.get_gate(&output_gate_id)?;
        self.eval_single_gate(inputs, output_gate_id, output_gate, &mut evaled_map)?;
        let output_bit = evaled_map
            .get(output_gate_id)
            .ok_or(BooleanCircuitError::UnknownEvaledBit(*output_gate_id))?;
        Ok(*output_bit)
    }
}

impl NandBasedCircuit {
    pub fn new() -> Self {
        Self {
            output_map: HashMap::new(),
            gate_map: HashMap::new(),
            input_len: 0,
            output_len: 0,
            num_gate: 0,
            num_nand: 0,
            num_const: 0,
        }
    }

    pub fn output_to_gate_id(&self, wire_id: &WireId) -> Result<&GateId, BooleanCircuitError> {
        self.output_map
            .get(wire_id)
            .ok_or(BooleanCircuitError::UnknownOutput(*wire_id))
    }

    pub fn get_gate(&self, gate_id: &GateId) -> Result<&BooleanGate, BooleanCircuitError> {
        self.gate_map
            .get(gate_id)
            .ok_or(BooleanCircuitError::UnknownGate(*gate_id))
    }

    fn eval_single_gate(
        &self,
        inputs: &[bool],
        gate_id: &GateId,
        gate: &BooleanGate,
        evaled_map: &mut HashMap<GateId, bool>,
    ) -> Result<(), BooleanCircuitError> {
        match gate {
            BooleanGate::Input(gate) => {
                let input_bit: bool = inputs[gate.wire_id.0 as usize];
                evaled_map.insert(*gate_id, input_bit);
                Ok(())
            }
            BooleanGate::Nand(gate) => {
                if evaled_map.get(&gate.left_id).is_none() {
                    let input_gate = self.get_gate(&gate.left_id)?;
                    self.eval_single_gate(inputs, &gate.left_id, &input_gate, evaled_map)?;
                }
                if evaled_map.get(&gate.right_id).is_none() {
                    let input_gate = self.get_gate(&gate.right_id)?;
                    self.eval_single_gate(inputs, &gate.right_id, &input_gate, evaled_map)?;
                }
                let input_bit_l = evaled_map
                    .get(&gate.left_id)
                    .ok_or(BooleanCircuitError::UnknownEvaledBit(gate.left_id))?;
                let input_bit_r = evaled_map
                    .get(&gate.right_id)
                    .ok_or(BooleanCircuitError::UnknownEvaledBit(gate.right_id))?;
                let output_bit = !(*input_bit_l && *input_bit_r);
                evaled_map.insert(*gate_id, output_bit);
                Ok(())
            }
            BooleanGate::Const(gate) => {
                if evaled_map.get(&gate.id).is_none() {
                    let input_gate = self.get_gate(&gate.id)?;
                    self.eval_single_gate(inputs, &gate.id, &input_gate, evaled_map)?;
                }
                let input_bit = *evaled_map
                    .get(&gate.id)
                    .ok_or(BooleanCircuitError::UnknownEvaledBit(gate.id))?;
                if input_bit != gate.value {
                    return Err(BooleanCircuitError::InvalidBitOfConstGate(
                        *gate_id, gate.value, input_bit,
                    ));
                }
                evaled_map.insert(*gate_id, input_bit);
                Ok(())
            }
            _ => Err(BooleanCircuitError::NotSupportedGate(*gate_id)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input1_output1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id = circuit.input().unwrap();
        circuit.output(input_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 1);
        assert_eq!(circuit.output_len(), 1);

        let inputs = vec![true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);
    }

    #[test]
    fn input1_not_ouput1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id = circuit.input().unwrap();
        let not_gate_id = circuit.not(&input_gate_id).unwrap();
        circuit.output(not_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 1);
        assert_eq!(circuit.output_len(), 1);
        assert_eq!(circuit.depth_whole(), 1);

        let inputs = vec![true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);

        let inputs = vec![false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);
    }

    #[test]
    fn input2_and_output1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id1 = circuit.input().unwrap();
        let input_gate_id2 = circuit.input().unwrap();
        let and_gate_id = circuit.and(&input_gate_id1, &input_gate_id2).unwrap();
        circuit.output(and_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 2);
        assert_eq!(circuit.output_len(), 1);

        let inputs = vec![true, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![true, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);

        let inputs = vec![false, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);

        let inputs = vec![false, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);
    }

    #[test]
    fn input2_or_output1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id1 = circuit.input().unwrap();
        let input_gate_id2 = circuit.input().unwrap();
        let or_gate_id = circuit.or(&input_gate_id1, &input_gate_id2).unwrap();
        circuit.output(or_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 2);
        assert_eq!(circuit.output_len(), 1);

        let inputs = vec![true, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![true, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);
    }

    #[test]
    fn input1_const_output1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id = circuit.input().unwrap();
        let const_gate_id = circuit.constant(&input_gate_id, true).unwrap();
        circuit.output(const_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 1);
        assert_eq!(circuit.output_len(), 1);

        let inputs = vec![true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false];
        let output = circuit.eval_output(&inputs, &WireId(0), None);
        assert_eq!(output.is_err(), true);
    }

    #[test]
    fn input_xor_output1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id1 = circuit.input().unwrap();
        let input_gate_id2 = circuit.input().unwrap();
        let or_gate_id = circuit.xor(&input_gate_id1, &input_gate_id2).unwrap();
        circuit.output(or_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 2);
        assert_eq!(circuit.output_len(), 1);

        let inputs = vec![true, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);

        let inputs = vec![true, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);
    }

    #[test]
    fn input3_and_or_output1() {
        let mut circuit = NandBasedCircuit::new();
        let input_gate_id1 = circuit.input().unwrap();
        let input_gate_id2 = circuit.input().unwrap();
        let input_gate_id3 = circuit.input().unwrap();
        let and_gate_id = circuit.and(&input_gate_id1, &input_gate_id2).unwrap();
        let or_gate_id = circuit.or(&and_gate_id, &input_gate_id3).unwrap();
        circuit.output(or_gate_id).unwrap();
        assert_eq!(circuit.input_len(), 3);
        assert_eq!(circuit.output_len(), 1);

        let inputs = vec![true, true, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![true, true, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![true, false, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![true, false, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);

        let inputs = vec![false, true, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false, true, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);

        let inputs = vec![false, false, true];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, true);

        let inputs = vec![false, false, false];
        let output = circuit.eval_output(&inputs, &WireId(0), None).unwrap();
        assert_eq!(output, false);
    }
}
