import {connection, integer_to_slice, Integers, ownerKeypair, programKeyPair, someUser, WALLET} from './config';
import {AccountInfo, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction} from "@solana/web3.js";
import {sendTransaction} from "./transaction";
import {deserializeUnchecked} from "borsh";
import {CampaignSchema, LotteryCampaign, UserData, UserSchema} from "./schemas";
import {instruction_data, Instructions} from "./instructions";

(async () => {
    console.log("start");

    let ticketBuyer = someUser;

    let campaign_num: number = 1;
    let ticket_amount: number = 10;

    // Get lottery machine PDA for deriving campaign PDA.
    let [machine_pda, _bump] = await PublicKey.findProgramAddress(
        [
            ownerKeypair.publicKey.toBuffer()
        ],
        programKeyPair.publicKey,
    );


    // Parse number to buffer.
    let campaign_num_buffer = integer_to_slice(Integers.u_16, campaign_num);

    // Get campaign PDA.
    let [campaign_pda, _] = await PublicKey.findProgramAddress(
        [
            machine_pda.toBuffer(),
            campaign_num_buffer
        ],
        programKeyPair.publicKey,
    );

    // Get ticket buyer PDA.
    let [ticket_buyer_pda, _b] = await PublicKey.findProgramAddress(
        [
            ticketBuyer.publicKey.toBuffer(),
            campaign_pda.toBuffer()
        ],
        programKeyPair.publicKey,
    );
    // Check if user already have an account and take his index,
    // or add instruction for creating new account with next user index
    // from campaign.
    let user_data = await connection.getAccountInfo(ticket_buyer_pda)

    let instructions: TransactionInstruction[] = [];
    let user_index;
    if (user_data == null) {
        let ix_data = instruction_data(Instructions.InitializeAccount);
        let initialize_ix = new TransactionInstruction({
                programId: programKeyPair.publicKey,
                data: ix_data,
                keys: [
                    {pubkey: ticketBuyer.publicKey, isSigner: true, isWritable: false},
                    {pubkey: ticket_buyer_pda, isSigner: false, isWritable: true},
                    {pubkey: campaign_pda, isSigner: false, isWritable: true},
                    {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
                    {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}

                ]

            }
        );
        instructions.push(initialize_ix);

        let campaign = await connection.getAccountInfo(campaign_pda)
            .then((account_info: AccountInfo<Buffer>) => {
                return deserializeUnchecked(CampaignSchema, LotteryCampaign, account_info.data);
            });
        let index: number = +campaign["user_index"];
        user_index = index + 1;
    } else {
        let user = deserializeUnchecked(UserSchema, UserData, user_data.data);
        user_index = +user["user_index"];

    }

    let ticket_buffer = integer_to_slice(Integers.u_16,user_index);
    // Get ticket account pda
    let [ticket_pda, ticket_bump] = await PublicKey.findProgramAddress(
        [
            campaign_pda.toBuffer(),
            ticket_buffer,
        ],
        programKeyPair.publicKey,
    );
    let buffer = instruction_data(Instructions.BuyTicket, ticket_amount,campaign_num);

    // Get campaign wallet
     let [wallet_acc, bump__] = await PublicKey.findProgramAddress(
        [
            campaign_pda.toBuffer(),
            Buffer.from(WALLET)
        ],
        programKeyPair.publicKey
    );

    let buy_ticket_ix = new TransactionInstruction({
            programId: programKeyPair.publicKey,
            data: buffer,
            keys: [
                {pubkey: ticketBuyer.publicKey, isSigner: true, isWritable: false},
                {pubkey: ticket_pda, isSigner: false, isWritable: true},
                {pubkey: ticket_buyer_pda, isSigner: false, isWritable: false},
                {pubkey: wallet_acc, isSigner: false, isWritable: true},
                {pubkey: campaign_pda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
                {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}
            ]
        }
    );
    instructions.push(buy_ticket_ix);

    let balance = 0;
    // Waiting for airdrop.
    while (balance == 0) {
        balance = await connection.getBalance(ticketBuyer.publicKey, "recent");
    }
    await sendTransaction(ticketBuyer, instructions)
        .catch(err => console.error(err));

    console.log("DONE!");

}).apply(this);







