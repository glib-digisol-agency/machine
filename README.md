# solana-examples



# Structure

## Client
`cd/client_ts`
- TypeScript client to interact with smart contract  

## Commands
Next commands for instructions:
1. "Initialize" instruction `sh commands.sh client initialize`
2. "Initialize Campaign" instruction `sh commands.sh client create`
3. "Buy Ticket" instruction `sh commands.sh client buy_ticket`
4. "Draw" instruction `sh commands.sh client draw`
5. "Claim" instruction `sh commands.sh client claim`
5. "Set Manager" instruction `sh commands.sh client set_manager`

Also useful commands:
1. Show user index `sh commands.sh client index_check`
2. Show existing campaigns `sh commands.sh client show`
3. Reset data + deploy contract `sh commands.sh rust redeploy`
4. Deploy contract `sh commands.sh rust deploy`


## Contract

Lottery Machine smart contract. 
- The first step to interact with the smart contract will be the instantiation of `admin` in the [`consts`](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/processor/consts.rs#L3).

- Then you can call Initialize [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L51), because only [`admin`](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/processor/consts.rs#L3) can do this. This instruction will create [Lottery Machine ](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/processor/data_handlers.rs#L73)structure. The structure holds Campaign Manager (He can call [`Initialize Campaign`](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L34) and [`Draw`](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L65) instructions, is admin by default. Can be changed by calling  [`Set Manager`](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L92) instructions) and campaign counter, that important in deriving Campaign PDA. 

 
- Initialize campaign [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L30). Can be invoked only by `admin` or `campaign manager`.
 Instantiates `tickets price`,  `NFT` of current campaign, `wallet` for payments, and `campaign index`.
 Creates NFT associated token account and sends NFT from the invoker account to this, that will be there till the invoking `Claim Reward` [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L82).

- Initialize account [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L46)Creates an account that holds a user index for new campaign members.

- Buy Ticket [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L18)  Can be invoked by every user. If the user didn't buy tickets in the campaign, at first user should call [`Initialize account`](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L46) instruction.

- Draw [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L62) Can be invoked by `admin` or `campaigns manager`. Derives Lottery Winner by deriving random numbers from ChailLink, set the campaign as 
`finished` and instantiate the winner user index.


- Claim [instruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L76) Can be invoked by Lottery Winner. Sends reward to User Associated token account.

- Set manager [intruction](https://gitlab.devlits.com/cryptolits/solana/examples/-/blob/pda_owned_accs/program/src/instruction.rs#L92)
Can be invoked only by the admin. Sets `Campaign manager`, which can call `Initialize Campaign` and `Draw` instructions.

