import {
  Account,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionSignature,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Token,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as anchor from "@project-serum/anchor";
import {
  DexInstructions,
  Market as MarketSerum,
  TokenInstructions,
} from "@project-serum/serum";
import { sleep } from "./util";

export async function TokenDeploy(provider: anchor.Provider) {
  const connection = provider.connection;
  const wallet = provider.wallet;
  const tokenA = await Token.createMint(
    connection,
    wallet.payer,
    wallet.publicKey,
    wallet.publicKey,
    9,
    TOKEN_PROGRAM_ID
  );

  //   const tokenB = await Token.createMint(
  //     connection,
  //     wallet.payer,
  //     wallet.publicKey,
  //     null,
  //     9,
  //     TOKEN_PROGRAM_ID
  //   );
  const tokenAMintAddress = tokenA.publicKey;

  console.log(
    ">>>>>>>>>>>>>>>>>>>>>>>> tokenAMintAddress: ",
    tokenAMintAddress
  );
  //   const tokenBMintAddress = tokenB.publicKey;
  const associatedTokenA = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    tokenAMintAddress,
    wallet.publicKey,
    true
  );
  //   const associatedTokenB = await Token.getAssociatedTokenAddress(
  //     ASSOCIATED_TOKEN_PROGRAM_ID,
  //     TOKEN_PROGRAM_ID,
  //     tokenBMintAddress,
  //     wallet.publicKey,
  //     true
  //   );

  console.log("createAssociatedTokenAccountInstruction ");
  const tx1 = new Transaction();
  tx1.add(
    Token.createAssociatedTokenAccountInstruction(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      tokenAMintAddress,
      associatedTokenA,
      wallet.publicKey,
      wallet.publicKey
    )
    // Token.createAssociatedTokenAccountInstruction(
    //   ASSOCIATED_TOKEN_PROGRAM_ID,
    //   TOKEN_PROGRAM_ID,
    //   tokenBMintAddress,
    //   associatedTokenB,
    //   wallet.publicKey,
    //   wallet.publicKey
    // )
  );
  let txid = await provider.send(tx1);
  console.log("txId: createAssociatedTokenAccountInstruction :: ", txid);

  console.log(">>>>>>>>> >>>>>>>>> createMintToInstruction: ");
  await sleep(5000);
  const tx2 = new Transaction();
  tx2.add(
    Token.createMintToInstruction(
      TOKEN_PROGRAM_ID,
      tokenAMintAddress,
      associatedTokenA,
      wallet.publicKey,
      [wallet.payer],
      100000000000000
    )
    // Token.createMintToInstruction(
    //   TOKEN_PROGRAM_ID,
    //   tokenBMintAddress,
    //   associatedTokenB,
    //   wallet.publicKey,
    //   [wallet.payer],
    //   200000000000000
    // )
  );
  txid = await provider.send(tx2);

  console.log(
    "create tokenA mint: ",
    tokenA.publicKey.toString(),
    // " tokenB: ",
    // tokenBMintAddress.toString(),
    "mint txid: ",
    txid
  );

  console.log(
    "========================>>>>>   Balance check: ========================>>>>>"
  );

  const accountInfo = await connection.getTokenAccountBalance(associatedTokenA);

  console.log(
    "Balance of token : ",
    tokenAMintAddress,
    " == ",
    accountInfo.value.amount
  );

  await sleep(5000);

  return { tokenAMintAddress };
}

export async function TokenMintTo(
  provider: anchor.Provider,
  amount: number,
  mintAddress: PublicKey
) {
  const wallet = provider.wallet;

  const associatedTokenA = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mintAddress,
    provider.wallet.publicKey,
    true
  );
  console.log("associatedTokenA: ", associatedTokenA.toString());
  const tx2 = new Transaction();
  tx2.add(
    Token.createMintToInstruction(
      TOKEN_PROGRAM_ID,
      mintAddress,
      associatedTokenA,
      wallet.publicKey,
      [wallet.payer],
      amount
    )
    // Token.createMintToInstruction(
    //   TOKEN_PROGRAM_ID,
    //   tokenBMintAddress,
    //   associatedTokenB,
    //   wallet.publicKey,
    //   [wallet.payer],
    //   200000000000000
    // )
  );
  let txid = await provider.send(tx2);

  console.log(
    "token mintTo: ",
    mintAddress.toString(),
    // " tokenB: ",
    // tokenBMintAddress.toString(),
    "mint txid: ",
    txid
  );

  console.log(
    "========================>>>>>   Balance check: ========================>>>>>"
  );

  const accountInfo = await provider.connection.getTokenAccountBalance(
    associatedTokenA
  );

  console.log("Balance of token : ", mintAddress, " == ", {
    amount: accountInfo.value.amount,
    decimals: accountInfo.value.decimals,
    uiAmountString: accountInfo.value.uiAmountString,
  });
}
