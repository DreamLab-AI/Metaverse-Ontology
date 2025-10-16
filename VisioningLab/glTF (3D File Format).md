- ### OntologyBlock
  id:: gltf-ontology
  collapsed:: true
	- metaverseOntology:: true
	- term-id:: 20110
	- preferred-term:: glTF (3D File Format)
	- definition:: A royalty-free, open-standard 3D asset transmission format developed by Khronos Group that efficiently specifies scene structure, geometry, materials, animations, and other properties for real-time rendering.
	- maturity:: mature
	- source:: [[Khronos Group]], [[EWG/MSF taxonomy]]
	- owl:class:: mv:glTFFormat
	- owl:physicality:: VirtualEntity
	- owl:role:: Object
	- owl:inferred-class:: mv:VirtualObject
	- owl:functional-syntax:: true
	- belongsToDomain:: [[CreativeMediaDomain]], [[InteractionDomain]]
	- implementedInLayer:: [[DataLayer]]
	- #### Relationships
	  id:: gltf-relationships
		- has-part:: [[Scene Graph]], [[Mesh Data]], [[Material Definition]], [[Texture References]], [[Animation Channels]], [[Binary Buffer]]
		- is-part-of:: [[3D Content Pipeline]], [[Asset Interchange System]]
		- requires:: [[JSON Schema]], [[Binary Encoding]], [[Graphics API]]
		- depends-on:: [[URI Specification]], [[Base64 Encoding]], [[MIME Types]]
		- enables:: [[3D Asset Exchange]], [[Runtime Rendering]], [[Cross-Platform Compatibility]], [[Content Interoperability]]
		- related-to:: [[USD]], [[FBX]], [[OBJ]], [[Collada]], [[WebXR]], [[Three.js]], [[Babylon.js]]
	- #### OWL Axioms
	  id:: gltf-owl-axioms
	  collapsed:: true
		- ```clojure
		  Declaration(Class(mv:glTFFormat))

		  # Classification along two primary dimensions
		  SubClassOf(mv:glTFFormat mv:VirtualEntity)
		  SubClassOf(mv:glTFFormat mv:Object)

		  # Format structure requirements
		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:containsSceneGraph mv:SceneGraph)
		  )

		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:specifiesGeometry mv:MeshData)
		  )

		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:definesMaterial mv:MaterialDefinition)
		  )

		  # Domain classifications
		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:belongsToDomain mv:CreativeMediaDomain)
		  )

		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:belongsToDomain mv:InteractionDomain)
		  )

		  # Layer classification
		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:implementedInLayer mv:DataLayer)
		  )

		  # Functional capabilities
		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:supportsAnimation mv:AnimationChannel)
		  )

		  SubClassOf(mv:glTFFormat
		    ObjectSomeValuesFrom(mv:enablesInteroperability mv:AssetExchange)
		  )
		  ```
