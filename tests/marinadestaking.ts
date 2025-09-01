import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import fs from 'fs';
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {  Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {  getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, NATIVE_MINT } from "@solana/spl-token";

describe("marinadestaking", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const provider = anchor.AnchorProvider.env();
    const idl = JSON.parse(fs.readFileSync('./target/idl/marinadestaking.json', 'utf8'));
    const programId = new anchor.web3.PublicKey('8dDbAnutWwbwAB5PhxPLcSCaxBCm4x6bD7o3VJyQ8DSp');
    const program = new Program(idl, programId, provider);
    let payer: anchor.web3.Keypair;
    let wallet: NodeWallet;
    let msolMint: PublicKey;
    let user: Keypair;
    let userMsolAccount: PublicKey;
    let protocolSolAccount: PublicKey;
    let protocolMsolAccount: PublicKey;
    let TOKEN_METADATA_PROGRAM_ID: PublicKey;
    let userWSolAccount: PublicKey;
    let state: PublicKey;

    before(" before hook", async () => {
        user = payer;
        wallet = provider.wallet as NodeWallet;
        payer = wallet.payer as anchor.web3.Keypair;

        msolMint = new PublicKey("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
        console.log("msolMint:", msolMint.toBase58());

        // Create user mSOL token account
        userMsolAccount = (await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer,
            msolMint,
            payer.publicKey
        )).address;
        console.log("User mSOL Account: ", userMsolAccount.toBase58());

        // Derive PDA for protocol_sol_account
        [protocolSolAccount] = await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from("protocol_sol_account")],
            program.programId
        );
        console.log("Protocol SOL Account: ", protocolSolAccount.toBase58());
        // Create protocol mSOL token account (owned by protocol PDA)
        protocolMsolAccount = (await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer,
            msolMint,
            program.programId, // let program own this
            true
        )).address;
        console.log("Protocol mSOL Account: ", protocolMsolAccount.toBase58());
        //topping up protocolSolAccount
        const topUpAmount = 1_000_000;
        const tx2 = new anchor.web3.Transaction().add(
            SystemProgram.transfer({
                fromPubkey: payer.publicKey,
                toPubkey: protocolSolAccount,
                lamports: topUpAmount,
            })
        );
        await provider.sendAndConfirm(tx2, [payer]);

        //NEW
        TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

        state = new anchor.web3.PublicKey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC")

        const endpoint = provider.connection.rpcEndpoint;
        if (endpoint.includes("devnet")) {
            console.log("Network: DEVNET");

        } else if (endpoint.includes("mainnet")) {
            console.log("Network: MAINNET");

        } else if (endpoint.includes("testnet")) {
            console.log("Network: TESTNET");
        } else {
            console.log("Network: UNKNOWN", endpoint);
        }

    
        userWSolAccount = await getAssociatedTokenAddress(
            NATIVE_MINT,
            payer.publicKey
        );

    });


    it("Deposit Withdraw", async () => {

        let lamports = new anchor.BN(10000000);
        const tx1 = await program.methods.marinadeDeposit(lamports).accounts({
            state: new anchor.web3.PublicKey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC"),
            msolMint: msolMint,
            liqPoolSolLegPda: new anchor.web3.PublicKey('UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q'),
            liqPoolMsolLeg: new anchor.web3.PublicKey('7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE'),
            liqPoolMsolLegAuthority: new anchor.web3.PublicKey('EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL'),
            reservePda: new anchor.web3.PublicKey('Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN'),
            transferFrom: provider.wallet.publicKey,
            mintTo: userMsolAccount,
            mintAuthority: new anchor.web3.PublicKey('3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM'),
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: new anchor.web3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
            marinadeProgram: new anchor.web3.PublicKey('MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD'),
        }).signers([payer]).rpc({ skipPreflight: true });
        await program.provider.connection.confirmTransaction(tx1, 'finalized');
        console.log("Marinade Deposit Tx:", tx1);

        // check spl msol balance
        const msolBalance = await provider.connection.getTokenAccountBalance(userMsolAccount);
        console.log("MSOL Token Balance:", msolBalance.value.uiAmount);

        const msolAmountToUnstake = new anchor.BN(msolBalance.value.amount);

        const tx2 = await program.methods.marinadeUnstake(msolAmountToUnstake)
            .accounts({
                state: new anchor.web3.PublicKey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC"),
                msolMint: msolMint,
                liqPoolSolLegPda: new anchor.web3.PublicKey('UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q'),
                liqPoolMsolLeg: new anchor.web3.PublicKey('7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE'),
                treasuryMsolAccount: new anchor.web3.PublicKey('8ZUcztoAEhpAeC2ixWewJKQJsSUGYSGPVAjkhDJYf5Gd'),
                getMsolFrom: userMsolAccount,
                getMsolFromAuthority: provider.wallet.publicKey,
                transferSolTo: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: new anchor.web3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
                marinadeProgram: new anchor.web3.PublicKey('MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD'),
            }).signers([payer]).rpc({ skipPreflight: true });
        await program.provider.connection.confirmTransaction(tx2, 'finalized');
        console.log("Unstake Tx: ", tx2);


    })

});




