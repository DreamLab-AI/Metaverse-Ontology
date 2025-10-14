# ETSI Domain: Human Interface / UX

## Properties
domain-type:: ETSI Functional Domain
term-count:: 1

## Terms in this Domain

- [[Cognitive Load Metric]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Human Interface / UX")]]
}
#+END_QUERY
```