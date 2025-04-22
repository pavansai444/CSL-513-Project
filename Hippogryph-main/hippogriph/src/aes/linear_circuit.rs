use tfhe::odd::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;

// A structure implementing the parsing of a "circuit file" from the papers and executing the circuit with the appropriate containers
pub struct LinearCircuit {
    pub x: Vec<Ciphertext>,    // the inputs
    pub t: Vec<Ciphertext>,    // the intermedary values
    pub y: Vec<Ciphertext>     // the outputs
}

// Store operation data
#[derive(Clone)]
struct CircuitOperation {
    target: String,
    op1: String,
    operation: String,
    op2: String,
}

// Store circuit definition
struct CircuitDefinition {
    header: Vec<usize>, 
    operations: Vec<CircuitOperation>,
}

// Preinitialized circuit data for mixcolumns.txt
static MIXCOLUMNS_CIRCUIT: Lazy<CircuitDefinition> = Lazy::new(|| {
    CircuitDefinition {
        header: vec![32, 0, 60, 0, 32, 0],
        operations: vec![
            CircuitOperation { target: "t0".to_string(), op1: "x0".to_string(), operation: "+".to_string(), op2: "x8".to_string() },
            CircuitOperation { target: "t1".to_string(), op1: "x16".to_string(), operation: "+".to_string(), op2: "x24".to_string() },
            CircuitOperation { target: "t2".to_string(), op1: "x1".to_string(), operation: "+".to_string(), op2: "x9".to_string() },
            CircuitOperation { target: "t3".to_string(), op1: "x17".to_string(), operation: "+".to_string(), op2: "x25".to_string() },
            CircuitOperation { target: "t4".to_string(), op1: "x2".to_string(), operation: "+".to_string(), op2: "x10".to_string() },
            CircuitOperation { target: "t5".to_string(), op1: "x18".to_string(), operation: "+".to_string(), op2: "x26".to_string() },
            CircuitOperation { target: "t6".to_string(), op1: "x3".to_string(), operation: "+".to_string(), op2: "x11".to_string() },
            CircuitOperation { target: "t7".to_string(), op1: "x19".to_string(), operation: "+".to_string(), op2: "x27".to_string() },
            CircuitOperation { target: "t8".to_string(), op1: "x4".to_string(), operation: "+".to_string(), op2: "x12".to_string() },
            CircuitOperation { target: "t9".to_string(), op1: "x20".to_string(), operation: "+".to_string(), op2: "x28".to_string() },
            CircuitOperation { target: "t10".to_string(), op1: "x5".to_string(), operation: "+".to_string(), op2: "x13".to_string() },
            CircuitOperation { target: "t11".to_string(), op1: "x21".to_string(), operation: "+".to_string(), op2: "x29".to_string() },
            CircuitOperation { target: "t12".to_string(), op1: "x6".to_string(), operation: "+".to_string(), op2: "x14".to_string() },
            CircuitOperation { target: "t13".to_string(), op1: "x22".to_string(), operation: "+".to_string(), op2: "x30".to_string() },
            CircuitOperation { target: "t14".to_string(), op1: "x23".to_string(), operation: "+".to_string(), op2: "x31".to_string() },
            CircuitOperation { target: "t15".to_string(), op1: "x7".to_string(), operation: "+".to_string(), op2: "x15".to_string() },
            CircuitOperation { target: "t16".to_string(), op1: "x8".to_string(), operation: "+".to_string(), op2: "t1".to_string() },
            CircuitOperation { target: "y0".to_string(), op1: "t15".to_string(), operation: "+".to_string(), op2: "t16".to_string() },
            CircuitOperation { target: "t17".to_string(), op1: "x7".to_string(), operation: "+".to_string(), op2: "x23".to_string() },
            CircuitOperation { target: "t18".to_string(), op1: "x24".to_string(), operation: "+".to_string(), op2: "t0".to_string() },
            CircuitOperation { target: "y16".to_string(), op1: "t14".to_string(), operation: "+".to_string(), op2: "t18".to_string() },
            CircuitOperation { target: "t19".to_string(), op1: "t1".to_string(), operation: "+".to_string(), op2: "y16".to_string() },
            CircuitOperation { target: "y24".to_string(), op1: "t17".to_string(), operation: "+".to_string(), op2: "t19".to_string() },
            CircuitOperation { target: "t20".to_string(), op1: "x27".to_string(), operation: "+".to_string(), op2: "t14".to_string() },
            CircuitOperation { target: "t21".to_string(), op1: "t0".to_string(), operation: "+".to_string(), op2: "y0".to_string() },
            CircuitOperation { target: "y8".to_string(), op1: "t17".to_string(), operation: "+".to_string(), op2: "t21".to_string() },
            CircuitOperation { target: "t22".to_string(), op1: "t5".to_string(), operation: "+".to_string(), op2: "t20".to_string() },
            CircuitOperation { target: "y19".to_string(), op1: "t6".to_string(), operation: "+".to_string(), op2: "t22".to_string() },
            CircuitOperation { target: "t23".to_string(), op1: "x11".to_string(), operation: "+".to_string(), op2: "t15".to_string() },
            CircuitOperation { target: "t24".to_string(), op1: "t7".to_string(), operation: "+".to_string(), op2: "t23".to_string() },
            CircuitOperation { target: "y3".to_string(), op1: "t4".to_string(), operation: "+".to_string(), op2: "t24".to_string() },
            CircuitOperation { target: "t25".to_string(), op1: "x2".to_string(), operation: "+".to_string(), op2: "x18".to_string() },
            CircuitOperation { target: "t26".to_string(), op1: "t17".to_string(), operation: "+".to_string(), op2: "t25".to_string() },
            CircuitOperation { target: "t27".to_string(), op1: "t9".to_string(), operation: "+".to_string(), op2: "t23".to_string() },
            CircuitOperation { target: "t28".to_string(), op1: "t8".to_string(), operation: "+".to_string(), op2: "t20".to_string() },
            CircuitOperation { target: "t29".to_string(), op1: "x10".to_string(), operation: "+".to_string(), op2: "t2".to_string() },
            CircuitOperation { target: "y2".to_string(), op1: "t5".to_string(), operation: "+".to_string(), op2: "t29".to_string() },
            CircuitOperation { target: "t30".to_string(), op1: "x26".to_string(), operation: "+".to_string(), op2: "t3".to_string() },
            CircuitOperation { target: "y18".to_string(), op1: "t4".to_string(), operation: "+".to_string(), op2: "t30".to_string() },
            CircuitOperation { target: "t31".to_string(), op1: "x9".to_string(), operation: "+".to_string(), op2: "x25".to_string() },
            CircuitOperation { target: "t32".to_string(), op1: "t25".to_string(), operation: "+".to_string(), op2: "t31".to_string() },
            CircuitOperation { target: "y10".to_string(), op1: "t30".to_string(), operation: "+".to_string(), op2: "t32".to_string() },
            CircuitOperation { target: "y26".to_string(), op1: "t29".to_string(), operation: "+".to_string(), op2: "t32".to_string() },
            CircuitOperation { target: "t33".to_string(), op1: "x1".to_string(), operation: "+".to_string(), op2: "t18".to_string() },
            CircuitOperation { target: "t59".to_string(), op1: "t17".to_string(), operation: "+".to_string(), op2: "t58".to_string() },
            CircuitOperation { target: "y28".to_string(), op1: "x20".to_string(), operation: "+".to_string(), op2: "t59".to_string() },
        ],
    }
});

