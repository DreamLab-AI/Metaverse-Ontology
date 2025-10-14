# ETSI Domain: Immersive Experiences

## Properties
domain-type:: ETSI Functional Domain
term-count:: 21

## Terms in this Domain

- [[Application Layer]]
- [[Authoring Tool]]
- [[Distributed Architecture]]
- [[Emotional Immersion]]
- [[Experience Layer]]
- [[Extended Reality (XR)]]
- [[Immersion]]
- [[Immersive Experience]]
- [[Mixed Reality (MR)]]
- [[Narrative Design Ontology]]
- [[Open World]]
- [[Physics-Based Animation]]
- [[Presence]]
- [[Procedural Audio Generator]]
- [[Real-Time Rendering Pipeline]]
- [[Scene Graph]]
- [[Spatial Audio Scene Description]]
- [[Virtual Lighting Model]]
- [[Virtual Reality (VR)]]
- [[Virtual World]]
- [[World Instance]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Immersive Experiences")]]
}
#+END_QUERY
```