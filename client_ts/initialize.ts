import {connection, ownerKeypair, programKeyPair, SOL} from "./config";
import {PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction} from "@solana/web3.js";
import {sendTransaction} from "./transaction";
import {instruction_data, Instructions} from "./instructions";

(async () => {
    console.log("start");

    let lottery_machine_owner = ownerKeypair;
    // Create Lottery Machine PDA.
    let [machine_pda, _] = await PublicKey.findProgramAddress(
        [
            lottery_machine_owner.publicKey.toBuffer()
        ],
        programKeyPair.publicKey,
    );
    // Create initialize Machine instruction.
    let ix_data = instruction_data(Instructions.InitializeMachine);
    let initialize_campaign_ix = new TransactionInstruction({
            programId: programKeyPair.publicKey,
            data: ix_data,
            keys: [
                {pubkey: lottery_machine_owner.publicKey, isSigner: true, isWritable: false},
                {pubkey: machine_pda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
                {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},


            ]
        }
    );

    await sendTransaction(ownerKeypair, [initialize_campaign_ix])
        .catch(err => console.error(err));


    console.log("DONE!");

}).apply(this);