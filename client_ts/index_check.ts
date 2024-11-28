import {AccountInfo, PublicKey} from "@solana/web3.js";
import {connection, integer_to_slice, Integers, ownerKeypair, programKeyPair, someUser} from "./config";
import {deserializeUnchecked} from "borsh";
import {CampaignSchema, LotteryCampaign, Machine, MachineSchema, UserData, UserSchema} from "./schemas";

(async () => {


        /// Number if campaign, where check user index.
        let campaign_num: number = 1;

        let user = someUser.publicKey;

        // Get Lottery Machine PDA.
        let [lottery_machine_pda, _] = await PublicKey.findProgramAddress(
            [
                ownerKeypair.publicKey.toBuffer()
            ],
            programKeyPair.publicKey,
        );
        let index_buffer = integer_to_slice(Integers.u_16, campaign_num);
        let [campaign_pda, b] =
            await PublicKey.findProgramAddress(
                [
                    lottery_machine_pda.toBuffer(),
                    index_buffer
                ],
                programKeyPair.publicKey,
            );
        // Derive account that hold user index.
        let [user_data_account, _user_bump] =
            await PublicKey.findProgramAddress(
                [
                    user.toBuffer(),
                    campaign_pda.toBuffer(),
                ],
                programKeyPair.publicKey,
            );


        let user_index: number = await connection.getAccountInfo(user_data_account)
            .then(account => {
                let data = deserializeUnchecked(UserSchema, UserData, account.data);

                return +data["user_index"]
            })
            .catch(error => console.error(error));

        console.log("User index", user_index);

    }
).apply(this);