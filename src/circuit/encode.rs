use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedBoolGate {
    pub gate_id: GateId,
    pub gate_type_id: u64,
    pub inputs: Vec<GateId>,
}

impl EncodedBoolGate {
    pub fn from_gate(gate: &BoolGate) -> Self {
        Self {
            gate_id: gate.gate_id,
            gate_type_id: gate.gate_type.gate_type_id(),
            inputs: gate.inputs.iter().map(|gate| gate.gate_id).collect(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).expect("The bytes of EncodedBoolGate is invalid.")
    }
}
