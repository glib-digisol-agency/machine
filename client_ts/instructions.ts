import {struct, Structure, u16, u32, u8} from "@solana/buffer-layout";
import {Struct} from "@solana/web3.js";
import {serialize} from "borsh";

// Corresponding to contract instructions.
enum Instructions {

    ///  Buy tickets
    ///  Account references
    ///  0.[SIGNER] Buyer account
    ///  1.[WRITE] Ticket account
    ///  2.[] Campaign buyer account
    ///  3 [WRITE] Campaign wallet account
    ///  4.[Write] Campaign account
    ///  5.[] System program
    ///  6.[] Rent
    BuyTicket = 0,

    ///  Initialize new campaign
    ///  Account references
    ///  0.[SIGNER] Initializer account
    ///  1.[WRITE] Campaign account
    ///  2.[Write] Lottery Machine account
    ///  3.[] NFT account
    ///  4.[WRITE] Initialize NFT associated account
    ///  5.[WRITE] Campaign NFT associated account
    ///  6.[] Campaign wallet account
    ///  7.[] Token program
    ///  8.[] Associated token program
    ///  9.[] System program
    ///  10.[] Rent

    InitializeCampaign = 1,


    ///  Initialize new use account.
    ///  Account references
    ///  0.[SIGNER] Ticket buyer account
    ///  1.[WRITE] New user account
    ///  2.[Write] Campaign account
    ///  4.[] System program
    ///  5.[] Rent
    InitializeAccount = 2,


    ///  Initialize Lottery Machine.
    ///  Account references
    ///  0.[SIGNER] Initializer account
    ///  1.[WRITE] New machine account
    ///  2.[] System program
    ///  3.[] Rent
    InitializeMachine = 3,

    /// Draw
    /// Account references
    /// 0.[SIGNER] Admin account
    /// 1.[] Lottery machine PDA
    /// 2.[WRITe] Campaign PDA
    /// 3.[] Chain link program
    /// 4.[] Chain link feed
    /// 5.[] Chain link feed
    /// 6.[] Chain link feed
    Draw = 4,

    ///  Claim reward for winner
    ///  Account references
    /// 0.[Signer] Winner account
    /// 1.[] Winner data account
    /// 2.[] Campaign account
    /// 3.[Write] Campaign Token account
    /// 4.[Write] Winner Token account
    /// 5.[] Token program
    /// 6.[] System program
    /// 7.[] Rent
    Claim = 5,

    ///  Set campaign manager
    ///  Account references
    /// 0.[Signer] Admin account
    /// 1.[Write] Lottery machine account
    /// 2.[] manager account
    SetManager = 6,
}

// Create instruction data for contract.
function instruction_data(instruction: Instructions, param?: any, param2?: any): Buffer {
    switch (instruction) {
        case Instructions.BuyTicket: {
            const dataLayout: Structure<any> = struct([
                u8('instruction'), u16('amount'), u16('campaign_index')
            ]);

            const data = Buffer.alloc(dataLayout.span);

            dataLayout.encode({
                instruction: 0,
                amount: param,
                campaign_index: param2,
            }, data);

            return data;
        }
        case Instructions.InitializeCampaign: {
            const dataLayout: Structure<any> = struct([
                u8('instruction'), u32('price')
            ]);

            const data = Buffer.alloc(dataLayout.span);

            dataLayout.encode({
                instruction: 1,
                price: param,
            }, data);

            return data;
        }

        case Instructions.InitializeAccount: {
            return Buffer.from(Uint8Array.from([2]));
        }
        case Instructions.InitializeMachine:
            return Buffer.from(Uint8Array.from([3]));
        case Instructions.Draw: {
            const dataLayout: Structure<any> = struct([
                u8('instruction'), u16('campaign_index')
            ]);

            const data = Buffer.alloc(dataLayout.span);

            dataLayout.encode({
                instruction: 4,
                campaign_index: param,
            }, data);

            return data;
        }
        case Instructions.Claim: {
            const dataLayout: Structure<any> = struct([
                u8('instruction'), u16('campaign_index'),
            ]);


            const data = Buffer.alloc(dataLayout.span);

            dataLayout.encode({
                instruction: 5,
                campaign_index: param,

            }, data);

            return data;
        }
        case Instructions.SetManager: {
            return Buffer.from(Uint8Array.from([6]));
        }

    }


}



function draw(len: number, map: Buffer[], campaign_index): Buffer {

    class Draw extends Struct {


        constructor(properties) {
            super(properties);
        }
    }

    const DrawSchema = new Map([
        [
            Draw,
            {
                kind: "struct",
                fields: [
                    ["instruction", "u8"],
                    ["campaign_index", "u16"],
                    ["data_len","u32"],
                    ["user",  [[3],len],

                ]
                ]
            }
        ]

    ]);


    let ix = new Draw({
        instruction: 4,
        campaign_index: campaign_index,
        data_len: len,
        user: map,
    }
    );

    return Buffer.from(serialize(DrawSchema, ix))

}





export
{
    instruction_data,
    Instructions,
    draw,
}
