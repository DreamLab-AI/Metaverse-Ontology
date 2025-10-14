# ETSI Domain: Application / Education

## Properties
domain-type:: ETSI Functional Domain
term-count:: 2

## Terms in this Domain

- [[Cultural Heritage XR Experience]]
- [[Education Metaverse]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Application / Education")]]
}
#+END_QUERY
```