# ETSI Domain: AI

## Properties
domain-type:: ETSI Functional Domain
term-count:: 3

## Terms in this Domain

- [[Autonomous Agent]]
- [[Intelligent Virtual Entity]]
- [[Procedural Content Generation]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "AI")]]
}
#+END_QUERY
```