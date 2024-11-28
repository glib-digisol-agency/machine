import {connection, integer_to_slice, Integers, ownerKeypair, programKeyPair} from "./config"
import {AccountInfo, PublicKey, TransactionInstruction,} from "@solana/web3.js";
import {draw, Instructions} from "./instructions";
import {sendTransaction} from "./transaction";
import {deserializeUnchecked} from "borsh";
import {CampaignSchema, LotteryCampaign, UserTickets, UserTicketsSchema} from "./schemas";
import {struct, Structure, u16, u8} from "@solana/buffer-layout";

(async () => {

        // Chain link accounts
        let sol = new PublicKey("99B2bTijsU6f1GCT73HmdR7HCFFjGMBcPZY6jZ96ynrR");
        let link = new PublicKey("99B2bTijsU6f1GCT73HmdR7HCFFjGMBcPZY6jZ96ynrR");
        let btc = new PublicKey("99B2bTijsU6f1GCT73HmdR7HCFFjGMBcPZY6jZ96ynrR");
        let chain_link_program = new PublicKey("HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny");

        let campaign_to_draw: number = 1;

        let [lottery_machine_pda, _] = await PublicKey.findProgramAddress(
            [
                ownerKeypair.publicKey.toBuffer()
            ],
            programKeyPair.publicKey,
        );
        //Get campaign PDA.
        let campaign_index = integer_to_slice(Integers.u_16, campaign_to_draw);
        let [campaign_pda, bump] = await PublicKey.findProgramAddress(
            [
                lottery_machine_pda.toBuffer(),
                campaign_index
            ],
            programKeyPair.publicKey,
        );
        let campaign = await connection.getAccountInfo(campaign_pda)
            .then((account_info: AccountInfo<Buffer>) => {
                return deserializeUnchecked(CampaignSchema, LotteryCampaign, account_info.data);
            })
        let users_total_count: number = campaign["user_index"];
        // let index_and_tickets_amount: Map<number,number> = new Map();
        let index_and_tickets_amount: Buffer[] = [];

        // Derive users PDAs and add to array.
        for (let i = 1; i <= users_total_count; i++) {
            let array = new Uint8Array(2);
            array.set([i]);

            let [user_pda, _] = await PublicKey.findProgramAddress(
                [
                    campaign_pda.toBuffer(),
                    array
                ],
                programKeyPair.publicKey,
            );

            let tickets_amount: number = await connection.getAccountInfo(user_pda)
                .then(account => {
                    let account_info = deserializeUnchecked(UserTicketsSchema, UserTickets, account.data);
                    return +account_info["tickets"];

                });

            const dataLayout: Structure<any> = struct([
                u16('index'), u8('tickets')
            ]);

            const data = Buffer.alloc(dataLayout.span);

            dataLayout.encode({
                index: i,
                tickets: tickets_amount,
            }, data);


            index_and_tickets_amount.push(data);
        }


        let ix_data = draw(index_and_tickets_amount.length, index_and_tickets_amount, campaign_to_draw);

        let draw_ix = new TransactionInstruction({
                programId: programKeyPair.publicKey,
                data: ix_data,
                keys: [
                    {pubkey: ownerKeypair.publicKey, isSigner: true, isWritable: false},
                    {pubkey: lottery_machine_pda, isSigner: false, isWritable: false},
                    {pubkey: campaign_pda, isSigner: false, isWritable: true},
                    {pubkey: chain_link_program, isSigner: false, isWritable: false},
                    {pubkey: sol, isSigner: false, isWritable: false},
                    {pubkey: link, isSigner: false, isWritable: false},
                    {pubkey: btc, isSigner: false, isWritable: false},
                ]


            }
        );
        await sendTransaction(ownerKeypair, [draw_ix])
            .catch(error => console.error(error));

    }
).apply(this);





