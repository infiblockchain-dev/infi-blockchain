const REOWN_PROJECT_ID = "922256be3a36e2c56443f4b09efaf7bf";
const APP_URL = "https://infi-blockchain.pages.dev";
const INFI_RPC_URL = "https://infi-testnet-rpc.onrender.com";
const INFI_EXPLORER_URL = "https://scan.infi.infi";
const INFI_TESTNET_CHAIN_ID = 98402;
const INFI_TESTNET_CAIP_ID = "eip155:98402";
const INFI_CHAIN_ICON_URL = `${APP_URL}/assets/brand/infi-logo-transparent.png`;
const INFI_TOKEN_ICON_URL = `${APP_URL}/assets/tokens/invertx-icon-512.png`;
const ACTIVE_APP_URL = window.location.origin && window.location.origin !== "null"
  ? window.location.origin
  : APP_URL;

function dispatchWalletConnectEvent(name, detail = {}) {
  document.dispatchEvent(new CustomEvent(name, { detail }));
}

async function loadAppKit() {
  const [appKitModule, ethersAdapterModule, networksModule] = await Promise.all([
    import("https://esm.sh/@reown/appkit"),
    import("https://esm.sh/@reown/appkit-adapter-ethers"),
    import("https://esm.sh/@reown/appkit/networks")
  ]);

  const { createAppKit } = appKitModule;
  const { EthersAdapter } = ethersAdapterModule;
  const { defineChain } = networksModule;

  if (!createAppKit || !EthersAdapter || !defineChain) {
    throw new Error("Reown AppKit module exports are unavailable.");
  }

  const infiTestnet = defineChain({
    id: INFI_TESTNET_CHAIN_ID,
    caipNetworkId: INFI_TESTNET_CAIP_ID,
    chainNamespace: "eip155",
    name: "INFI Testnet",
    nativeCurrency: {
      decimals: 18,
      name: "test InvertX",
      symbol: "tINVX"
    },
    rpcUrls: {
      default: {
        http: [INFI_RPC_URL]
      }
    },
    blockExplorers: {
      default: {
        name: "INFI Scan",
        url: INFI_EXPLORER_URL
      }
    },
    testnet: true
  });

  const metadata = {
    name: "INFI Blockchain",
    description: "INFI Testnet wallet setup, test InvertX faucet, and public RPC access.",
    url: ACTIVE_APP_URL,
    icons: [INFI_CHAIN_ICON_URL, INFI_TOKEN_ICON_URL]
  };

  const modal = createAppKit({
    adapters: [new EthersAdapter()],
    networks: [infiTestnet],
    defaultNetwork: infiTestnet,
    metadata,
    projectId: REOWN_PROJECT_ID,
    customRpcUrls: {
      [INFI_TESTNET_CAIP_ID]: [{ url: INFI_RPC_URL }]
    },
    chainImages: {
      [INFI_TESTNET_CHAIN_ID]: INFI_CHAIN_ICON_URL
    },
    features: {
      analytics: true,
      email: false,
      socials: []
    },
    themeMode: "dark",
    themeVariables: {
      "--w3m-accent": "#22ff9a",
      "--w3m-color-mix": "#06161c",
      "--w3m-color-mix-strength": 24,
      "--w3m-border-radius-master": "2px"
    }
  });

  return {
    modal,
    open() {
      return modal.open({ view: "Connect" });
    }
  };
}

async function bootWalletConnect() {
  window.INFI_WALLETCONNECT = {
    ready: false,
    loading: true,
    projectId: REOWN_PROJECT_ID
  };

  try {
    const session = await loadAppKit();
    window.INFI_WALLETCONNECT = {
      ...window.INFI_WALLETCONNECT,
      ...session,
      ready: true,
      loading: false
    };
    dispatchWalletConnectEvent("infi:walletconnect-ready");
  } catch (error) {
    console.error("INFI WalletConnect failed to load", error);
    window.INFI_WALLETCONNECT = {
      ready: false,
      loading: false,
      projectId: REOWN_PROJECT_ID,
      error
    };
    dispatchWalletConnectEvent("infi:walletconnect-error", { error });
  }
}

bootWalletConnect();
