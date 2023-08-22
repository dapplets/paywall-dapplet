import {} from "@dapplets/dapplet-extension";

async function getConnectedWallet() {
  const sessions = await Core.sessions();
  const walletOrigin = "near/mainnet";
  const session = sessions.find((x) => x.authMethod === walletOrigin);
  return session?.wallet();
}

async function connectWallet() {
  const session = await Core.login({ authMethods: ["near/mainnet"] });
  return session.wallet();
}

@Injectable
export default class Dapplet {
  @Inject("twitter-bos-config")
  public adapter: any;

  public wallet: any;

  async activate(): Promise<void> {
    this.wallet = await getConnectedWallet();

    const { bos } = this.adapter.exports;
    this.adapter.attachConfig({
      POST: (post) => [
        bos({
          DEFAULT: {
            src: "dapplets.near/widget/Web3Hackfest-Paywall-Content",
            post,
            accountId: this.wallet?.accountId,
            onConnect: this.handleConnectClick,
            onBuy: this.handleBuyClick,
          },
        }),
      ],
    });
  }

  handleConnectClick = async (_, me) => {
    try {
      me.loading = true;
      this.wallet = await connectWallet();
      me.accountId = this.wallet?.accountId;
    } catch (err) {
      console.error(err);
    } finally {
      me.loading = false;
    }
  };

  handleBuyClick = async (_, me) => {
    try {
      me.loading = true;
      // this.wallet = await connectWallet();
      // me.accountId = this.wallet?.accountId;
    } catch (err) {
      console.error(err);
    } finally {
      me.loading = false;
    }
  };
}