- ## About glTF (3D File Format)
  id:: gltf-about
	- **glTF** (GL Transmission Format) is a modern, royalty-free specification for efficient transmission and loading of 3D scenes and models by applications. Often called the "JPEG of 3D," glTF minimizes both the size of 3D assets and the runtime processing needed to unpack and use those assets. The format was developed by the Khronos Group, the consortium behind OpenGL, Vulkan, and WebGL, specifically to address the need for a standardized, interoperable format optimized for real-time rendering across platforms.
	- Unlike legacy 3D formats designed for content creation workflows, glTF is engineered for efficient transmission over networks and fast loading by runtime engines. It uses JSON for describing scene structure with external references to binary geometry data and texture images. The format supports complete scene description including meshes, materials with PBR (Physically-Based Rendering) workflows, textures, skins, skeletal hierarchies, morph targets, animations, cameras, and lights.
	- glTF has become the de facto standard for 3D content delivery on the web and is increasingly adopted by game engines, AR/VR platforms, and metaverse applications. Major industry players including Meta, Microsoft, Adobe, Epic Games, Unity, and Google support glTF. The format's extensibility through official and vendor-specific extensions allows it to evolve with emerging technologies while maintaining backward compatibility.
	- ### Key Characteristics
	  id:: gltf-characteristics
		- **Compact Transmission**: Optimized binary format (GLB) minimizes file size and network transfer time
		- **Runtime Efficiency**: Scene data structured for direct consumption by graphics APIs without extensive processing
		- **Complete Scene Description**: Single format containing geometry, materials, animations, and scene hierarchy
		- **PBR Materials**: First-class support for physically-based rendering workflows using metallic-roughness or specular-glossiness
		- **Extensibility**: Official and vendor-specific extensions enable new features without breaking compatibility
		- **JSON Foundation**: Human-readable scene structure enables tooling, debugging, and automated processing
		- **Cross-Platform**: Works across web, mobile, desktop, VR/AR devices without platform-specific variants
		- **Open Standard**: Royalty-free specification maintained by Khronos with broad industry participation
	- ### Technical Components
	  id:: gltf-components
		- [[Scene Graph]] - Hierarchical node structure defining object relationships, transforms, and parent-child connections
		- [[Mesh Data]] - Vertex positions, normals, tangents, texture coordinates, and vertex colors in efficient binary buffers
		- [[Material Definition]] - PBR material properties including base color, metallic factor, roughness, normal maps, and emissive properties
		- [[Texture References]] - Image data references with sampler configuration for filtering and wrapping modes
		- [[Animation Channels]] - Keyframe animations targeting node transforms, morph weights, or material properties
		- [[Skin Definition]] - Skeletal animation data including joint hierarchies, inverse bind matrices, and vertex weights
		- [[Binary Buffer]] - Raw binary data containing geometry, animation, and other bulk data in efficient formats
		- [[Accessor Specification]] - Typed views into binary buffers defining data layout, stride, and component types
	- ### Functional Capabilities
	  id:: gltf-capabilities
		- **Asset Interoperability**: Enables seamless 3D content exchange between creation tools, game engines, and runtime platforms without lossy conversions
		- **Streaming Optimization**: Supports progressive loading with coarse meshes displaying first while detailed geometry and textures load asynchronously
		- **Animation Playback**: Provides skeletal animations, morph target animations, and property animations with interpolation modes and timing control
		- **PBR Rendering**: Delivers physically accurate materials that respond realistically to lighting across different rendering engines
		- **Extension Mechanism**: Allows applications to add custom data, compress textures (KTX2, Basis), add lighting (KHR_lights_punctual), or enable advanced features (KHR_materials_variants)
		- **Variant Support**: Enables single asset containing multiple material or LOD variants selectable at runtime for different quality or style requirements
		- **Metadata Embedding**: Stores asset provenance, licensing, authorship, and custom application-specific data within the format
		- **Validation Tooling**: Official validator ensures conformance to specification before deployment
	- ### Use Cases
	  id:: gltf-use-cases
		- **Web-based 3D Experiences**: Delivering 3D product visualizations for e-commerce, using Three.js or Babylon.js to render glTF models with interactive controls and AR preview
		- **AR Applications**: Mobile AR experiences displaying 3D furniture, products, or art in real-world spaces using glTF as interchange format between authoring tools and AR frameworks
		- **Metaverse Asset Exchange**: Transferring avatars, wearables, and virtual objects between metaverse platforms while preserving appearance and functionality
		- **Game Asset Pipelines**: Streamlining workflows where artists export glTF from Blender or Maya for direct import into Unity, Unreal, or Godot without conversion loss
		- **Digital Twin Visualization**: Rendering complex industrial equipment or architectural models in web-based digital twin platforms with accurate materials and animations
		- **Virtual Museums**: Displaying high-quality 3D scans of artifacts, sculptures, and historical objects in accessible web-based galleries
		- **Automotive Configurators**: Building car customization interfaces where users select colors, wheels, and features with real-time PBR rendering
		- **Medical Visualization**: Presenting anatomical models, surgical simulations, or medical device demonstrations in web applications
	- ### Standards & References
	  id:: gltf-standards
		- [[glTF 2.0 Specification]] - Core specification defining JSON schema, binary format, and required features
		- [[KHR_materials_pbrSpecularGlossiness]] - Extension providing alternative PBR workflow for legacy content
		- [[KHR_draco_mesh_compression]] - Extension enabling highly efficient mesh compression using Google Draco
		- [[KHR_texture_basisu]] - Extension supporting Basis Universal texture compression for GPU-efficient formats
		- [[EXT_meshopt_compression]] - Extension using meshoptimizer library for mesh data compression
		- [[KHR_lights_punctual]] - Extension adding point, spot, and directional lights to scenes
		- [[ISO/IEC 23090-3]] - ISO standard for 3D graphics asset interchange including glTF
		- [[MSF Interchange WG]] - Metaverse Standards Forum working group promoting glTF adoption for metaverse interoperability
		- [[glTF-Validator]] - Official validation tool ensuring conformance to specification
	- ### Related Concepts
	  id:: gltf-related
		- [[USD]] - Universal Scene Description format for complex production workflows with different optimization trade-offs
		- [[FBX]] - Autodesk proprietary format common in game development but less optimized for web delivery
		- [[WebXR]] - Web standard for immersive VR/AR experiences, commonly rendering glTF assets
		- [[Three.js]] - JavaScript 3D library with comprehensive glTF loader and rendering capabilities
		- [[Babylon.js]] - Web rendering engine with native glTF support and extension ecosystem
		- [[Blender]] - 3D creation suite with high-quality glTF import/export capabilities
		- [[Scene Graph]] - The hierarchical structure that glTF uses to organize 3D objects
		- [[PBR Material]] - Physically-based rendering approach that glTF materials implement
		- [[VirtualObject]] - The inferred ontology classification for glTF as a virtual, passive specification
