# ETSI Domain: Governance & Compliance

## Properties
domain-type:: ETSI Functional Domain
term-count:: 27

## Terms in this Domain

- [[AI Governance Framework]]
- [[AI-Generated Content Disclosure]]
- [[Audit Trail]]
- [[Community Governance Model]]
- [[Compliance Audit Trail]]
- [[Cross-Platform Compliance Hub]]
- [[Decentralized Autonomous Organization (DAO)]]
- [[Digital Citizens’ Assembly]]
- [[Digital Constitution]]
- [[Digital Jurisdiction]]
- [[Digital Rights Management (Extended)]]
- [[Digital Taxonomy Registry]]
- [[Digital Well-Being Index]]
- [[Dispute Resolution Mechanism]]
- [[E-Contract Arbitration]]
- [[Environmental Impact Metric]]
- [[Environmental Sustainability Label]]
- [[Governance Model]]
- [[Metaverse Liability Model]]
- [[Metaverse Safety Protocol]]
- [[Policy Engine]]
- [[Social Impact Assessment (SIA)]]
- [[User Agreement Compliance]]
- [[User Consent Token]]
- [[Virtual Notary Service]]
- [[Virtual Property Right]]
- [[Virtual Securities Offering (VSO)]]

## Query All Terms
```clojure
#+BEGIN_QUERY
{:query [:find (pull ?p [*])
        :where
        [?p :block/properties ?props]
        [(get ?props :domain) ?d]
        [(clojure.string/includes? ?d "Governance & Compliance")]]
}
#+END_QUERY
```