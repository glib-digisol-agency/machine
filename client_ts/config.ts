const solana = require('@solana/web3.js');
const url = "https://api.devnet.solana.com";
import {Keypair, PublicKey} from "@solana/web3.js"

const fs = require('fs');
const someUser = keyPairFromFile("../keys/some.json");
const ownerKeypair = keyPairFromFile("../keys/owner.json");
const nftKeypair = keyPairFromFile("../keys/nft.json");
const nftAccos = keyPairFromFile("../keys/nft_acc.json");
const winnerAccos = keyPairFromFile("../keys/winner_acc.json");
const WALLET = "wallet";

const programKeyPair = keyPairFromFile("../dist/program/example_program-keypair.json");
const SOL = 1000000000;

function keyPairFromFile(path: string): Keypair {
    let payerJson = JSON.parse(fs.readFileSync(path));
    return Keypair.fromSecretKey(new Uint8Array(payerJson))
}


const connection = new solana.Connection(
    url,
    'confirmed'
);
const getAirdrop = async function (pubkey: PublicKey): Promise<void> {
    await connection.requestAirdrop(pubkey, SOL);
}
const integer_to_slice = function (ty: Integers,num: number): Buffer {
    let array = new Uint8Array(ty);

    array.set([num]);

    return Buffer.from(array)
}
enum Integers
{
    u_8 = 1,
    u_16 = 2,
    u_32 = 4,
    u_64 = 8,

}

export {
    connection,
    programKeyPair,
    SOL,
    someUser,
    getAirdrop,
    ownerKeypair,
    integer_to_slice,
    Integers,
    nftKeypair,
    nftAccos,
    WALLET,
    winnerAccos
}