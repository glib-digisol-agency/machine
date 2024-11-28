import {ownerKeypair, programKeyPair, someUser} from "./config";
import {PublicKey, TransactionInstruction} from "@solana/web3.js";
import {sendTransaction} from "./transaction";
import {instruction_data, Instructions} from "./instructions";

(async () => {
    console.log("start");

    let new_manager = someUser.publicKey;

    let lottery_machine_owner = ownerKeypair;
    // Create Lottery Machine PDA.
    let [machine_pda, _] = await PublicKey.findProgramAddress(
        [
            lottery_machine_owner.publicKey.toBuffer()
        ],
        programKeyPair.publicKey,
    );
    let data_ix = instruction_data(Instructions.SetManager);
    // Create initialize Machine instruction.
    let initialize_campaign_ix = new TransactionInstruction({
            programId: programKeyPair.publicKey,
            data: data_ix,
            keys: [
                {pubkey: lottery_machine_owner.publicKey, isSigner: true, isWritable: false},
                {pubkey: machine_pda, isSigner: false, isWritable: true},
                {pubkey: new_manager, isSigner: false, isWritable: false},


            ]
        }
    );

    await sendTransaction(ownerKeypair, [initialize_campaign_ix])
        .catch(err => console.error(err));


    console.log("DONE!");

}).apply(this);
