# ETSI Domain: Interoperability

## Properties
domain-type:: ETSI Functional Domain
term-count:: 18

## Terms in this Domain

- [[3D Scene Exchange Protocol (SXP)]]
- [[API Standard]]
- [[Avatar Interoperability]]
- [[Compatibility Process]]
- [[Digital Twin Interop Protocol]]
- [[Digital Twin Synchronisation Bus]]
- [[Discovery Layer]]
- [[Interoperability]]
- [[Interoperability Framework]]
- [[Multiverse]]
- [[Persistence]]
- [[Platform Layer]]
- [[Portability]]
- [[Service Layer]]
- [[State Synchronization]]
- [[Universal Manifest]]
- [[WebXR API]]
- [[glTF (3D File Format)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Interoperability")]]
}
#+END_QUERY
```