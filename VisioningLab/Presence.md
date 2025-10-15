- ### OntologyBlock
  id:: presence-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: 20256
	- preferred-term:: Presence
	- definition:: Perceptual state in which a user feels located inside a virtual or mixed environment, experiencing spatial, social, and self presence.
	- maturity:: mature
	- source:: [[ACM]]
	- owl:class:: mv:Presence
	- owl:physicality:: VirtualEntity
	- owl:role:: Object
	- owl:inferred-class:: mv:VirtualObject
	- owl:functional-syntax:: true
	- belongsToDomain:: [[InteractionDomain]]
	- implementedInLayer:: [[ComputeLayer]]
	- #### Relationships
	  id:: presence-relationships
		- has-part:: [[Spatial Presence]], [[Social Presence]], [[Self Presence]]
		- is-part-of:: [[Immersive Experience]]
		- requires:: [[Sensory Feedback]], [[Haptic Device]], [[Visual Display]]
		- depends-on:: [[Latency]], [[Frame Rate]], [[Field of View]]
		- enables:: [[Immersion]], [[Engagement]], [[Embodiment]], [[Social Connection]]
	- #### OWL Axioms
	  id:: presence-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:Presence))

		  # Classification along two primary dimensions
		  SubClassOf(mv:Presence mv:VirtualEntity)
		  SubClassOf(mv:Presence mv:Object)

		  # Presence is a psychological construct that can be measured
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:hasMeasurableAttribute mv:PresenceMetric)
		  )

		  # Presence has distinct dimensional components
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:hasPart mv:SpatialPresence)
		  )

		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:hasPart mv:SocialPresence)
		  )

		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:hasPart mv:SelfPresence)
		  )

		  # Presence requires sensory feedback to manifest
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:requires mv:SensoryFeedback)
		  )

		  # Presence depends on technical performance factors
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:dependsOn mv:Latency)
		  )

		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:dependsOn mv:FrameRate)
		  )

		  # Presence enables immersive experiences
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:enables mv:Immersion)
		  )

		  # Presence is part of immersive experiences
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:isPartOf mv:ImmersiveExperience)
		  )

		  # Domain classification
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
		  )

		  # Layer classification
		  SubClassOf(mv:Presence
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:ComputeLayer)
		  )
		  ```
- ## About Presence
  id:: presence-about
	- Presence represents the psychological state where users perceive themselves to be located within a virtual or mixed environment rather than their physical surroundings. This fundamental concept in immersive experiences encompasses multiple dimensions of subjective experience that determine the quality and effectiveness of metaverse interactions.
	- ### Key Characteristics
	  id:: presence-characteristics
		- **Spatial Presence**: Feeling of "being there" in the virtual environment
		- **Social Presence**: Perception of co-location and interaction with other entities
		- **Self Presence**: Sense of embodiment and ownership of virtual representation
		- **Measurable Construct**: Quantifiable through questionnaires, behavioral metrics, and physiological responses
	- ### Technical Components
	  id:: presence-components
		- [[Spatial Presence]] - Sense of physical location within virtual space
		- [[Social Presence]] - Awareness and connection with other users or agents
		- [[Self Presence]] - Body ownership and agency over virtual avatar
		- [[Presence Metrics]] - Measurement instruments (IPQ, SUS, ITC-SOPI)
		- [[Sensory Feedback]] - Multisensory stimulation supporting presence illusion
	- ### Functional Capabilities
	  id:: presence-capabilities
		- **Immersion Enhancement**: Increases depth of engagement with virtual content
		- **Social Connection**: Facilitates meaningful interaction between remote users
		- **Embodiment**: Creates sense of inhabiting and controlling virtual body
		- **Suspension of Disbelief**: Reduces awareness of mediation and technology
		- **Emotional Engagement**: Enables authentic emotional responses to virtual events
	- ### Use Cases
	  id:: presence-use-cases
		- Virtual reality training simulations requiring realistic behavioral responses
		- Social VR platforms where authentic connection between users is critical
		- Therapeutic applications leveraging presence for exposure therapy
		- Remote collaboration requiring sense of co-location and shared workspace
		- Gaming experiences designed to maximize player immersion and engagement
		- Virtual events and conferences creating feeling of physical attendance
	- ### Standards & References
	  id:: presence-standards
		- [[ACM Metaverse Glossary]] - Standard terminology definition
		- [[ETSI GR ARF 010]] - Architectural framework for AR
		- [[IEEE P2733]] - Immersive experience standards
		- [[IPQ (iGroup Presence Questionnaire)]] - Measurement instrument
		- [[SUS (Slater-Usoh-Steed) Questionnaire]] - Presence assessment
		- Research: Slater & Wilbur (1997) "A Framework for Immersive Virtual Environments"
	- ### Related Concepts
	  id:: presence-related
		- [[Immersion]] - Technical capability that supports presence
		- [[Embodiment]] - Physical manifestation of presence experience
		- [[Avatar]] - Visual representation through which self-presence is experienced
		- [[Haptic Device]] - Technology providing tactile feedback supporting presence
		- [[Field of View]] - Visual parameter affecting spatial presence
		- [[VirtualObject]] - Ontology classification for psychological constructs
