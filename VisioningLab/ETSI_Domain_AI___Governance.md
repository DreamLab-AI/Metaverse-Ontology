# ETSI Domain: AI / Governance

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[Algorithmic Transparency Index]]
- [[Explainable AI (XAI)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "AI / Governance")]]
}
#+END_QUERY
```