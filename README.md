![image](/docs/paywall-dapplet.png)

# NEAR Paywall Dapplet

The Paywall Dapplet seamlessly integrates with Twitter, utilizing the NEAR Protocol and NEAR BOS to display paid content, a solution developed during the [Web3 Hackfest 2023 hackathon](https://web3hackfest.org/).

We got first place in two tracks. More details can be found here: https://devfolio.co/projects/dapplets-72b4

See our demo video on YouTube: [Embedding BOS-components into existing websites](https://www.youtube.com/watch?v=FhgCqj5oWds)

## How to try?

1. Install the forked Dapplet Extension from [Chrome Web Store](https://chrome.google.com/webstore/detail/dapplets-development-buil/oldijfflfojekjlmkjclmjmnpdinieaa)
2. Go to Twitter profile with paid content [https://twitter.com/MrConCreator](https://twitter.com/MrConCreator)
3. Click on the extension icon, activate the NEAR Paywall dapplet
4. BOS-components with blurred pictures should be injected into tweets
5. Connect mainnet NEAR wallet and buy some pictures
6. Pictures should become unblurred
7. Click on the dapplet's icon in the right side overlay
8. Make sure that purchased content became visible there

## Related Repositories

* [Paywall Dapplet](https://github.com/dapplets/paywall-dapplet)
* [Dapplet Extension (forked branch `near-bos`)](https://github.com/dapplets/dapplet-extension/tree/near-bos)
* [NEAR BOS Twitter Config (forked branch `near-bos`)](https://github.com/dapplets/modules-monorepo/tree/near-bos)

## Getting Started

1. Go to module folder and `npm i` to install dependences.  
2. `npm start` to run module at localhost.

## Learn more

This project was bootstrapped with [Create Dapplet App](https://github.com/dapplets/create-dapplet-app)

* **Dapplets Project** - [dapplets site](https://dapplets.org/)
* **Documentation** - [documentation](https://docs.dapplets.org/docs/)
* **GitHub Project Dapplets** - [github](https://github.com/dapplets)
