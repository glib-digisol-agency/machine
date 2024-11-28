import {Struct} from "@solana/web3.js";


// Corresponding to contract data structures.

class LotteryCampaign extends Struct {
    constructor(properties) {
        super(properties);
    }
}


const CampaignSchema = new Map(
    [
        [
            LotteryCampaign,
            {
                kind: 'struct',
                fields: [
                    ["campaign_index", "u16"],
                    ["tickets", "u16"],
                    ["bought_tickets", "u16"],
                    ["user_index", "u16"],
                    ["price", "u32"],
                    ["wallet", [32]],
                    ["nft", [32]],
                    ["is_finished", "u8"],
                    ["winner", "u16"],
                ]

            }
        ]
    ]
);

class UserData extends Struct {
    constructor(properties) {
        super(properties);
    }
}

const UserSchema = new Map(
    [
        [
            UserData,
            {
                kind: 'struct',
                fields: [
                    ["user_index", "u16"],
                ]

            }
        ]
    ]
);

class Machine extends Struct {
    constructor(properties) {
        super(properties);
    }
}

const MachineSchema = new Map(
    [
        [
            Machine,
            {
                kind: 'struct',
                fields: [
                    ["campaign_index", "u16"],
                ]

            }
        ]
    ]
);

class UserTickets extends Struct {
    constructor(properties) {
        super(properties);
    }
}

const UserTicketsSchema = new Map(
    [
        [
            UserTickets,
            {
                kind: 'struct',
                fields: [
                    ["owner", [32]],
                    ["tickets", "u16"],
                    ["campaign_index", "u16"]
                ]

            }
        ]
    ]
);


export
{
    UserData,
    UserSchema,
    CampaignSchema,
    LotteryCampaign,
    Machine,
    MachineSchema,
    UserTickets,
    UserTicketsSchema
}