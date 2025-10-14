# ETSI Domain: Governance / Society

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[Content Moderation]]
- [[Ethical Framework]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Governance / Society")]]
}
#+END_QUERY
```