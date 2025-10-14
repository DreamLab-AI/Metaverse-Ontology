# ETSI Domain: Application / Health

## Properties
domain-type:: ETSI Functional Domain
term-count:: 1

## Terms in this Domain

- [[Health Metaverse Application]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Application / Health")]]
}
#+END_QUERY
```