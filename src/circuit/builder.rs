use super::*;
pub struct BoolCircuitBuilder {
    pub num_input: usize,
    pub num_gate: usize,
}

impl BoolCircuitBuilder {
    pub fn new() -> Self {
        Self {
            num_input: 0,
            num_gate: 0,
        }
    }

    pub fn input(&mut self, num_input: usize) -> Vec<Arc<BoolGate>> {
        debug_assert_eq!(self.num_input, 0);
        let input_gates = (0..num_input)
            .map(|idx| {
                Arc::new(BoolGate::new(
                    GateId::new(idx),
                    Box::new(InputGateType::new()),
                    vec![],
                ))
            })
            .collect_vec();
        self.num_input = num_input;
        self.num_gate += num_input;
        input_gates
    }

    pub fn output(self, output_gates: Vec<Arc<BoolGate>>) -> BoolCircuit {
        let gate_id_offset = GateId::new(self.num_gate);
        let output_gates = output_gates
            .iter()
            .enumerate()
            .map(|(idx, gate)| {
                let gate_id = gate_id_offset + GateId::new(idx);
                Arc::new(BoolGate::new(
                    gate_id,
                    Box::new(OutputGateType::new()),
                    vec![gate.clone()],
                ))
            })
            .collect_vec();
        BoolCircuit::new(output_gates, self.num_input)
    }

    // pub fn register_chip(&mut self, chip: BoolCircuit) -> Arc<BoolCircuit> {
    //     Arc::new(chip)
    // }

    // pub fn call_chip(
    //     &mut self,
    //     inputs: &[Arc<BoolGate>],
    //     chip_ref: &Arc<BoolCircuit>,
    // ) -> Vec<Arc<BoolGate>> {
    //     debug_assert_eq!(inputs.len(), chip_ref.num_input);
    //     let num_output = chip_ref.num_output();
    //     let chip_num_gate = chip_ref.num_gates() - chip_ref.num_input() - num_output;
    //     let gate_id_offset = GateId::new(self.num_gate + chip_num_gate);
    //     self.num_gate += chip_num_gate + num_output;
    //     (0..num_output)
    //         .map(|idx| {
    //             let gate_id = gate_id_offset + GateId::new(idx);
    //             Arc::new(BoolGate::new(
    //                 gate_id,
    //                 Box::new(ChipRefType::new(chip_ref.clone(), idx)),
    //                 inputs.into_iter().map(|gate| gate.clone()).collect_vec(),
    //             ))
    //         })
    //         .collect()
    // }

    pub fn not(&mut self, input: &Arc<BoolGate>) -> Arc<BoolGate> {
        let gate_id = GateId::new(self.num_gate);
        self.num_gate += 1;
        Arc::new(BoolGate::new(
            gate_id,
            Box::new(NotGateType::new()),
            vec![input.clone()],
        ))
    }

    pub fn xor(&mut self, input1: &Arc<BoolGate>, input2: &Arc<BoolGate>) -> Arc<BoolGate> {
        let gate_id = GateId::new(self.num_gate);
        self.num_gate += 1;
        Arc::new(BoolGate::new(
            gate_id,
            Box::new(XorGateType::new()),
            vec![input1.clone(), input2.clone()],
        ))
    }

    pub fn and(&mut self, input1: &Arc<BoolGate>, input2: &Arc<BoolGate>) -> Arc<BoolGate> {
        let gate_id = GateId::new(self.num_gate);
        self.num_gate += 1;
        Arc::new(BoolGate::new(
            gate_id,
            Box::new(AndGateType::new()),
            vec![input1.clone(), input2.clone()],
        ))
    }

    pub fn or(&mut self, input1: &Arc<BoolGate>, input2: &Arc<BoolGate>) -> Arc<BoolGate> {
        let gate_id = GateId::new(self.num_gate);
        self.num_gate += 1;
        Arc::new(BoolGate::new(
            gate_id,
            Box::new(OrGateType::new()),
            vec![input1.clone(), input2.clone()],
        ))
    }
}
