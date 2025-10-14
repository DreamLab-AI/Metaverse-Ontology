# ETSI Domain: Security & Privacy

## Properties
domain-type:: ETSI Functional Domain
term-count:: 10

## Terms in this Domain

- [[Cross-Border Data Transfer Rule]]
- [[Digital Evidence Chain of Custody]]
- [[Metaverse Psychology Profile]]
- [[Post-Quantum Cryptography]]
- [[Privacy Impact Assessment (PIA)]]
- [[Privacy-Enhancing Computation (PEC)]]
- [[Security Layer]]
- [[Threat Surface Map]]
- [[Token Custody Service]]
- [[Zero-Trust Architecture (ZTA)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Security & Privacy")]]
}
#+END_QUERY
```