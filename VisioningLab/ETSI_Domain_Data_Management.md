# ETSI Domain: Data Management

## Properties
domain-type:: ETSI Functional Domain
term-count:: 14

## Terms in this Domain

- [[Collective Memory Archive]]
- [[Conversion Pipeline]]
- [[Data Fabric Architecture]]
- [[Data Integration Interface]]
- [[Data Protection]]
- [[Data Provenance]]
- [[Data Storage Layer]]
- [[Digital Ontology Repository]]
- [[Metadata Standard]]
- [[Ontology]]
- [[Provenance Ontology (PROV-O)]]
- [[Semantic Metadata Registry]]
- [[Spatial Index]]
- [[Telemetry & Analytics]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Data Management")]]
}
#+END_QUERY
```