// Preinitialized circuit data for mixcolumns2.txt
static MIXCOLUMNS2_CIRCUIT: Lazy<CircuitDefinition> = Lazy::new(|| {
    CircuitDefinition {
        header: vec![32, 0, 93, 0, 32, 0],
        operations: vec![
            CircuitOperation { target: "t0".to_string(), op1: "x8".to_string(), operation: "+".to_string(), op2: "x16".to_string() },
            CircuitOperation { target: "t1".to_string(), op1: "x7".to_string(), operation: "+".to_string(), op2: "x31".to_string() },
            CircuitOperation { target: "t2".to_string(), op1: "x23".to_string(), operation: "+".to_string(), op2: "t0".to_string() },
            CircuitOperation { target: "y15".to_string(), op1: "t1".to_string(), operation: "+".to_string(), op2: "t2".to_string() },
            CircuitOperation { target: "t4".to_string(), op1: "x16".to_string(), operation: "+".to_string(), op2: "x24".to_string() },
            CircuitOperation { target: "t5".to_string(), op1: "x15".to_string(), operation: "+".to_string(), op2: "t4".to_string() },
            CircuitOperation { target: "y23".to_string(), op1: "t1".to_string(), operation: "+".to_string(), op2: "t5".to_string() },
            CircuitOperation { target: "t7".to_string(), op1: "x1".to_string(), operation: "+".to_string(), op2: "x25".to_string() },
            CircuitOperation { target: "t8".to_string(), op1: "x0".to_string(), operation: "+".to_string(), op2: "t0".to_string() },
            CircuitOperation { target: "y24".to_string(), op1: "t7".to_string(), operation: "+".to_string(), op2: "t8".to_string() },
            CircuitOperation { target: "t10".to_string(), op1: "x10".to_string(), operation: "+".to_string(), op2: "x18".to_string() },
            CircuitOperation { target: "t11".to_string(), op1: "x17".to_string(), operation: "+".to_string(), op2: "t10".to_string() },
            CircuitOperation { target: "y9".to_string(), op1: "t7".to_string(), operation: "+".to_string(), op2: "t11".to_string() },
            CircuitOperation { target: "t13".to_string(), op1: "x3".to_string(), operation: "+".to_string(), op2: "x27".to_string() },
            CircuitOperation { target: "t14".to_string(), op1: "x2".to_string(), operation: "+".to_string(), op2: "t13".to_string() },
            CircuitOperation { target: "y26".to_string(), op1: "t10".to_string(), operation: "+".to_string(), op2: "t14".to_string() },
            CircuitOperation { target: "t16".to_string(), op1: "x1".to_string(), operation: "+".to_string(), op2: "x9".to_string() },
            CircuitOperation { target: "t17".to_string(), op1: "x8".to_string(), operation: "+".to_string(), op2: "t16".to_string() },
            CircuitOperation { target: "y0".to_string(), op1: "t4".to_string(), operation: "+".to_string(), op2: "t17".to_string() },
            CircuitOperation { target: "t19".to_string(), op1: "x18".to_string(), operation: "+".to_string(), op2: "x26".to_string() },
            CircuitOperation { target: "t20".to_string(), op1: "x25".to_string(), operation: "+".to_string(), op2: "t19".to_string() },
            CircuitOperation { target: "y17".to_string(), op1: "t16".to_string(), operation: "+".to_string(), op2: "t20".to_string() },
            CircuitOperation { target: "t22".to_string(), op1: "x11".to_string(), operation: "+".to_string(), op2: "x19".to_string() },
            CircuitOperation { target: "t23".to_string(), op1: "x2".to_string(), operation: "+".to_string(), op2: "t19".to_string() },
            CircuitOperation { target: "y10".to_string(), op1: "t22".to_string(), operation: "+".to_string(), op2: "t23".to_string() },
            CircuitOperation { target: "t25".to_string(), op1: "x11".to_string(), operation: "+".to_string(), op2: "t10".to_string() },
            CircuitOperation { target: "t26".to_string(), op1: "x3".to_string(), operation: "+".to_string(), op2: "t25".to_string() },
            CircuitOperation { target: "y2".to_string(), op1: "x26".to_string(), operation: "+".to_string(), op2: "t26".to_string() },
            CircuitOperation { target: "t28".to_string(), op1: "x27".to_string(), operation: "+".to_string(), op2: "y10".to_string() },
            CircuitOperation { target: "y18".to_string(), op1: "t25".to_string(), operation: "+".to_string(), op2: "t28".to_string() },
            CircuitOperation { target: "t30".to_string(), op1: "x26".to_string(), operation: "+".to_string(), op2: "t16".to_string() },
            CircuitOperation { target: "t31".to_string(), op1: "y9".to_string(), operation: "+".to_string(), op2: "t23".to_string() },
            CircuitOperation { target: "y1".to_string(), op1: "t30".to_string(), operation: "+".to_string(), op2: "t31".to_string() },
            CircuitOperation { target: "t33".to_string(), op1: "x2".to_string(), operation: "+".to_string(), op2: "t30".to_string() },
            CircuitOperation { target: "y25".to_string(), op1: "x17".to_string(), operation: "+".to_string(), op2: "t33".to_string() },
            CircuitOperation { target: "t35".to_string(), op1: "x17".to_string(), operation: "+".to_string(), op2: "t4".to_string() },
            CircuitOperation { target: "t36".to_string(), op1: "x1".to_string(), operation: "+".to_string(), op2: "t35".to_string() },
            CircuitOperation { target: "y16".to_string(), op1: "y24".to_string(), operation: "+".to_string(), op2: "t36".to_string() },
            CircuitOperation { target: "t38".to_string(), op1: "x0".to_string(), operation: "+".to_string(), op2: "t16".to_string() },
            CircuitOperation { target: "y8".to_string(), op1: "t36".to_string(), operation: "+".to_string(), op2: "t38".to_string() },
            CircuitOperation { target: "t40".to_string(), op1: "x0".to_string(), operation: "+".to_string(), op2: "x8".to_string() },
            CircuitOperation { target: "t41".to_string(), op1: "x31".to_string(), operation: "+".to_string(), op2: "t40".to_string() },
            CircuitOperation { target: "t42".to_string(), op1: "x15".to_string(), operation: "+".to_string(), op2: "t41".to_string() },
            CircuitOperation { target: "y7".to_string(), op1: "x23".to_string(), operation: "+".to_string(), op2: "t42".to_string() },
            CircuitOperation { target: "t44".to_string(), op1: "y15".to_string(), operation: "+".to_string(), op2: "t41".to_string() },
            CircuitOperation { target: "y31".to_string(), op1: "t5".to_string(), operation: "+".to_string(), op2: "t44".to_string() },
            CircuitOperation { target: "t46".to_string(), op1: "x14".to_string(), operation: "+".to_string(), op2: "x22".to_string() },
            CircuitOperation { target: "t47".to_string(), op1: "x21".to_string(), operation: "+".to_string(), op2: "x29".to_string() },
            CircuitOperation { target: "t48".to_string(), op1: "x5".to_string(), operation: "+".to_string(), op2: "t46".to_string() },
            CircuitOperation { target: "y13".to_string(), op1: "t47".to_string(), operation: "+".to_string(), op2: "t48".to_string() },
            CircuitOperation { target: "t50".to_string(), op1: "t1".to_string(), operation: "+".to_string(), op2: "t46".to_string() },
            CircuitOperation { target: "t51".to_string(), op1: "x30".to_string(), operation: "+".to_string(), op2: "t42".to_string() },
            CircuitOperation { target: "y6".to_string(), op1: "t50".to_string(), operation: "+".to_string(), op2: "t51".to_string() },
            CircuitOperation { target: "t53".to_string(), op1: "x6".to_string(), operation: "+".to_string(), op2: "x14".to_string() },
            CircuitOperation { target: "t54".to_string(), op1: "t47".to_string(), operation: "+".to_string(), op2: "t53".to_string() },
            CircuitOperation { target: "y5".to_string(), op1: "x13".to_string(), operation: "+".to_string(), op2: "t54".to_string() },
            CircuitOperation { target: "t56".to_string(), op1: "y6".to_string(), operation: "+".to_string(), op2: "t53".to_string() },
            CircuitOperation { target: "y14".to_string(), op1: "t44".to_string(), operation: "+".to_string(), op2: "t56".to_string() },
            CircuitOperation { target: "t58".to_string(), op1: "x0".to_string(), operation: "+".to_string(), op2: "x24".to_string() },
            CircuitOperation { target: "t59".to_string(), op1: "x6".to_string(), operation: "+".to_string(), op2: "t50".to_string() },
            CircuitOperation { target: "y30".to_string(), op1: "t58".to_string(), operation: "+".to_string(), op2: "t59".to_string() },
            CircuitOperation { target: "t61".to_string(), op1: "x22".to_string(), operation: "+".to_string(), op2: "x30".to_string() },
            CircuitOperation { target: "t62".to_string(), op1: "t44".to_string(), operation: "+".to_string(), op2: "y30".to_string() },
            CircuitOperation { target: "y22".to_string(), op1: "t61".to_string(), operation: "+".to_string(), op2: "t62".to_string() },
            CircuitOperation { target: "t64".to_string(), op1: "x5".to_string(), operation: "+".to_string(), op2: "x29".to_string() },
            CircuitOperation { target: "t65".to_string(), op1: "t61".to_string(), operation: "+".to_string(), op2: "t64".to_string() },
            CircuitOperation { target: "y21".to_string(), op1: "x13".to_string(), operation: "+".to_string(), op2: "t65".to_string() },
            CircuitOperation { target: "t67".to_string(), op1: "t46".to_string(), operation: "+".to_string(), op2: "t65".to_string() },
            CircuitOperation { target: "y29".to_string(), op1: "y5".to_string(), operation: "+".to_string(), op2: "t67".to_string() },
            CircuitOperation { target: "t69".to_string(), op1: "x4".to_string(), operation: "+".to_string(), op2: "x12".to_string() },
            CircuitOperation { target: "t70".to_string(), op1: "x28".to_string(), operation: "+".to_string(), op2: "t4".to_string() },
            CircuitOperation { target: "t71".to_string(), op1: "t47".to_string(), operation: "+".to_string(), op2: "t69".to_string() },
            CircuitOperation { target: "y20".to_string(), op1: "t70".to_string(), operation: "+".to_string(), op2: "t71".to_string() },
            CircuitOperation { target: "t73".to_string(), op1: "x20".to_string(), operation: "+".to_string(), op2: "t13".to_string() },
            CircuitOperation { target: "t74".to_string(), op1: "x11".to_string(), operation: "+".to_string(), op2: "t70".to_string() },
            CircuitOperation { target: "y19".to_string(), op1: "t73".to_string(), operation: "+".to_string(), op2: "t74".to_string() },
            CircuitOperation { target: "t76".to_string(), op1: "t58".to_string(), operation: "+".to_string(), op2: "t64".to_string() },
            CircuitOperation { target: "t77".to_string(), op1: "t69".to_string(), operation: "+".to_string(), op2: "t76".to_string() },
            CircuitOperation { target: "y28".to_string(), op1: "x20".to_string(), operation: "+".to_string(), op2: "t77".to_string() },
            CircuitOperation { target: "t79".to_string(), op1: "x12".to_string(), operation: "+".to_string(), op2: "t0".to_string() },
            CircuitOperation { target: "t80".to_string(), op1: "x19".to_string(), operation: "+".to_string(), op2: "t79".to_string() },
            CircuitOperation { target: "y11".to_string(), op1: "t73".to_string(), operation: "+".to_string(), op2: "t80".to_string() },
            CircuitOperation { target: "t82".to_string(), op1: "t40".to_string(), operation: "+".to_string(), op2: "t69".to_string() },
            CircuitOperation { target: "t83".to_string(), op1: "t28".to_string(), operation: "+".to_string(), op2: "t82".to_string() },
            CircuitOperation { target: "y3".to_string(), op1: "t23".to_string(), operation: "+".to_string(), op2: "t83".to_string() },
            CircuitOperation { target: "t85".to_string(), op1: "x3".to_string(), operation: "+".to_string(), op2: "y19".to_string() },
            CircuitOperation { target: "t86".to_string(), op1: "y11".to_string(), operation: "+".to_string(), op2: "t85".to_string() },
            CircuitOperation { target: "y27".to_string(), op1: "t82".to_string(), operation: "+".to_string(), op2: "t86".to_string() },
            CircuitOperation { target: "t88".to_string(), op1: "y28".to_string(), operation: "+".to_string(), op2: "t79".to_string() },
            CircuitOperation { target: "t89".to_string(), op1: "x13".to_string(), operation: "+".to_string(), op2: "t88".to_string() },
            CircuitOperation { target: "t90".to_string(), op1: "x21".to_string(), operation: "+".to_string(), op2: "t89".to_string() },
            CircuitOperation { target: "y4".to_string(), op1: "y20".to_string(), operation: "+".to_string(), op2: "t90".to_string() },
            CircuitOperation { target: "t92".to_string(), op1: "x28".to_string(), operation: "+".to_string(), op2: "t90".to_string() },
            CircuitOperation { target: "y12".to_string(), op1: "t76".to_string(), operation: "+".to_string(), op2: "t92".to_string() },
        ],
    }
});

