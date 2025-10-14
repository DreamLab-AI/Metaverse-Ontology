- ### OntologyBlock
  id:: eye-tracking-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: 20152
	- preferred-term:: Eye Tracking
	- definition:: Physical sensor hardware that measures gaze direction, pupil dilation, and eye movements to enable foveated rendering, attention analytics, and natural interaction in XR devices.
	- maturity:: mature
	- source:: [[ACM]], [[ETSI GR ARF 010]]
	- owl:class:: mv:EyeTracking
	- owl:physicality:: PhysicalEntity
	- owl:role:: Object
	- owl:inferred-class:: mv:PhysicalObject
	- owl:functional-syntax:: true
	- belongsToDomain:: [[InteractionDomain]]
	- implementedInLayer:: [[EdgeLayer]]
	- #### Relationships
	  id:: eye-tracking-relationships
		- has-part:: [[Infrared Camera]], [[Infrared LED Illuminator]], [[Hot Mirror]], [[Image Sensor]], [[Pupil Detection Algorithm]], [[Calibration System]]
		- requires:: [[High-Speed Camera]], [[Infrared Light Source]], [[Optical Calibration Target]], [[Real-Time Processing Unit]], [[Low-Latency Data Bus]]
		- enables:: [[Foveated Rendering]], [[Gaze-Based Interaction]], [[Attention Analytics]], [[Vergence-Accommodation Matching]], [[Eye Gesture Control]]
		- depends-on:: [[XR Headset]], [[Graphics Processing Unit]], [[Head-Mounted Display]], [[Rendering Engine]]
		- is-part-of:: [[Perceptual Computing System]], [[Human-Computer Interaction Framework]]
	- #### OWL Axioms
	  id:: eye-tracking-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:EyeTracking))

		  # Classification along two primary dimensions
		  SubClassOf(mv:EyeTracking mv:PhysicalEntity)
		  SubClassOf(mv:EyeTracking mv:Object)

		  # Domain-specific constraints
		  SubClassOf(mv:EyeTracking
		    ObjectSomeValuesFrom(mv:hasPart mv:InfraredCamera)
		  )

		  SubClassOf(mv:EyeTracking
		    ObjectMinCardinality(2 mv:hasPart mv:InfraredCamera)
		  )

		  SubClassOf(mv:EyeTracking
		    ObjectSomeValuesFrom(mv:enables mv:FoveatedRendering)
		  )

		  # Domain classification
		  SubClassOf(mv:EyeTracking
		    ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
		  )

		  # Layer classification
		  SubClassOf(mv:EyeTracking
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:EdgeLayer)
		  )
		  ```
- ## About Eye Tracking
  id:: eye-tracking-about
	- **Eye Tracking** hardware is a precision opto-electronic system embedded in VR/AR headsets that continuously measures where users are looking, how their pupils respond to stimuli, and how their eyes move during immersive experiences. Using infrared cameras and illuminators, these sensors track the 3D gaze vector at 90-200 Hz, enabling performance-critical features like foveated rendering (which reduces GPU load by 50-70%) and intuitive gaze-based interfaces. Eye tracking represents a fundamental shift from controller-centric to attention-centric interaction paradigms.
	- ### Key Characteristics
	  id:: eye-tracking-characteristics
		- **High-Speed Capture**: Infrared cameras operating at 90-200 fps capture eye images with 3-5 ms latency for responsive interactions
		- **Dual-Camera Stereo Setup**: Separate cameras track each eye independently to compute accurate 3D gaze vectors and depth of focus
		- **Invisible Infrared Illumination**: 850 nm LED/VCSEL arrays illuminate eyes without visible light distraction or pupil constriction
		- **Sub-Degree Accuracy**: Achieves 0.5-1.0° gaze estimation accuracy after calibration, sufficient for foveated rendering zones
		- **Wide Tracking Range**: Maintains tracking across ±30° horizontal and ±20° vertical field of regard to cover natural eye movements
		- **Robustness to Glasses**: Advanced algorithms handle reflections and refraction from corrective lenses and contact lenses
	- ### Technical Components
	  id:: eye-tracking-components
		- [[Infrared Camera]] - Global shutter CMOS sensors (640×480+ pixels) with 850 nm bandpass filters capture eye images at 90-200 fps
		- [[Infrared LED Illuminator]] - Arrays of 850 nm LEDs or VCSELs provide controlled glint patterns (bright/dark pupil illumination)
		- [[Hot Mirror]] - Dichroic optical element reflects IR light to camera while transmitting visible light from display to user's eye
		- [[Image Sensor]] - High-sensitivity monochrome sensors with enhanced near-infrared quantum efficiency (>60% at 850 nm)
		- [[Pupil Detection Algorithm]] - Real-time computer vision pipelines detect pupil center, corneal reflections (glints), and iris boundaries
		- [[Calibration System]] - User-guided fixation on known screen targets establishes personalized gaze mapping models
		- [[Real-Time Processing Unit]] - Dedicated DSP or GPU compute shaders run eye detection at frame rate with <10 ms total latency
		- [[Low-Latency Data Bus]] - USB 3.0 or custom high-speed interfaces stream gaze coordinates to rendering engine at 200+ Hz
		- **Physical Installation**: Cameras mount on headset's internal frame near eye lenses (4-6 cm from cornea); IR illuminators positioned for specular reflection; hot mirrors integrate into optical path between display and eye; calibration performed per-user via 5-9 point screen targets
	- ### Functional Capabilities
	  id:: eye-tracking-capabilities
		- **Foveated Rendering**: Dynamically reduces peripheral resolution to 25-50% while maintaining full resolution in 5-10° foveal region, cutting GPU load 50-70%
		- **Gaze-Based Selection**: Users select UI elements, objects, or menu items by dwelling gaze for 300-800 ms (dwell time) or blinking
		- **Attention Heatmaps**: Records gaze fixations and saccades across virtual scenes for UX research, training assessment, and advertising analytics
		- **Vergence-Accommodation Matching**: Adjusts display focus depth to match binocular gaze convergence point, reducing VR sickness
		- **Pupillometry**: Measures pupil diameter changes (2-8 mm range) to infer cognitive load, emotional arousal, or lighting adaptation
		- **Eye Gesture Recognition**: Detects intentional blinks, winks, eye rolls as input commands for hands-free interaction
		- **Accessibility Features**: Enables gaze-based text input, navigation, and communication for users with motor impairments
	- ### Use Cases
	  id:: eye-tracking-use-cases
		- **High-Fidelity VR Gaming**: Flagship headsets use foveated rendering to deliver 4K-per-eye visuals on mobile GPUs without performance loss
		- **Enterprise Training Simulations**: Aviation, surgery, and manufacturing training tracks where trainees look to ensure critical steps are observed
		- **Marketing and Retail Analytics**: Brands analyze shopper gaze patterns in virtual stores to optimize product placement and packaging
		- **Automotive HMI Research**: Car manufacturers test dashboard layouts by recording driver gaze fixations during simulated driving
		- **Medical Diagnostics**: Ophthalmology apps detect eye diseases (glaucoma, diabetic retinopathy) via pupil response and saccade metrics
		- **Accessibility and Assistive Tech**: Eye-gaze keyboards and control systems empower users with ALS, spinal cord injuries, or cerebral palsy
		- **Social VR and Avatars**: Eye tracking data drives realistic avatar eye contact, gaze following, and non-verbal communication cues
	- ### Standards & References
	  id:: eye-tracking-standards
		- [[ETSI GR ARF 010]] - Augmented Reality Framework specifying eye tracking integration for XR systems
		- [[ISO 9241-960]] - Ergonomics of human-system interaction: Framework for tactile and haptic interactions (covers gaze interaction)
		- [[IEEE P2733]] - Clinical IoT Data and Device Interoperability (relevant for medical eye tracking)
		- **ACM ETRA Conference** - Eye Tracking Research and Applications symposium publishing latest algorithms and hardware
		- **OpenXR Eye Gaze Extension** - Khronos Group standard API for accessing eye tracking in XR applications
		- **Tobii Pro SDK** - Industry reference implementation for eye tracking data formats and calibration protocols
		- **GDPR Article 9** - Biometric data protections apply to eye tracking templates used for identification
	- ### Related Concepts
	  id:: eye-tracking-related
		- [[Foveated Rendering]] - GPU optimization technique directly enabled by real-time gaze tracking
		- [[XR Headset]] - Host device that integrates eye tracking sensors into near-eye display optics
		- [[Graphics Processing Unit]] - Rendering hardware that consumes gaze coordinates for foveation and variable rate shading
		- [[Biosensing Interface]] - Broader category of physiological sensors (eye tracking measures oculomotor physiology)
		- [[Gaze-Based Interaction]] - Interaction paradigm leveraging eye tracking for selection and navigation
		- [[Attention Analytics]] - Data science methods applied to gaze fixation recordings
		- [[Vergence-Accommodation Matching]] - Vision science principle guiding depth-adaptive display systems
		- [[PhysicalObject]] - Parent ontology class for tangible sensor hardware
