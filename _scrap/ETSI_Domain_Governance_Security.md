# ETSI Domain: Governance/Security

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[Decentralization Layer]]
- [[Zero-Knowledge Proof (ZKP)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Governance/Security")]]
}
#+END_QUERY
```