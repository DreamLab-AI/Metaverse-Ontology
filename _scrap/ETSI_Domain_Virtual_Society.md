# ETSI Domain: Virtual Society

## Properties
domain-type:: ETSI Functional Domain
term-count:: 8

## Terms in this Domain

- [[Accessibility Standard]]
- [[Avatar]]
- [[Collective Intelligence System]]
- [[Digital Citizenship]]
- [[Digital Ritual]]
- [[Digital Twin of Society (DToS)]]
- [[Metaverse]]
- [[XR Accessibility Guideline]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Virtual Society")]]
}
#+END_QUERY
```