# ETSI Domain: Creative Media

## Properties
domain-type:: ETSI Functional Domain
term-count:: 1

## Terms in this Domain

- [[Procedural Texture]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Creative Media")]]
}
#+END_QUERY
```