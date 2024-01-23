import {} from '@dapplets/dapplet-extension'

type Session = { accountId: string; contract: any }

async function getExistingSession(): Promise<Session> {
    const sessions = await Core.sessions()
    const walletOrigin = 'near/mainnet'
    const session = sessions.find((x) => x.authMethod === walletOrigin)
    if (!session) {
        return { accountId: null, contract: null }
    }

    const { accountId } = await session.wallet()
    const contract = await session.contract('app.paywall.near', {
        viewMethods: ['purchases', 'purchased'],
        changeMethods: ['buy'],
    })

    return { accountId, contract }
}

async function createSession(): Promise<Session> {
    const session = await Core.login({ authMethods: ['near/mainnet'] })
    if (!session) {
        return { accountId: null, contract: null }
    }

    const { accountId } = await session.wallet()
    const contract = await session.contract('app.paywall.near', {
        viewMethods: ['purchases', 'purchased'],
        changeMethods: ['buy'],
    })

    return { accountId, contract }
}

@Injectable
export default class {
    @Inject('twitter-bos-config') private adapter

    private session: Session
    private state = Core.state({ accountId: null })

    async activate(): Promise<void> {
        this.session = await getExistingSession()
        this.state.global.accountId.next(this.session.accountId)

        Core.onAction(async () => {
            if (!this.state.global.accountId.value) {
                try {
                    this.session = await createSession()
                    this.state.global.accountId.next(this.session.accountId)
                } catch (err) {
                    console.error(err)
                }
            }
        })

        const { bos } = this.adapter.exports
        this.adapter.attachConfig({
            POST: (post) => [
                bos({
                    DEFAULT: {
                        src: 'bos.dapplets.near/widget/Paywall.Main',
                        link: {
                            linkId: post.id + '/twitter',
                        },
                        nearAccountId: this.state.global.accountId.value,
                    },
                }),
            ],
        })
    }
}
