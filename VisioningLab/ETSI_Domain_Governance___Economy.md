# ETSI Domain: Governance / Economy

## Properties
domain-type:: ETSI Functional Domain
term-count:: 1

## Terms in this Domain

- [[Ownership & Freedom (distributed)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Governance / Economy")]]
}
#+END_QUERY
```