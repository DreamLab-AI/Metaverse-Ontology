# ETSI Domain: Governance/Compliance

## Properties
domain-type:: ETSI Functional Domain
term-count:: 3

## Terms in this Domain

- [[Consent Management]]
- [[Testing Process]]
- [[Validation Process]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Governance/Compliance")]]
}
#+END_QUERY
```