import {
    connection,
    integer_to_slice,
    Integers,
    nftAccos,
    nftKeypair,
    ownerKeypair,
    programKeyPair,
    WALLET
} from './config';
import {AccountInfo, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction} from "@solana/web3.js";
import {sendTransaction} from "./transaction";
import {deserializeUnchecked} from "borsh";
import {Machine, MachineSchema} from "./schemas";
import {instruction_data, Instructions} from "./instructions";
import {getAssociatedTokenAddress, TOKEN_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID} from "@solana/spl-token"

(async () => {

    console.log("start");
    // await connection.requestAirdrop(payerKeypair.publicKey, SOL);

    // Get lottery machine PDA.
    let [machine_pda, _] = await PublicKey.findProgramAddress(
        [
            ownerKeypair.publicKey.toBuffer()
        ],
        programKeyPair.publicKey,
    );
    // Get next campaign number from Lottery Machine.
    let campaign_index: number = await connection.getAccountInfo(machine_pda)
        .then((account_info: AccountInfo<Buffer>) => {
            let data = deserializeUnchecked(MachineSchema, Machine, account_info.data);
            return +data["campaign_index"]
        })
        .catch(error => console.error(error));


    let buffer = integer_to_slice(Integers.u_16, campaign_index + 1);
    // Create new campaign PDA from Lottery Machine PDA,
    // and campaign index.
    let [campaign_pda, bump] = await PublicKey.findProgramAddress(
        [
            machine_pda.toBuffer(),
            buffer
        ],
        programKeyPair.publicKey,
    );


    let owner_assoc = nftAccos;



    let [wallet_acc, bump__] = await PublicKey.findProgramAddress(
        [
            campaign_pda.toBuffer(),
            Buffer.from(WALLET)
        ],
        programKeyPair.publicKey
    );

    let campaign_nft_account = await getAssociatedTokenAddress(nftKeypair.publicKey,campaign_pda,true);


    // Create initialize instruction.
    let ix_data = instruction_data(Instructions.InitializeCampaign,100000);
    let create_campaign_ix = new TransactionInstruction({
            programId: programKeyPair.publicKey,
            data: ix_data,
            keys: [
                {pubkey: ownerKeypair.publicKey, isSigner: true, isWritable: true},
                {pubkey: campaign_pda, isSigner: false, isWritable: true},
                {pubkey: machine_pda, isSigner: false, isWritable: true},
                {pubkey: nftKeypair.publicKey, isSigner: false, isWritable: false},
                {pubkey: owner_assoc.publicKey, isSigner: false, isWritable: true},
                {pubkey: campaign_nft_account, isSigner: false, isWritable: true},
                {pubkey: wallet_acc, isSigner: false, isWritable: false},
                {pubkey: TOKEN_PROGRAM_ID, isSigner:false,isWritable:false},
                {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner:false,isWritable:false},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
                {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},


            ]
        }
    );

    await sendTransaction(ownerKeypair, [create_campaign_ix])
        .catch(err => console.error(err));


    console.log("DONE!");
}).apply(this);

