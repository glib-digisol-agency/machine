import {
    connection,
    integer_to_slice,
    Integers,
    nftKeypair,
    ownerKeypair,
    programKeyPair,
    someUser,
    winnerAccos
} from "./config";
import {PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction} from "@solana/web3.js";
import {instruction_data, Instructions} from "./instructions";
import {sendTransaction} from "./transaction";
import {deserializeUnchecked} from "borsh";
import {CampaignSchema, LotteryCampaign} from "./schemas";
import {getAssociatedTokenAddress, TOKEN_PROGRAM_ID} from "@solana/spl-token";

(async () => {
    console.log("start");

    /// Winner derives by random, so, this call may not work,
    // if the winner was somebody else.
    let possible_winner = someUser;

    let campaign_num = 1;

    let lottery_machine_owner = ownerKeypair;

    // Create Lottery Machine PDA.
    let [machine_pda, _] = await PublicKey.findProgramAddress(
        [
            lottery_machine_owner.publicKey.toBuffer()
        ],
        programKeyPair.publicKey,
    );

    let buffer = integer_to_slice(Integers.u_16, campaign_num);
    let [campaign_pda, bump] = await PublicKey.findProgramAddress(
        [
            machine_pda.toBuffer(),
            buffer
        ],
        programKeyPair.publicKey,
    );
    let [winner_data_account, b] = await  PublicKey.findProgramAddress(
        [
            possible_winner.publicKey.toBuffer(),
            campaign_pda.toBuffer()
        ],
        programKeyPair.publicKey
    );



    let campaign_assoc = await getAssociatedTokenAddress(
        nftKeypair.publicKey,
        campaign_pda,
        true
    );

    // Create initialize Machine instruction.
    let ix_data = instruction_data(Instructions.Claim, 1);
    let initialize_campaign_ix = new TransactionInstruction({
            programId: programKeyPair.publicKey,
            data: ix_data,
            keys: [
                {pubkey: possible_winner.publicKey, isSigner: true, isWritable: false},
                {pubkey: winner_data_account, isSigner: false, isWritable: false},
                {pubkey: campaign_pda, isSigner: false, isWritable: false},
                {pubkey: campaign_assoc, isSigner: false, isWritable: true},
                {pubkey: winnerAccos.publicKey,isSigner:false,isWritable:true},
                {pubkey: TOKEN_PROGRAM_ID,isSigner:false,isWritable:false},



            ]
        }
    );


    await sendTransaction(possible_winner, [initialize_campaign_ix])
        .catch(err => console.error(err));


    console.log("DONE!");

}).apply(this);