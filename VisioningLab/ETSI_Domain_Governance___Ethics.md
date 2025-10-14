# ETSI Domain: Governance / Ethics

## Properties
domain-type:: ETSI Functional Domain
term-count:: 1

## Terms in this Domain

- [[Deepfakes]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Governance / Ethics")]]
}
#+END_QUERY
```