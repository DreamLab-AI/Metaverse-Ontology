# ETSI Domain: Application / Creative

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[Virtual Performance Space]]
- [[Virtual Production Volume]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Application / Creative")]]
}
#+END_QUERY
```