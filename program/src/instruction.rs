use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ExampleInstruction {
    ///  Buy tickets
    ///  Account references
    ///  0.[SIGNER] Buyer account
    ///  1.[WRITE] Ticket account
    ///  2.[] Campaign buyer account
    ///  3 [WRITE] Campaign wallet account
    ///  4.[Write] Campaign account
    ///  5.[] System program
    ///  6.[] Rent
    BuyTicket {
        #[allow(dead_code)]
        amount: u16,
        #[allow(dead_code)]
        campaign_index: u16,
    },

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
    InitializeCampaign {
        #[allow(dead_code)]
        price: u32,
    },

    ///  Initialize new use account.
    ///  Account references
    ///  0.[SIGNER] Ticket buyer account
    ///  1.[WRITE] New user account
    ///  2.[Write] Campaign account
    ///  4.[] System program
    ///  5.[] Rent
    InitializeAccount,

    ///  Initialize Lottery Machine.
    ///  Account references
    ///  0.[SIGNER] Initializer account
    ///  1.[WRITE] New machine account
    ///  2.[] System program
    ///  3.[] Rent
    InitializeMachine,

    /// Draw
    /// Account references
    /// 0.[SIGNER] Admin account
    /// 1.[] Lottery machine PDA
    /// 2.[WRITe] Campaign PDA
    /// 3.[] Chain link program
    /// 4.[] Chain link feed
    /// 5.[] Chain link feed
    /// 6.[] Chain link feed
    Draw {
        #[allow(dead_code)]
        campaign_index: u16,
        #[allow(dead_code)]
        user: Vec<(u16, u8)>,
    },

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
    ClaimReward {
        #[allow(dead_code)]
        campaign_index: u16,
    },

    ///  Set campaign manager
    ///  Account references
    /// 0.[Signer] Admin account
    /// 1.[Write] Lottery machine account
    /// 2.[] manager account
    SetCampaignManager,
}
