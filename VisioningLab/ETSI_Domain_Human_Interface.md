# ETSI Domain: Human Interface

## Properties
domain-type:: ETSI Functional Domain
term-count:: 13

## Terms in this Domain

- [[Augmented Reality (AR)]]
- [[Behavioural Feedback Loop]]
- [[Biometric Binding Mechanism]]
- [[Biosensing Interface]]
- [[Cognitive Feedback Interface]]
- [[Display Metrology]]
- [[Eye Tracking]]
- [[Feedback Mechanism]]
- [[Haptics]]
- [[Human Interface Device]]
- [[Human Interface Layer (HIL)]]
- [[Spatial Anchor]]
- [[Voice Interaction]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Human Interface")]]
}
#+END_QUERY
```