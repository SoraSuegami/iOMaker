use super::*;

pub struct PlainBoolCircuitEvaluator {
    pub circuit: Arc<BoolCircuit>,
    pub wires: HashMap<GateId, bool>,
}

impl PlainBoolCircuitEvaluator {
    pub fn new(circuit: Arc<BoolCircuit>) -> Self {
        Self {
            circuit,
            wires: HashMap::new(),
        }
    }

    pub fn eval(&mut self, input: &[bool]) -> Vec<bool> {
        debug_assert_eq!(input.len(), self.circuit.num_input);
        self.wires.clear();
        for (idx, input) in input.iter().enumerate() {
            self.wires.insert(GateId::new(idx), *input);
        }
        for idx in 0..self.circuit.num_output() {
            let output_gate = self.circuit.output_gates[idx].clone();
            self.eval_gate(&output_gate);
        }
        self.circuit
            .output_gates
            .iter()
            .map(|gate| self.wires[&gate.gate_id])
            .collect()
    }

    fn eval_gate(&mut self, gate: &BoolGate) {
        let input = gate
            .inputs
            .iter()
            .map(|input_gate| {
                if let Some(wire) = self.wires.get(&input_gate.gate_id) {
                    *wire
                } else {
                    self.eval_gate(input_gate);
                    self.wires[&input_gate.gate_id]
                }
            })
            .collect_vec();
        let output = gate.gate_type.eval(&input);
        debug_assert_eq!(output.len(), gate.gate_type.num_output());
        for idx in 0..output.len() {
            self.wires
                .insert(gate.gate_id + GateId::new(idx), output[idx]);
        }
    }
}
