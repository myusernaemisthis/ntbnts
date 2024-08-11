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
use risc0_zkvm::guest::env;

pub fn main() {
	let inputs: Inputs = env::read();
	let allplayer1input : Vec::<u64> = inputs.allplayer1input;
	let allplayer2input : Vec::<u64> = inputs.allplayer2input;
	let winner : String = inputs.winner;
    env::commit(&winner);
	let mut lastinput = 1;
	let mut i = 0;
    let mut won : String;
	loop {
	       let player1 = allplayer1input[i];
	if lastinput > player1||player1 > (lastinput*2) {
	won = "player 2".to_string();
	break;
	}
	lastinput = player1;
	let player2 = allplayer2input[i];
	i += 1;
	if lastinput > player2||player2 > (lastinput*2) {
	 won = "player 1".to_string();
	break;
	}
	   lastinput = player2;
	}
	assert!(won == winner);
}
