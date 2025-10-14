# ETSI Domain: Reality Capture

## Properties
domain-type:: ETSI Functional Domain
term-count:: 5

## Terms in this Domain

- [[Digital Twin]]
- [[Human Capture & Recognition]]
- [[Motion Capture Rig]]
- [[Photogrammetry]]
- [[Reality Capture System]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Reality Capture")]]
}
#+END_QUERY
```