# INFI Testing Strategy

INFI needs tests at every layer.

## Test Levels

### Unit Tests

Test individual modules:

- address parsing
- amount math
- transaction validation
- storage updates
- mempool ordering
- RPC formatting

### Integration Tests

Test full flows:

- submit transaction
- execute transaction
- produce block
- query balance
- index transaction

### EVM Compatibility Tests

Required before mainnet:

- contract deployment
- contract calls
- reverts
- events
- gas accounting
- precompiles
- EIP-155 replay protection

### RPC Compatibility Tests

Required:

- `eth_chainId`
- `eth_blockNumber`
- `eth_getBalance`
- `eth_getTransactionCount`
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`
- `eth_call`
- `eth_estimateGas`
- `eth_getLogs`

### Wallet Tests

Test:

- injected wallet connection
- WalletConnect QR
- mobile deep links
- `wallet_addEthereumChain`
- `wallet_switchEthereumChain`
- manual setup fallback

### Explorer Tests

Test:

- transaction search
- address search
- block search
- indexer restart
- indexer lag reporting
- finalized block consistency

### Security Tests

Test:

- malformed transactions
- replay attempts
- invalid signatures
- nonce errors
- insufficient balance
- RPC abuse
- database recovery
- validator restart

### Tokenized Asset Tests

Test:

- mint events
- burn events
- transfer indexing
- metadata hash verification
- admin permission detection
- issuer verification display
- risk warning display

### INFI Domain Tests

Test:

- free domain creation
- reserved-name protection
- ownership transfers
- resolver updates
- wallet address resolution
- reverse resolution
- domain search indexing
- impersonation warnings
- official domain verification

### VPN Function Tests

Test:

- provider registration
- provider reputation updates
- payment settlement
- dispute handling
- slashing events
- privacy-preserving indexing
- abuse-prevention rate limits

## Mainnet Rule

If it is not tested, it is not ready for mainnet.
