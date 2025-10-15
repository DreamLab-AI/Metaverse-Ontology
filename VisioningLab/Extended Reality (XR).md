- ### OntologyBlock
  id:: extended-reality-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: 20245
	- preferred-term:: Extended Reality (XR)
	- definition:: Umbrella term encompassing all immersive technologies including Augmented Reality (AR), Virtual Reality (VR), and Mixed Reality (MR), representing the full spectrum from entirely physical to entirely virtual environments and all hybrid states between.
	- maturity:: mature
	- source:: [[ACM Glossary]], [[ISO 9241-940]]
	- owl:class:: mv:ExtendedReality
	- owl:physicality:: HybridEntity
	- owl:role:: Object
	- owl:inferred-class:: mv:HybridObject
	- owl:functional-syntax:: true
	- belongsToDomain:: [[InteractionDomain]]
	- implementedInLayer:: [[NetworkLayer]], [[ComputeLayer]]
	- #### Relationships
	  id:: extended-reality-relationships
		- has-part:: [[Virtual Reality (VR)]], [[Augmented Reality (AR)]], [[Mixed Reality (MR)]], [[Reality-Virtuality Continuum]]
		- is-part-of:: [[Spatial Computing]], [[Immersive Technology]]
		- requires:: [[Head-Mounted Display]], [[Spatial Tracking]], [[Real-Time Rendering]], [[Input Device]]
		- depends-on:: [[Computer Vision]], [[Graphics Processing]], [[Sensor Fusion]], [[Human-Computer Interaction]]
		- enables:: [[Immersive Experiences]], [[Spatial Interaction]], [[Presence]], [[Cross-Reality Transitions]]
		- binds-to:: [[Physical Environment]], [[Virtual Environment]]
	- #### OWL Axioms
	  id:: extended-reality-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:ExtendedReality))

		  # Classification along two primary dimensions
		  SubClassOf(mv:ExtendedReality mv:HybridEntity)
		  SubClassOf(mv:ExtendedReality mv:Object)

		  # Encompassing relationship to constituent technologies
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:hasPart mv:VirtualReality)
		  )
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:hasPart mv:AugmentedReality)
		  )
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:hasPart mv:MixedReality)
		  )

		  # Reality-virtuality continuum definition
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:hasPart mv:RealityVirtualityContinuum)
		  )

		  # Display system requirement
		  SubClassOf(mv:ExtendedReality
		    ObjectMinCardinality(1 mv:requires mv:DisplayDevice)
		  )

		  # Spatial tracking capability
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:requires mv:SpatialTracking)
		  )

		  # Domain classification
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
		  )

		  # Layer classification
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:NetworkLayer)
		  )
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:ComputeLayer)
		  )

		  # Immersive experience capability
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:enables mv:ImmersiveExperiences)
		  )

		  # Cross-reality transition support
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:enables mv:CrossRealityTransitions)
		  )

		  # Spatial interaction paradigm
		  SubClassOf(mv:ExtendedReality
		    ObjectSomeValuesFrom(mv:enables mv:SpatialInteraction)
		  )
		  ```
- ## About Extended Reality (XR)
  id:: extended-reality-about
	- Extended Reality (XR) serves as a comprehensive HybridObject category that encompasses the entire spectrum of immersive technologies. Rather than referring to a single technology, XR represents the continuum described by Milgram and Kishino's Reality-Virtuality Continuum, ranging from purely physical environments (reality) through Augmented Reality and Mixed Reality to purely digital environments (Virtual Reality). XR is increasingly used as an industry-wide term for discussing platforms, standards, and experiences that span multiple points on this continuum, recognizing that many applications fluidly transition between AR, MR, and VR modalities.
	- ### Key Characteristics
	  id:: extended-reality-characteristics
		- **Technology Agnostic**: Encompasses all immersive technologies regardless of physical-virtual balance
		- **Continuum Representation**: Treats reality and virtuality as endpoints of a continuous spectrum rather than discrete categories
		- **Modal Flexibility**: Supports applications that dynamically shift between AR, MR, and VR experiences
		- **Unified Development**: Enables common frameworks and APIs (like OpenXR) that work across the entire XR spectrum
		- **Cross-Reality Interoperability**: Facilitates seamless transitions and data sharing between different immersive modalities
	- ### Technical Components
	  id:: extended-reality-components
		- [[OpenXR Runtime]] - Cross-platform API standard supporting all XR modalities
		- [[Head-Mounted Display]] - VR headsets, AR glasses, or MR devices with varying pass-through capabilities
		- [[Spatial Tracking System]] - 6DOF inside-out or outside-in tracking for head and hand position
		- [[XR Interaction Toolkit]] - Unified input abstraction layer for controllers, hand tracking, and gaze
		- [[Reality-Virtuality Continuum]] - Conceptual framework from fully real to fully virtual environments
		- [[WebXR Device API]] - Browser-based XR experiences across AR, VR, and MR
		- [[Passthrough Display]] - Video or optical pass-through enabling variable reality-virtuality balance
	- ### Functional Capabilities
	  id:: extended-reality-capabilities
		- **Modality Switching**: Applications can dynamically adjust between AR overlay, MR interaction, and VR immersion
		- **Unified Input Handling**: Consistent interaction patterns across different XR modalities and devices
		- **Cross-Platform Development**: Write once, deploy across multiple XR devices with varying capabilities
		- **Progressive Immersion**: Gradually transition users from AR (low immersion) to VR (high immersion) as needed
		- **Spatial Persistence**: Maintain spatial anchors and content positioning across different XR modalities
		- **Accessibility Options**: Provide multiple reality-virtuality balance options to accommodate different user comfort levels
	- ### Use Cases
	  id:: extended-reality-use-cases
		- **Adaptive Training**: Start with AR instructions, transition to MR simulation, then full VR immersion for complex procedures
		- **Retail Experiences**: AR product browsing in physical stores, MR virtual try-on, VR showroom exploration
		- **Healthcare**: AR patient data overlay during examination, MR surgical planning, VR exposure therapy
		- **Education**: AR textbook augmentation, MR collaborative lab simulations, VR historical immersion
		- **Design Review**: AR on-site visualization, MR collaborative manipulation, VR full-scale walkthroughs
		- **Entertainment**: AR location-based games, MR tabletop experiences, VR fully immersive worlds
	- ### Standards & References
	  id:: extended-reality-standards
		- [[ISO 9241-940]] - Ergonomics of human-system interaction for XR systems
		- [[IEEE P2048-3]] - Unified standard for Virtual Reality and Augmented Reality device interoperability
		- [[ACM Metaverse Glossary]] - Standardized XR terminology definitions
		- [[OpenXR Specification]] - Royalty-free, open standard API for XR platforms and devices
		- [[WebXR Device API]] - W3C standard for accessing XR hardware through web browsers
		- [[Milgram's Reality-Virtuality Continuum]] - Foundational taxonomy defining the XR spectrum
		- [[ETSI GR ARF 010]] - European standards for AR/XR framework and architecture
	- ### Related Concepts
	  id:: extended-reality-related
		- [[Virtual Reality (VR)]] - XR modality representing fully immersive virtual environments
		- [[Augmented Reality (AR)]] - XR modality overlaying digital content on physical environment
		- [[Mixed Reality (MR)]] - XR modality with bidirectional physical-virtual interaction
		- [[Spatial Computing]] - Broader computing paradigm where XR provides primary user interfaces
		- [[Metaverse]] - Persistent virtual worlds often accessed through XR technologies
		- [[HybridObject]] - Ontology classification for XR as category spanning physical-virtual spectrum
