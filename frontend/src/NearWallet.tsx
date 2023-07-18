// near api js
import { providers } from "near-api-js";

// wallet selector
import "@near-wallet-selector/modal-ui/styles.css";
import { setupModal } from "@near-wallet-selector/modal-ui";
import { NetworkId, WalletSelector, setupWalletSelector, Wallet } from "@near-wallet-selector/core";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";

const THIRTY_TGAS = "30000000000000";
const NO_DEPOSIT = "0";

// Wallet that simplifies using the wallet selector
export class AppWallet {
    walletSelector?: WalletSelector | undefined;
    wallet?: Wallet | undefined;
    network;
    createAccessKeyFor?: string | undefined;
    accountId?: string | undefined;

    constructor({
        createAccessKeyFor = undefined,
        network = "testnet",
    }: {
        createAccessKeyFor?: string;
        network?: string;
    }) {
        // Login to a wallet passing a contractId will create a local
        // key, so the user skips signing non-payable transactions.
        // Omitting the accountId will result in the user being
        // asked to sign all transactions.
        this.createAccessKeyFor = createAccessKeyFor;
        this.network = "testnet";
    }

    // To be called when the website loads
    async startUp() {
        this.walletSelector = await setupWalletSelector({
            network: this.network as NetworkId,
            modules: [setupMyNearWallet()],
        });

        const isSignedIn = this.walletSelector.isSignedIn();

        if (isSignedIn) {
            this.wallet = await this.walletSelector.wallet();
            this.accountId = this.walletSelector.store.getState().accounts[0].accountId;
        }

        return isSignedIn;
    }

    // Sign-in method
    signIn() {
        const description = "Please select a wallet to sign in.";
        const modal = setupModal(this.walletSelector!, { contractId: this.createAccessKeyFor!, description });
        modal.show();
    }

    // Sign-out method
    async signOut() {
        await this.wallet?.signOut();
        this.wallet = undefined;
        this.accountId = undefined;
        this.createAccessKeyFor = undefined;
        window.location.replace(window.location.origin + window.location.pathname);
    }

    // Make a read-only call to retrieve information from the network
    async viewMethod({ contractId, method, args = {} }: { contractId: string; method: string; args?: any }) {
        const { network } = this.walletSelector!.options;
        const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

        const res: any = await provider.query({
            request_type: "call_function",
            account_id: contractId,
            method_name: method,
            args_base64: Buffer.from(JSON.stringify(args)).toString("base64"),
            finality: "optimistic",
        });

        // eslint-disable-next-line @typescript-eslint/no-unsafe-return, @typescript-eslint/no-unsafe-argument, @typescript-eslint/no-unsafe-member-access
        return JSON.parse(Buffer.from(res.result).toString());
    }

    // Call a method that changes the contract's state
    async callMethod({
        contractId,
        method,
        args = {},
        gas = THIRTY_TGAS,
        deposit = NO_DEPOSIT,
    }: {
        contractId: string;
        method: string;
        args?: object;
        gas?: string;
        deposit?: string;
    }) {
        // Sign a transaction with the "FunctionCall" action
        return await this.wallet?.signAndSendTransaction({
            signerId: this.accountId,
            receiverId: contractId,
            actions: [
                {
                    type: "FunctionCall",
                    params: {
                        methodName: method,
                        args,
                        gas,
                        deposit,
                    },
                },
            ],
        });
    }

    // Get transaction result from the network
    async getTransactionResult(txhash: string) {
        const { network } = this.walletSelector!.options;
        const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

        // Retrieve transaction result from the network
        const transaction = await provider.txStatus(txhash, "unnused");
        // eslint-disable-next-line @typescript-eslint/no-unsafe-return
        return providers.getTransactionLastResult(transaction);
    }
}