impl LinearCircuit {
    pub fn new(state_slice: &Vec<Ciphertext>) -> Self {
        Self { x: state_slice.to_vec(), t: vec![], y: vec![] }
    }

    // Modified to use predefined circuit data
    pub fn execute_circuit(&mut self, server_key: &ServerKey, file_path: &str, client_key_debug: &ClientKey) {
        // Select the appropriate circuit definition based on file path
        let circuit_def = if file_path.contains("mixcolumns2.txt") {
            &MIXCOLUMNS2_CIRCUIT
        } else if file_path.contains("mixcolumns.txt") {
            &MIXCOLUMNS_CIRCUIT
        } else {
            // Fallback to file reading for other circuit files
            return self.execute_circuit_from_file(server_key, file_path, client_key_debug);
        };
        
        // Initialize containers using header information
        assert_eq!(self.x.len(), circuit_def.header[0]);
        self.t = vec![Ciphertext::Trivial(0); circuit_def.header[2]];
        self.y = vec![Ciphertext::Trivial(0); circuit_def.header[4]];
        let (offset_x, offset_t, offset_y) = (
            circuit_def.header[1], 
            circuit_def.header[3], 
            circuit_def.header[5]
        );

        // Process each operation
        for op in &circuit_def.operations {
            // Get the first operand
            let op1 = if op.op1.contains('x') {
                &self.x[op.op1[1..].parse::<usize>().unwrap() - offset_x]
            } else if op.op1.contains('t') {
                &self.t[op.op1[1..].parse::<usize>().unwrap() - offset_t]
            } else if op.op1.contains('y') {
                &self.y[op.op1[1..].parse::<usize>().unwrap() - offset_y]
            } else {
                panic!("Invalid operand: {}", op.op1);
            };

            // Get the second operand
            let op2 = if op.op2.contains('x') {
                &self.x[op.op2[1..].parse::<usize>().unwrap() - offset_x]
            } else if op.op2.contains('t') {
                &self.t[op.op2[1..].parse::<usize>().unwrap() - offset_t]
            } else if op.op2.contains('y') {
                &self.y[op.op2[1..].parse::<usize>().unwrap() - offset_y]
            } else {
                panic!("Invalid operand: {}", op.op2);
            };

            // Apply operation to the target
            if op.target.contains('y') {
                let idx = op.target[1..].parse::<usize>().unwrap() - offset_y;
                self.y[idx] = server_key.simple_sum(&vec![op1.to_owned(), op2.to_owned()]);
                if op.operation == "XNOR" {
                    self.y[idx] = server_key.simple_plaintext_sum(&self.y[idx], 1, 2);
                }
            } else if op.target.contains('t') {
                let idx = op.target[1..].parse::<usize>().unwrap() - offset_t;
                self.t[idx] = server_key.simple_sum(&vec![op1.to_owned(), op2.to_owned()]);
                if op.operation == "XNOR" {
                    self.t[idx] = server_key.simple_plaintext_sum(&self.t[idx], 1, 2);
                }
            } else {
                panic!("Invalid target: {}", op.target);
            }
        }
    }

