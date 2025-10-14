# ETSI Domain: Identity & Trust

## Properties
domain-type:: ETSI Functional Domain
term-count:: 14

## Terms in this Domain

- [[Decentralized Identity (DID)]]
- [[Digital Identity Framework]]
- [[Digital Identity Wallet]]
- [[Federated Credential Exchange]]
- [[Humanity Attestation]]
- [[Identity Federation]]
- [[Identity Graph]]
- [[Identity Provider (IdP)]]
- [[Reputation Data]]
- [[Reputation Scoring Model]]
- [[Self-Sovereign Identity (SSI)]]
- [[Trust Framework Policy]]
- [[Trust Score Metric]]
- [[Verifiable Credential (VC)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Identity & Trust")]]
}
#+END_QUERY
```