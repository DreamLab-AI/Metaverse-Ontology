# ETSI Domain: Ethics & Law

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[Ethics & Law Layer]]
- [[Right to Be Forgotten]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Ethics & Law")]]
}
#+END_QUERY
```