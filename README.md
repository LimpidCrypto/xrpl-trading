# xrpl-trading
A trading bot that performs arbitrage and market making on the XRP Ledger

## Concepts
1. Arbitrage
  1.1 Cross exchange arbitrage
  1.2 Triangular arbitrage
2. Market making

### 1. Arbitrage
Arbitrage trading describes the practice of taking advantage of differences in prices of the same assets in two or more markets.
In other words you are buying an asset in one market and at the same time you are selling the same asset at a higher price in a second market.

Often times arbitrageurs have prefunded multiple asstets on multiple exchanges to monitor as many possibilities for arbitrage opportunities as possible and to act as fast as possible. Arbitrageurs always have to make sure they've funded their assets they want to trade. If they don't have them funded they have to first buy the asset they want to spend which takes extra time and fees. Using the DeX and path finding of the XRP Ledger we can fund the wished currency, buy an assets and sell that asset all at the same time with just one transaction. So the arbitrage opportunity will be taken super fast and will cost a friction of a cent (in the best case).

```
FT = FTxn + FTfr
```
where,
 - F<sub>T</sub>: Total fee
 - F<sub>Txn</sub>: Transaction fee
 - F<sub>Tfr</sub>: Transfer fee

### 1.1 Cross exchange arbitrage
>Cross-Exchange Trade (Simple Arbitrage)
>One is what's known as a cross-exchange trade, also just known as simple arbitrage. This is where you buy a cryptocurrency on one exchange and then >transfer it to another, where it's sold for a higher price [^1]

[^1]: https://www.fxcm.com/markets/insights/crypto-arbitrage-guide/
