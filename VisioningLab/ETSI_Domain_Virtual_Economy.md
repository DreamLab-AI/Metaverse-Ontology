# ETSI Domain: Virtual Economy

## Properties
domain-type:: ETSI Functional Domain
term-count:: 33

## Terms in this Domain

- [[Carbon Credit Token]]
- [[Central Bank Digital Currency (CBDC)]]
- [[Creator Economy]]
- [[Creator Royalty Token]]
- [[Crypto Token]]
- [[Cryptocurrency]]
- [[Decentralized Exchange (DEX)]]
- [[Digital Asset]]
- [[Digital Goods]]
- [[Digital Goods Registry]]
- [[Digital Real Estate]]
- [[Digital Tax Compliance Node]]
- [[Fractionalized NFT]]
- [[Industrial Metaverse]]
- [[Liquidity Pool]]
- [[Loyalty Token]]
- [[Marketplace]]
- [[Micropayment]]
- [[NFT Renting]]
- [[NFT Swapping]]
- [[NFT Wrapping]]
- [[Non-Fungible Token (NFT)]]
- [[Play-to-Earn (P2E)]]
- [[Provenance Verification]]
- [[Royalty Mechanism]]
- [[Smart Contract]]
- [[Smart Royalties Ledger]]
- [[Smart Royalty Contract]]
- [[Social Token Economy]]
- [[Stablecoin]]
- [[Token Bonding Curve]]
- [[Tokenization]]
- [[Transaction Standard]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Virtual Economy")]]
}
#+END_QUERY
```