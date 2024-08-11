// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ntbnts_core::Inputs;
use ntbnts_methods::{NTBNTSREPLAY_ELF, NTBNTSREPLAY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

fn main() {
    let inputs = Inputs {
        allplayer1input:[2, 7, 17, 28].to_vec() ,
        allplayer2input:[4, 10, 34].to_vec() ,
        winner: "player 2".to_string(),
    };

    let receipt = ntbnts(&inputs);

    // Verify receipt and parse it for committed data
    receipt.verify(NTBNTSREPLAY_ID).unwrap();
    let committed_state: String = receipt.journal.decode().unwrap();
    assert_eq!(inputs.winner, committed_state);

    println!(
        "Player {} did win the game",
        committed_state,
    );
}

fn ntbnts(inputs: &Inputs) -> Receipt {
    let env = ExecutorEnv::builder()
        .write(inputs)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    prover.prove(env, NTBNTSREPLAY_ELF).unwrap().receipt
}
