import {Keypair, sendAndConfirmTransaction, Transaction, TransactionInstruction} from "@solana/web3.js";

import {connection} from './config';

async function sendTransaction(signer: Keypair, instruction: TransactionInstruction[]) {
    let transaction = new Transaction();

    instruction
        .map(
            instruction =>
                transaction.add(
                    instruction
                )
        );

    sendAndConfirmTransaction(
        connection,
        transaction,
        [signer]
    )
        .then(signature => console.log("DONE! ", signature))
        .catch(err => console.error(err));

}

export
{
    sendTransaction
}