import {} from "@dapplets/dapplet-extension";

@Injectable
export default class Dapplet {
  @Inject("twitter-bos-config")
  public adapter: any;

  async activate(): Promise<void> {
    const { bos } = this.adapter.exports;
    this.adapter.attachConfig({
      POST: (ctx: any) => [
        bos({
          initial: "DEFAULT",
          DEFAULT: {
            src: 'dapplets.near/widget/Button',
            label: "Example",
          },
        }),
      ],
    });
  }
}
