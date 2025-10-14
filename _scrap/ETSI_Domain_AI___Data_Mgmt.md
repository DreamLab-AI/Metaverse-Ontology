# ETSI Domain: AI / Data Mgmt

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[AI Model Card]]
- [[Synthetic Data Generator]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "AI / Data Mgmt")]]
}
#+END_QUERY
```