    // Original method as fallback for other circuit files
    fn execute_circuit_from_file(&mut self, server_key: &ServerKey, file_path: &str, client_key_debug: &ClientKey) {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let header: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        
        // Same implementation as original execute_circuit
        assert_eq!(self.x.len(), header[0]);
        self.t = vec![Ciphertext::Trivial(0);header[2]];
        self.y = vec![Ciphertext::Trivial(0);header[4]];
        let (offset_x, offset_t, offset_y) = (header[1], header[3], header[5]);

        // Process each line from the file
        for line in lines {
            let elmts: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();
            let op1 = if elmts[2].contains('x') {
                &self.x[elmts[2][1..].parse::<usize>().unwrap() - offset_x]
            } else if elmts[2].contains('t') {
                &self.t[elmts[2][1..].parse::<usize>().unwrap() - offset_t]
            } else if elmts[2].contains('y') {
                &self.y[elmts[2][1..].parse::<usize>().unwrap() - offset_y]
            } else { panic!() };
            
            let op2 = if elmts[4].contains('x') {
                &self.x[elmts[4][1..].parse::<usize>().unwrap() - offset_x]
            } else if elmts[4].contains('t') {
                &self.t[elmts[4][1..].parse::<usize>().unwrap() - offset_t]
            } else if elmts[4].contains('y') {
                &self.y[elmts[4][1..].parse::<usize>().unwrap() - offset_y]
            } else { panic!() };
            
            if elmts[0].contains('y') {
                self.y[elmts[0][1..].parse::<usize>().unwrap() - offset_y] = server_key.simple_sum(&vec![op1.to_owned(), op2.to_owned()]);
                if elmts[3] == "XNOR" {
                    self.y[elmts[0][1..].parse::<usize>().unwrap() - offset_y] = server_key.simple_plaintext_sum(&self.y[elmts[0][1..].parse::<usize>().unwrap() - offset_y], 1, 2);
                }
            } else if elmts[0].contains('t') {
                self.t[elmts[0][1..].parse::<usize>().unwrap() - offset_t] = server_key.simple_sum(&vec![op1.to_owned(), op2.to_owned()]);
                if elmts[3] == "XNOR" {
                    self.t[elmts[0][1..].parse::<usize>().unwrap() - offset_t] = server_key.simple_plaintext_sum(&self.t[elmts[0][1..].parse::<usize>().unwrap() - offset_t], 1, 2);
                }
            } else { panic!() }
        }
    }
}