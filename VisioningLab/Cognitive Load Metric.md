- ### OntologyBlock
  id:: cognitive-load-metric-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: 20253
	- preferred-term:: Cognitive Load Metric
	- definition:: Quantitative measure of mental effort required during virtual interaction tasks, typically assessed using standardized scales like NASA-TLX.
	- maturity:: mature
	- source:: [[ISO 9241-112]]
	- owl:class:: mv:CognitiveLoadMetric
	- owl:physicality:: VirtualEntity
	- owl:role:: Object
	- owl:inferred-class:: mv:VirtualObject
	- owl:functional-syntax:: true
	- belongsToDomain:: [[InteractionDomain]]
	- implementedInLayer:: [[ApplicationLayer]], [[PresentationLayer]]
	- #### Relationships
	  id:: cognitive-load-metric-relationships
		- is-part-of:: [[User Experience Assessment]], [[Usability Testing]]
		- requires:: [[Measurement Framework]], [[Psychometric Scale]]
		- enables:: [[Performance Optimization]], [[Interface Design Validation]]
		- depends-on:: [[User Feedback]], [[Task Complexity Analysis]]
		- related-to:: [[Usability Metric]], [[Mental Workload Assessment]], [[NASA-TLX]]
	- #### OWL Axioms
	  id:: cognitive-load-metric-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:CognitiveLoadMetric))

		  # Classification along two primary dimensions
		  SubClassOf(mv:CognitiveLoadMetric mv:VirtualEntity)
		  SubClassOf(mv:CognitiveLoadMetric mv:Object)

		  # Measurement construct constraints
		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:quantifiesMentalEffort mv:CognitiveProcess)
		  )

		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:usesScale mv:PsychometricScale)
		  )

		  # Domain classification
		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
		  )

		  # Layer classification
		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:ApplicationLayer)
		  )

		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:PresentationLayer)
		  )

		  # Assessment relationships
		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:assessesAspectOf mv:UserInterface)
		  )

		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:informsDesignOf mv:InteractionPattern)
		  )

		  # Data dependencies
		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:derivedFrom mv:UserFeedback)
		  )

		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:requires mv:MeasurementFramework)
		  )

		  # Validation capabilities
		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:enables mv:PerformanceOptimization)
		  )

		  SubClassOf(mv:CognitiveLoadMetric
		    ObjectSomeValuesFrom(mv:enables mv:InterfaceDesignValidation)
		  )
		  ```
- ## About Cognitive Load Metric
  id:: cognitive-load-metric-about
	- Cognitive Load Metric is a standardized quantitative measurement construct used to assess the mental effort and workload experienced by users during virtual environment interactions. This metric provides objective data for evaluating interface usability, task complexity, and overall user experience quality.
	- ### Key Characteristics
	  id:: cognitive-load-metric-characteristics
		- Quantitative assessment of mental workload using validated psychometric scales
		- Multi-dimensional evaluation covering task load, temporal demand, and frustration
		- Standardized measurement protocols enabling cross-study comparisons
		- Real-time or post-task assessment capabilities
	- ### Technical Components
	  id:: cognitive-load-metric-components
		- [[NASA-TLX Scale]] - Six-dimension workload assessment instrument
		- [[Measurement Framework]] - Structured data collection and analysis protocols
		- [[Psychometric Scale]] - Validated rating instruments (e.g., 0-100 subjective scales)
		- [[Statistical Analysis Tools]] - Data processing and interpretation methods
	- ### Functional Capabilities
	  id:: cognitive-load-metric-capabilities
		- **Mental Effort Quantification**: Measures cognitive demand across task dimensions
		- **Usability Validation**: Provides objective data for interface design decisions
		- **Performance Prediction**: Correlates mental workload with task performance
		- **Comparative Analysis**: Enables benchmarking across different interface designs
	- ### Use Cases
	  id:: cognitive-load-metric-use-cases
		- VR interface usability testing comparing different navigation paradigms
		- AR training application assessment measuring learning curve progression
		- Metaverse platform evaluation quantifying onboarding complexity
		- Enterprise collaboration tool optimization based on cognitive load reduction
		- Accessibility evaluation ensuring interfaces accommodate diverse cognitive abilities
	- ### Standards & References
	  id:: cognitive-load-metric-standards
		- [[ISO 9241-112]] - Ergonomics of human-system interaction: Principles
		- [[IEEE 2733]] - XR usability and user experience evaluation
		- [[W3C XR Accessibility User Requirements]] - Cognitive accessibility guidelines
		- NASA-TLX (Task Load Index) - Validated workload assessment methodology
	- ### Related Concepts
	  id:: cognitive-load-metric-related
		- [[Usability Metric]] - Broader category of UX measurement constructs
		- [[Mental Workload Assessment]] - Parent concept in ergonomics domain
		- [[User Experience Assessment]] - Holistic evaluation framework
		- [[VirtualObject]] - Ontology classification as measurement construct
