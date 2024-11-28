import {AccountInfo, PublicKey} from "@solana/web3.js";
import {connection, ownerKeypair, programKeyPair, integer_to_slice,Integers} from "./config";
import {deserializeUnchecked} from "borsh";
import {CampaignSchema, LotteryCampaign, Machine, MachineSchema} from "./schemas";

(async () => {

        // Get Lottery Machine PDA.
        let [lottery_machine_pda, _] = await PublicKey.findProgramAddress(
            [
                ownerKeypair.publicKey.toBuffer()
            ],
            programKeyPair.publicKey,
        );
        // Get amount of campaigns.
        let campaign_amount: number = await connection.getAccountInfo(lottery_machine_pda)
            .then(account => {
                let machine = deserializeUnchecked(MachineSchema, Machine, account.data);
                return +machine["campaign_index"]
            });

        for (let i = 1; i <= campaign_amount; i++) {
            let campaign_index = integer_to_slice(Integers.u_16,i);
            // Get campaign address by index.
            let [campaign_pda, _] = await PublicKey.findProgramAddress(
                [
                    lottery_machine_pda.toBuffer(),
                    campaign_index,
                ],
                programKeyPair.publicKey,
            );
            // Derive and show data.
            await connection.getAccountInfo(campaign_pda)
                .then((account_info: AccountInfo<Buffer>) => {
                    let campaign = deserializeUnchecked(CampaignSchema, LotteryCampaign, account_info.data);
                    console.log(campaign);
                })
        }


    }
).apply(this);