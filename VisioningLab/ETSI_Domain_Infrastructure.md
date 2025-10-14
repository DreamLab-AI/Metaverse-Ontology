# ETSI Domain: Infrastructure

## Properties
domain-type:: ETSI Functional Domain
term-count:: 22

## Terms in this Domain

- [[6G Network Slice]]
- [[Cloud Rendering Service]]
- [[Compute Layer]]
- [[Content Delivery Network (CDN)]]
- [[Context Awareness]]
- [[Distributed Ledger Technology (DLT)]]
- [[Edge Computing Node]]
- [[Edge Mesh Network]]
- [[Edge Network]]
- [[Edge Orchestration]]
- [[Hardware Abstraction Layer (HAL)]]
- [[Infrastructure Layer]]
- [[Latency]]
- [[Latency Management Protocol]]
- [[Metaverse Architecture Stack]]
- [[Middleware]]
- [[Network Infrastructure]]
- [[Networking Layer]]
- [[Physics Engine]]
- [[Spatial Computing]]
- [[Spatial Computing Layer]]
- [[Visualization Layer]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Infrastructure")]]
}
#+END_QUERY
```