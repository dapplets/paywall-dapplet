import {} from "@dapplets/dapplet-extension";

const { near } = Core;

type Session = { accountId: string; contract: any };

async function getExistingSession(): Promise<Session> {
  const sessions = await Core.sessions();
  const walletOrigin = "near/mainnet";
  const session = sessions.find((x) => x.authMethod === walletOrigin);
  if (!session) {
    return { accountId: null, contract: null };
  }

  const { accountId } = await session.wallet();
  const contract = await session.contract("app.paywall.near", {
    viewMethods: ["purchases", "purchased"],
    changeMethods: ["buy"],
  });

  return { accountId, contract };
}

async function createSession(): Promise<Session> {
  const session = await Core.login({ authMethods: ["near/mainnet"] });
  if (!session) {
    return { accountId: null, contract: null };
  }

  const { accountId } = await session.wallet();
  const contract = await session.contract("app.paywall.near", {
    viewMethods: ["purchases", "purchased"],
    changeMethods: ["buy"],
  });

  return { accountId, contract };
}

@Injectable
export default class {
  @Inject("twitter-bos-config")
  public adapter;

  public session: Session;

  async activate(): Promise<void> {
    this.session = await getExistingSession();

    const { bos } = this.adapter.exports;
    this.adapter.attachConfig({
      POST: (post) => [
        bos({
          DEFAULT: {
            src: "paywall.near/widget/PaywallDapplet-Content",
            contentId: post.id,
            accountId: this.session.accountId,
            loading: false,
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
      this.session = await createSession();
      me.accountId = this.session.accountId;
    } catch (err) {
      console.error(err);
    } finally {
      me.loading = false;
    }
  };

  handleBuyClick = async ({ contentId, price }, me) => {
    try {
      me.loading = true;
      await this.session.contract.buy(
        { content_id: contentId },
        null, // default gas
        near.utils.format.parseNearAmount(price)
      );
    } catch (err) {
      console.error(err);
      me.loading = false;
    }
  };
}
