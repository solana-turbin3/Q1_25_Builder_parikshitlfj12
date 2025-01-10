import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com");
(async () => {
  try {
    const txhash = await connection.requestAirdrop(
      keypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();

// (async () => {
//   try {
//     const txhash = await connection.getBalance(
//       new PublicKey("5ME4aiNn5PxGQ5zNHM16e4b9AvQwawByQj7t1AXJfdKT")
//     );

//     console.log(`Balance - ${txhash}`);
//   } catch (e) {
//     console.error(`Oops, something went wrong: ${e}`);
//   }
// })();
