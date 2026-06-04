//! Official-schema integration tests for the IEEE 1685-2014 register path.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use ip_xact::v2014::types::{
    AbstractionDefPortConstraints, AbstractionDefinition, AbstractionPort, AbstractionPortStyle,
    AbstractionProtocolType, Abstractor, AbstractorGenerators, AbstractorInstance,
    AbstractorInstances, AbstractorInstantiations, AbstractorInterface, AbstractorInterfaces,
    AbstractorMode, AbstractorModeValue, AbstractorModel, AbstractorPort, AbstractorPortStyle,
    AbstractorPorts, AbstractorView, AbstractorViews, AbstractorWirePort, Access, AccessHandles,
    AccessViewRef, ActiveInterface, AdHocConnection, AdHocConnections, Assertion, Assertions,
    BankEntry, BitExpression, BitSteeringExpression, BuildCommand, BuildFlags, CellClass,
    CellFunction, CellFunctionValue, CellSpecification, CellStrength, ChainGenerator, ClockDriver,
    ClockEdge, ComponentGenerator, ComponentGeneratorSelector, ComponentGenerators,
    ComponentInstance, ComponentInstances, ConfigurableArray, ConfigurableArrays,
    ConfigurableElementValue, ConfigurableElementValues, ConstraintSet, ConstraintSetRef,
    ConstraintSets, Cpu, Cpus, DelayType, DelayUnit, Dependency, DesignConfiguration, Direction,
    DirectionValue, DriveConstraint, Driver, DriverKind, Drivers, Endianness, ExcludePort,
    ExcludePorts, ExecutableImage, ExportedName, ExternalPortReference, FileBuilder,
    FileSetFunction, FileSetRefGroup, FunctionArgument, FunctionDataType, FunctionReturnType,
    FunctionSourceFile, GeneratorApi, GeneratorApiType, GeneratorChain, GeneratorChainSelector,
    GeneratorGroup, GeneratorRef, GeneratorScope, GroupSelector, ImageType, IncludeFile,
    IndexedAccessHandle, IndexedAccessHandles, Indices, Initiative, InitiativeValue,
    Interconnection, InterconnectionConfiguration, Interconnections, InterfaceRef,
    InternalPortReference, LanguageFileBuilder, LanguageLinker, LanguageTools, LeafAccessHandle,
    LinkerCommandFile, LoadConstraint, LogicalName, ModuleParameter, ModuleParameterUsage,
    ModuleParameters, MonitorInterconnection, MonitorInterface, MultipleGroupSelectionOperator,
    NameValuePair, NonIndexedAccessHandles, NonIndexedLeafAccessHandle, OnSystem, OtherClockDriver,
    OtherClockDrivers, PartSelect, PathSegment, PathSegments, Payload, PayloadExtension,
    PayloadType, PortAccess, PortProtocolType, PortReferences, Presence, PresenceValue, Protocol,
    ProtocolTypeType, ProtocolTypeValue, Qualifier, RealExpression, RegisterDim, ResetType,
    ResetTypes, ServiceTypeDef, ServiceTypeName, SignedLongintExpression, SimpleAccessHandle,
    SimpleAccessHandles, SimplePortAccess, SingleShotDriver, Slice, Slices, StringExpression,
    SystemGroupName, SystemGroupNames, TiedValue, TimingConstraint, TransTypeDef, TransTypeDefs,
    TransactionalAbstraction, TransactionalPortMode, TransactionalTypeName, TransportMethods,
    TypeDefViewRef, TypeDefinition, TypeParameters, UnsignedBitVectorExpression,
    UnsignedIntExpression, UnsignedPositiveIntExpression, UnsignedPositiveLongintExpression,
    VendorExtension, VendorExtensions, ViewConfiguration, WhiteboxElement, WhiteboxElementRef,
    WhiteboxElementRefs, WhiteboxElements, WhiteboxType, WireAbstraction, WirePortDriver,
    WirePortMode, WireTypeDef, WireTypeDefinition, WireTypeDefs, WireTypeViewRef,
};
use ip_xact::v2014::types::{
    AbstractionType, AbstractionTypes, AbstractionViewRef, AddressBlock, AddressSpace,
    AddressSpaceRef, AddressSpaces, AlternateGroups, AlternateRegister, AlternateRegisters, Bank,
    BankAlignment, BankedAddressBlock, BankedBank, BankedSubspaceMap, BusDefinition, BusInterface,
    BusInterfaceMode, BusInterfaces, Catalog, Channel, ChannelBusInterfaceRef, Channels, Choice,
    ChoiceEnumeration, Choices, Component, ComponentInstantiation, ConfigurableLibraryRef, Design,
    DesignConfigurationInstantiation, DesignInstantiation, EnumeratedValue, EnumeratedValueUsage,
    EnumeratedValues, EnvironmentIdentifier, Field, File, FileSet, FileSetGroup, FileSetRef,
    FileSets, FileType, FileTypeValue, IndirectInterface, IndirectInterfaceTarget,
    IndirectInterfaces, Instantiation, Instantiations, IpxactFile, IpxactFiles, Language,
    LibraryRefType, LocalBank, LocalBankedBank, LocalMemoryMap, LocalMemoryMapEntry, LogicalPort,
    Master, MemoryMap, MemoryMapEntry, MemoryMaps, MemoryRemap, MemoryUsage, MirroredMaster,
    MirroredSlave, MirroredSlaveBaseAddresses, MirroredSystem, Model, ModifiedWriteValue,
    ModifiedWriteValueKind, Monitor, MonitoredInterfaceMode, NumericExpression, Parameter,
    ParameterFormat, ParameterPrefix, ParameterResolve, ParameterSign, ParameterUnit, Parameters,
    PhysicalPort, Port, PortConnection, PortDirection, PortInitiative, PortKind, PortMap,
    PortMapTarget, PortMaps, PortRange, PortStyle, PortVector, PortVectors, Ports, ReadAction,
    ReadActionKind, Register, RegisterData, RegisterFile, RemapAddress, RemapPort, RemapPorts,
    RemapState, RemapStates, Reset, Resets, Segment, Segments, Shared, Slave, SlaveFileSetRefGroup,
    SlaveTarget, StringURIExpression, SubspaceMap, System, TestConstraint, Testable,
    TransactionalPort, TransparentBridge, View, Views, WirePort, WriteValueConstraint,
    WriteValueConstraintChoice,
};

fn register_component() -> Component {
    let mut field = Field::new("ENABLE", "0", "1");
    field.id = Some("enable-field".into());
    field.access = Some(Access::ReadWrite);
    field.resets = Some(Resets {
        reset: vec![Reset::new("0")],
    });

    let mut register = Register::new("CONTROL", "0x0", "32");
    register.add_field(field);

    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.id = Some("register-block".into());
    block.usage = Some(MemoryUsage::Register);
    block.add_register(register);

    let mut map = MemoryMap::new("default");
    map.shared = Some(Shared::Yes);
    map.add_address_block(block);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });
    component
}

fn simple_register(name: &str, address_offset: &str) -> Register {
    let mut register = Register::new(name, address_offset, "32");
    register.add_field(Field::new("VALUE", "0", "32"));
    register
}

fn path_segments(path_segment: &str) -> PathSegments {
    PathSegments {
        path_segment: vec![PathSegment::new(path_segment)],
    }
}

fn indexed_access_handle(path_segment: &str) -> IndexedAccessHandles {
    IndexedAccessHandles {
        access_handle: vec![IndexedAccessHandle::new(path_segments(path_segment))],
    }
}

fn simple_access_handle(path_segment: &str) -> SimpleAccessHandles {
    SimpleAccessHandles {
        access_handle: vec![SimpleAccessHandle::new(path_segments(path_segment))],
    }
}

fn non_indexed_access_handle(path_segment: &str) -> NonIndexedAccessHandles {
    let slice = Slice::new(PathSegment::new(path_segment));
    NonIndexedAccessHandles {
        access_handle: vec![NonIndexedLeafAccessHandle::new(Slices {
            slice: vec![slice],
        })],
    }
}

fn bus_type() -> ConfigurableLibraryRef {
    ConfigurableLibraryRef::new("example.org", "buses", "apb", "1.0")
}

fn master_bus_interface(name: &str) -> BusInterface {
    BusInterface::new(
        name,
        bus_type(),
        BusInterfaceMode::Master(Master::default()),
    )
}

#[test]
fn serializes_register_component_with_2014_namespace_and_attributes() {
    let mut component = register_component();
    component.memory_maps.as_mut().unwrap().memory_map[0]
        .entries
        .iter_mut()
        .for_each(|entry| {
            if let ip_xact::v2014::types::MemoryMapEntry::AddressBlock(block) = entry {
                block.base_address.minimum = Some(0);
                block
                    .base_address
                    .extension_attributes
                    .insert("xmlns:irgen", "urn:irgen:test");
                block
                    .base_address
                    .extension_attributes
                    .insert("irgen:addressSource", "register-map");
                block
                    .range
                    .extension_attributes
                    .insert("xmlns:irgen", "urn:irgen:test");
                block
                    .range
                    .extension_attributes
                    .insert("irgen:rangeSource", "register-map");
            }
        });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");

    assert!(xml.starts_with("<ipxact:component"));
    assert!(xml.contains("xmlns:ipxact=\"http://www.accellera.org/XMLSchema/IPXACT/1685-2014\""));
    assert!(xml.contains("<ipxact:addressBlock xml:id=\"register-block\">"));
    assert!(xml.contains("irgen:addressSource=\"register-map\""));
    assert!(xml.contains("irgen:rangeSource=\"register-map\""));
    assert!(xml.contains("<ipxact:usage>register</ipxact:usage>"));
    assert!(xml.contains("<ipxact:addressOffset>0x0</ipxact:addressOffset>"));
    assert!(xml.contains("<ipxact:shared>yes</ipxact:shared>"));
    assert!(xml.contains("<ipxact:size>32</ipxact:size>"));
    assert!(xml.contains("<ipxact:field xml:id=\"enable-field\">"));
    assert!(xml.contains(
        "<ipxact:resets><ipxact:reset><ipxact:value>0</ipxact:value></ipxact:reset></ipxact:resets>"
    ));
    validate_xml("component-with-address-attributes", &xml);

    let parsed =
        Component::from_xml_str(&xml).expect("component should deserialize from its 2014 XML");
    let memory_map = &parsed
        .memory_maps
        .as_ref()
        .expect("component should retain memory maps")
        .memory_map[0];
    let ip_xact::v2014::types::MemoryMapEntry::AddressBlock(block) = &memory_map.entries[0] else {
        panic!("memory map should retain address block");
    };
    assert_eq!(
        block
            .base_address
            .extension_attributes
            .attributes
            .get("irgen:addressSource")
            .map(String::as_str),
        Some("register-map")
    );
    assert_eq!(
        block
            .range
            .extension_attributes
            .attributes
            .get("irgen:rangeSource")
            .map(String::as_str),
        Some("register-map")
    );
}

#[test]
fn register_component_validates_against_official_2014_xsd() {
    validate_xml(
        "component",
        &quick_xml::se::to_string(&register_component()).expect("component should serialize"),
    );
}

#[test]
fn register_component_roundtrips_through_2014_xml() {
    let component = register_component();
    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    let parsed =
        Component::from_xml_str(&xml).expect("component should deserialize from its 2014 XML");

    assert_eq!(parsed, component);
}

#[test]
fn component_description_validates_against_official_2014_xsd() {
    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.description = Some("Timer peripheral".into());

    validate_xml(
        "component-description",
        &quick_xml::se::to_string(&component).expect("component should serialize"),
    );
}

#[test]
fn component_parameter_choice_ref_validates_against_official_2014_xsd() {
    let mut choice = Choice::new("clock-frequency");
    let mut enumeration = ChoiceEnumeration::new("100");
    enumeration.text = Some("100 MHz".into());
    enumeration.help = Some("Select the timer input clock".into());
    enumeration
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    enumeration
        .extension_attributes
        .insert("irgen:displayRank", "primary");
    choice.add_enumeration(enumeration);
    choice.add_enumeration(ChoiceEnumeration::new("200"));

    let mut parameter = Parameter::new("CLOCK_FREQUENCY", "100");
    parameter.parameter_id = Some("clockFrequency".into());
    parameter.prompt = Some("Clock frequency".into());
    parameter.choice_ref = Some("clock-frequency".into());
    parameter.order = Some(1.0);
    parameter.config_groups = Some("timing".into());
    parameter.minimum = Some("100".into());
    parameter.maximum = Some("200".into());
    parameter.format = Some(ParameterFormat::Int);
    parameter.sign = Some(ParameterSign::Unsigned);
    parameter.prefix = Some(ParameterPrefix::Mega);
    parameter.unit = Some(ParameterUnit::Hertz);
    parameter.resolve = Some(ParameterResolve::User);
    parameter
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    parameter.extension_attributes.insert("irgen:ui", "slider");
    parameter
        .value
        .extension_attributes
        .insert("irgen:valueSource", "choice");
    parameter.vectors = Some(PortVectors {
        vector: vec![PortVector::new("31", "0")],
    });
    parameter.arrays = Some(ConfigurableArrays {
        array: vec![ConfigurableArray::new("3", "0")],
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.choices = Some(Choices {
        choice: vec![choice],
    });
    component.parameters = Some(Parameters {
        parameter: vec![parameter],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("component-parameter-choice-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    assert_eq!(
        parsed
            .choices
            .as_ref()
            .expect("component should retain choices")
            .choice[0]
            .enumeration[0]
            .extension_attributes
            .attributes
            .get("irgen:displayRank")
            .map(String::as_str),
        Some("primary")
    );
    let parsed_parameter = &parsed
        .parameters
        .as_ref()
        .expect("component should retain parameters")
        .parameter[0];
    assert_eq!(
        parsed_parameter
            .extension_attributes
            .attributes
            .get("irgen:ui")
            .map(String::as_str),
        Some("slider")
    );
    assert_eq!(
        parsed_parameter
            .value
            .extension_attributes
            .attributes
            .get("irgen:valueSource")
            .map(String::as_str),
        Some("choice")
    );
}

#[test]
fn component_file_set_ref_validates_against_official_2014_xsd() {
    let mut file = File::new(
        "rtl/timer.sv",
        FileType::new(FileTypeValue::SystemVerilogSource),
    );
    file.file_id = Some("timerRtl".into());

    let mut file_set = FileSet::new("rtlSources");
    file_set.group.push(FileSetGroup::new("rtl"));
    file_set.add_file(file);

    let mut instantiation = ComponentInstantiation::new("rtlComponent");
    let mut file_set_ref = FileSetRef::new("rtlSources");
    let mut file_set_ref_presence = BitExpression::new("true");
    file_set_ref_presence
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    file_set_ref_presence
        .extension_attributes
        .insert("irgen:condition", "rtl-enabled");
    file_set_ref.is_present = Some(file_set_ref_presence);
    instantiation.file_set_ref.push(file_set_ref);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.model = Some(Model {
        views: None,
        instantiations: Some(Instantiations {
            instantiation: vec![Instantiation::Component(instantiation)],
        }),
        ports: None,
    });
    component.file_sets = Some(FileSets {
        file_set: vec![file_set],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:condition=\"rtl-enabled\""));
    validate_xml("component-file-set-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let instantiation = &parsed
        .model
        .as_ref()
        .expect("component should retain model")
        .instantiations
        .as_ref()
        .expect("model should retain instantiations")
        .instantiation[0];
    let Instantiation::Component(component_instantiation) = instantiation else {
        panic!("expected component instantiation");
    };
    assert_eq!(
        component_instantiation.file_set_ref[0]
            .is_present
            .as_ref()
            .expect("fileSetRef should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        component_instantiation.file_set_ref[0]
            .is_present
            .as_ref()
            .expect("fileSetRef should retain isPresent")
            .extension_attributes
            .attributes
            .get("irgen:condition")
            .map(String::as_str),
        Some("rtl-enabled")
    );
}

#[test]
fn component_cpu_address_space_ref_validates_against_official_2014_xsd() {
    let mut cpu = Cpu::new("cpu0", AddressSpaceRef::new("cpuSpace"));
    cpu.is_present = Some(BitExpression::new("true"));
    cpu.parameters = Some(Parameters {
        parameter: vec![Parameter::new("HART_COUNT", "1")],
    });
    cpu.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:cpu").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut component = Component::new("example.org", "processors", "cpu", "1.0");
    component.address_spaces = Some(AddressSpaces {
        address_space: vec![AddressSpace::new("cpuSpace", "0x1000", "32")],
    });
    component.cpus = Some(Cpus { cpu: vec![cpu] });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("<ipxact:isPresent>true</ipxact:isPresent>"));
    assert!(xml.contains("acme:cpu"));
    validate_xml("component-cpu-address-space-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let cpu = &parsed
        .cpus
        .as_ref()
        .expect("component should retain CPUs")
        .cpu[0];
    assert_eq!(
        cpu.is_present
            .as_ref()
            .expect("cpu should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        cpu.vendor_extensions
            .as_ref()
            .expect("cpu should retain vendor extensions")
            .element[0]
            .name,
        "acme:cpu"
    );
}

#[test]
fn address_space_executable_image_validates_against_official_2014_xsd() {
    let mut image = ExecutableImage::new("firmwareImage", "firmware");
    image.image_type = Some("elf".into());
    image.parameters = Some(Parameters {
        parameter: vec![Parameter::new("LOAD_ADDRESS", "0x0")],
    });
    image.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:image").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    let mut image_builder = LanguageFileBuilder::new(FileType::new(FileTypeValue::CSource), "cc");
    let mut image_flags = StringExpression::new("-Os");
    image_flags
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    image_flags
        .extension_attributes
        .insert("irgen:stringSource", "image-builder");
    image_builder.flags = Some(image_flags);
    image_builder.replace_default_flags = Some(BitExpression::new("false"));
    image_builder.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:fileBuilder")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    let mut linker_command_file = LinkerCommandFile::new("link/timer.ld", "-T", "true");
    let mut generator_ref = GeneratorRef::new("firmwareGenerator");
    generator_ref.id = Some("firmware-generator-ref".into());
    linker_command_file.generator_ref.push(generator_ref);
    linker_command_file.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:linkerCommandFile")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    image.language_tools = Some(LanguageTools {
        file_builder: vec![image_builder],
        linker: Some(LanguageLinker::flags_with_command_file(
            "ld",
            "-nostdlib",
            linker_command_file,
        )),
    });
    image.file_set_ref_group = Some(FileSetRefGroup {
        file_set_ref: vec![FileSetRef::new("software")],
    });

    let mut address_space = AddressSpace::new("cpuSpace", "0x1000", "32");
    address_space.executable_image.push(image);
    address_space.parameters = Some(Parameters {
        parameter: vec![Parameter::new("ADDRESS_WIDTH", "32")],
    });

    let mut component = Component::new("example.org", "processors", "cpu", "1.0");
    component.address_spaces = Some(AddressSpaces {
        address_space: vec![address_space],
    });
    component.file_sets = Some(FileSets {
        file_set: vec![FileSet::new("software")],
    });
    let mut generator = ComponentGenerator::new("firmwareGenerator", "tools/build-firmware");
    generator.scope = Some(GeneratorScope::Entity);
    let mut phase = RealExpression::new("1.0");
    phase
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    phase
        .extension_attributes
        .insert("irgen:phaseSource", "firmware");
    generator.phase = Some(phase);
    generator.parameters = Some(Parameters {
        parameter: vec![Parameter::new("OPT_LEVEL", "s")],
    });
    generator.api_type = Some(GeneratorApi::new(GeneratorApiType::Tgi2014Base));
    generator.transport_methods = Some(TransportMethods::file());
    let mut generator_group = GeneratorGroup::new("firmware");
    generator_group.id = Some("firmware-generator-group".into());
    generator.group.push(generator_group);
    component.component_generators = Some(ComponentGenerators {
        component_generator: vec![generator],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:stringSource=\"image-builder\""));
    assert!(xml.contains("irgen:phaseSource=\"firmware\""));
    assert!(xml.contains(
        "<ipxact:generatorRef xml:id=\"firmware-generator-ref\">firmwareGenerator</ipxact:generatorRef>"
    ));
    assert!(
        xml.contains("<ipxact:group xml:id=\"firmware-generator-group\">firmware</ipxact:group>")
    );
    validate_xml("address-space-executable-image", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let image = &parsed
        .address_spaces
        .as_ref()
        .expect("component should retain address spaces")
        .address_space[0]
        .executable_image[0];
    assert_eq!(
        image
            .vendor_extensions
            .as_ref()
            .expect("executable image should retain vendor extensions")
            .element[0]
            .name,
        "acme:image"
    );
    let language_tools = image
        .language_tools
        .as_ref()
        .expect("image should retain language tools");
    assert_eq!(
        language_tools.file_builder[0]
            .vendor_extensions
            .as_ref()
            .expect("file builder should retain vendor extensions")
            .element[0]
            .name,
        "acme:fileBuilder"
    );
    assert_eq!(
        language_tools.file_builder[0]
            .flags
            .as_ref()
            .expect("file builder should retain flags")
            .extension_attributes
            .attributes
            .get("irgen:stringSource")
            .map(String::as_str),
        Some("image-builder")
    );
    let Some(LanguageLinker::Flags {
        linker,
        linker_flags,
        linker_command_file: Some(linker_command_file),
    }) = &language_tools.linker
    else {
        panic!("language tools should retain linker flags branch with command file");
    };
    assert_eq!(linker.value, "ld");
    assert_eq!(linker_flags.value, "-nostdlib");
    assert_eq!(
        linker_command_file.generator_ref[0].id.as_deref(),
        Some("firmware-generator-ref")
    );
    assert_eq!(
        linker_command_file.generator_ref[0].value,
        "firmwareGenerator"
    );
    assert_eq!(
        linker_command_file
            .vendor_extensions
            .as_ref()
            .expect("linker command file should retain vendor extensions")
            .element[0]
            .name,
        "acme:linkerCommandFile"
    );
    assert_eq!(
        parsed
            .component_generators
            .as_ref()
            .expect("component should retain generators")
            .component_generator[0]
            .phase
            .as_ref()
            .expect("generator should retain phase")
            .extension_attributes
            .attributes
            .get("irgen:phaseSource")
            .map(String::as_str),
        Some("firmware")
    );
    let parsed_generator = &parsed
        .component_generators
        .as_ref()
        .expect("component should retain generators")
        .component_generator[0];
    assert_eq!(
        parsed_generator.group[0].id.as_deref(),
        Some("firmware-generator-group")
    );
    assert_eq!(parsed_generator.group[0].value, "firmware");
}

#[test]
fn language_tools_rejects_linker_without_choice_branch() {
    let error = quick_xml::de::from_str::<LanguageTools>(
        "<languageTools><linker>ld</linker></languageTools>",
    )
    .expect_err("languageTools should reject linker without flags or command file");

    assert!(
        error
            .to_string()
            .contains("languageTools linker requires linkerFlags or linkerCommandFile")
    );
}

#[test]
fn instantiation_constraints_and_whitebox_refs_validate_against_official_2014_xsd() {
    let mut wire = WirePort::new(PortDirection::In);
    wire.vectors = Some(PortVectors {
        vector: vec![PortVector::new("7", "0")],
    });
    let mut wire_type_def = WireTypeDef::new("std_logic_vector");
    wire_type_def.id = Some("wire-rtl-type".into());
    wire_type_def.type_name.as_mut().unwrap().constrained = Some(true);
    let mut wire_type_definition = WireTypeDefinition::new("IEEE.std_logic_1164.all");
    wire_type_definition.id = Some("std-logic-package".into());
    wire_type_def.type_definition.push(wire_type_definition);
    let mut wire_type_view_ref = WireTypeViewRef::new("rtl");
    wire_type_view_ref.id = Some("wire-rtl-view-ref".into());
    wire_type_def.view_ref.push(wire_type_view_ref);
    wire.wire_type_defs = Some(WireTypeDefs {
        wire_type_def: vec![wire_type_def],
    });
    let mut default_driver = Driver::default_value("0");
    default_driver.range = Some(PortRange::new("7", "0"));
    let mut clock_driver = ClockDriver::new("10", "0", "1", "5");
    clock_driver.clock_name = Some("clk".into());
    clock_driver.clock_period.units = Some(DelayUnit::Nanoseconds);
    clock_driver
        .clock_period
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    clock_driver
        .clock_period
        .extension_attributes
        .insert("irgen:clockSource", "constraint");
    wire.drivers = Some(Drivers {
        driver: vec![
            default_driver,
            Driver::clock(clock_driver),
            Driver::single_shot(SingleShotDriver::new("1", "1", "2")),
        ],
    });
    let mut constraint_set = ConstraintSet::new("timing");
    constraint_set.vector = Some(PortRange::new("7", "0"));
    let mut drive_cell = CellSpecification::function(CellFunction::new(CellFunctionValue::Buffer));
    drive_cell.cell_strength = Some(CellStrength::Median);
    constraint_set.drive_constraint = Some(DriveConstraint::new(drive_cell));
    constraint_set.load_constraint =
        Some(LoadConstraint::new(CellSpecification::class(CellClass::Sequential)).with_count("2"));
    let mut timing = TimingConstraint::new(12.5, "clk");
    timing.clock_edge = Some(ClockEdge::Rise);
    timing.delay_type = Some(DelayType::Max);
    constraint_set.timing_constraint.push(timing);
    wire.constraint_sets = Some(ConstraintSets {
        constraint_set: vec![constraint_set],
    });

    let mut instantiation = ComponentInstantiation::new("rtlComponent");
    let mut default_builder = FileBuilder::new(FileType::new(FileTypeValue::SystemVerilogSource));
    default_builder.id = Some("rtl-default-builder".into());
    default_builder.command = Some(StringExpression::new("vlog"));
    default_builder.flags = Some(StringExpression::new("+acc"));
    default_builder.replace_default_flags = Some(BitExpression::new("false"));
    instantiation.default_file_builder.push(default_builder);
    let mut file_set_ref = FileSetRef::new("rtlFiles");
    file_set_ref.is_present = Some(BitExpression::new("true"));
    instantiation.file_set_ref.push(file_set_ref);
    instantiation
        .constraint_set_ref
        .push(ConstraintSetRef::new("timing"));
    instantiation.whitebox_element_refs = Some(WhiteboxElementRefs {
        whitebox_element_ref: vec![WhiteboxElementRef::new(
            "internalSignal",
            Slices {
                slice: vec![Slice::new(
                    PathSegment::new("internal_signal")
                        .with_index("1")
                        .with_index("0"),
                )],
            },
        )],
    });
    instantiation.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:componentInstantiation")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.file_sets = Some(FileSets {
        file_set: vec![FileSet::new("rtlFiles")],
    });
    let mut whitebox = WhiteboxElement::new("internalSignal", WhiteboxType::Signal);
    whitebox.is_present = Some(BitExpression::new("true"));
    whitebox.driveable = Some(true);
    whitebox.parameters = Some(Parameters {
        parameter: vec![Parameter::new("WB_WIDTH", "8")],
    });
    whitebox.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:whitebox").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    component.whitebox_elements = Some(WhiteboxElements {
        whitebox_element: vec![whitebox],
    });
    component.model = Some(Model {
        views: Some(Views {
            view: vec![View::new("rtl")],
        }),
        instantiations: Some(Instantiations {
            instantiation: vec![Instantiation::Component(instantiation)],
        }),
        ports: Some(Ports {
            port: vec![Port::new("trigger", PortStyle::Wire(wire))],
        }),
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:clockSource=\"constraint\""));
    assert!(xml.contains("<ipxact:wireTypeDef xml:id=\"wire-rtl-type\">"));
    assert!(xml.contains("<ipxact:typeDefinition xml:id=\"std-logic-package\">"));
    assert!(xml.contains("<ipxact:viewRef xml:id=\"wire-rtl-view-ref\">rtl</ipxact:viewRef>"));
    assert!(xml.contains("<ipxact:defaultFileBuilder xml:id=\"rtl-default-builder\">"));
    assert!(xml.contains("<ipxact:fileSetRef><ipxact:localName>rtlFiles</ipxact:localName>"));
    assert!(xml.contains("acme:componentInstantiation"));
    assert!(xml.contains("acme:whitebox"));
    validate_xml("instantiation-constraints-whitebox-refs", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let whitebox = &parsed
        .whitebox_elements
        .as_ref()
        .expect("component should retain whitebox elements")
        .whitebox_element[0];
    assert_eq!(
        whitebox
            .is_present
            .as_ref()
            .expect("whitebox should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(whitebox.driveable, Some(true));
    assert_eq!(
        whitebox
            .parameters
            .as_ref()
            .expect("whitebox should retain parameters")
            .parameter[0]
            .name,
        "WB_WIDTH"
    );
    assert_eq!(
        whitebox
            .vendor_extensions
            .as_ref()
            .expect("whitebox should retain vendor extensions")
            .element[0]
            .name,
        "acme:whitebox"
    );
    let PortStyle::Wire(wire) = &parsed
        .model
        .as_ref()
        .expect("component should retain model")
        .ports
        .as_ref()
        .expect("model should retain ports")
        .port[0]
        .style
    else {
        panic!("expected wire component port");
    };
    let vector = &wire
        .vectors
        .as_ref()
        .expect("wire port should retain vectors")
        .vector[0];
    assert_eq!(vector.left.value, "7");
    assert_eq!(vector.right.value, "0");
    let wire_type_def = &wire
        .wire_type_defs
        .as_ref()
        .expect("wire port should retain wire type definitions")
        .wire_type_def[0];
    assert_eq!(wire_type_def.id.as_deref(), Some("wire-rtl-type"));
    assert_eq!(
        wire_type_def.type_definition[0].id.as_deref(),
        Some("std-logic-package")
    );
    assert_eq!(
        wire_type_def.view_ref[0].id.as_deref(),
        Some("wire-rtl-view-ref")
    );
    let default_range = wire
        .drivers
        .as_ref()
        .expect("wire port should retain drivers")
        .driver[0]
        .range
        .as_ref()
        .expect("default driver should retain range");
    assert_eq!(default_range.left.value, "7");
    assert_eq!(default_range.right.value, "0");
    let Some(DriverKind::DefaultValue(default_value)) = &wire
        .drivers
        .as_ref()
        .expect("wire port should retain drivers")
        .driver[0]
        .kind
    else {
        panic!("default driver should retain unsigned bit-vector value");
    };
    assert_eq!(default_value.value, "0");
    let Some(DriverKind::Clock(clock_value)) = &wire
        .drivers
        .as_ref()
        .expect("wire port should retain drivers")
        .driver[1]
        .kind
    else {
        panic!("clock driver should retain unsigned bit-vector pulse value");
    };
    assert_eq!(clock_value.clock_pulse_value.value, "1");
    assert_eq!(
        clock_value
            .clock_period
            .extension_attributes
            .attributes
            .get("irgen:clockSource")
            .map(String::as_str),
        Some("constraint")
    );
    let Some(DriverKind::SingleShot(single_shot_value)) = &wire
        .drivers
        .as_ref()
        .expect("wire port should retain drivers")
        .driver[2]
        .kind
    else {
        panic!("single-shot driver should retain unsigned bit-vector value");
    };
    assert_eq!(single_shot_value.single_shot_value.value, "1");
    let constraint_range = wire
        .constraint_sets
        .as_ref()
        .expect("wire port should retain constraint sets")
        .constraint_set[0]
        .vector
        .as_ref()
        .expect("constraint set should retain vector");
    assert_eq!(constraint_range.left.value, "7");
    assert_eq!(constraint_range.right.value, "0");
    assert_eq!(
        wire.constraint_sets
            .as_ref()
            .expect("wire port should retain constraint sets")
            .constraint_set[0]
            .load_constraint
            .as_ref()
            .expect("constraint set should retain load constraint")
            .count
            .as_ref()
            .expect("load constraint should retain count")
            .value,
        "2"
    );
    let Instantiation::Component(instantiation) = &parsed
        .model
        .as_ref()
        .expect("component should retain model")
        .instantiations
        .as_ref()
        .expect("model should retain instantiations")
        .instantiation[0]
    else {
        panic!("expected component instantiation");
    };
    assert_eq!(
        instantiation.default_file_builder[0].id.as_deref(),
        Some("rtl-default-builder")
    );
    assert_eq!(
        instantiation.default_file_builder[0]
            .command
            .as_ref()
            .expect("default file builder should retain command")
            .value,
        "vlog"
    );
    assert_eq!(instantiation.file_set_ref[0].local_name, "rtlFiles");
    assert_eq!(
        instantiation
            .vendor_extensions
            .as_ref()
            .expect("component instantiation should retain vendor extensions")
            .element[0]
            .name,
        "acme:componentInstantiation"
    );
    let indices = &instantiation
        .whitebox_element_refs
        .as_ref()
        .expect("instantiation should retain whitebox refs")
        .whitebox_element_ref[0]
        .location[0]
        .slice[0]
        .path_segments
        .path_segment[0]
        .indices
        .as_ref()
        .expect("path segment should retain indices")
        .index;
    assert_eq!(indices[0].value, "1");
    assert_eq!(indices[1].value, "0");
}

#[test]
fn file_set_build_metadata_validates_against_official_2014_xsd() {
    let mut include_file = IncludeFile::new(true);
    include_file.external_declarations = Some(true);

    let mut logical_name = LogicalName::new("timer_lib");
    logical_name.default = Some(true);

    let mut build_flags = BuildFlags::new("-O2");
    build_flags.append = Some(true);
    build_flags
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    build_flags
        .extension_attributes
        .insert("irgen:flagsSource", "file");

    let mut file = File::new("sw/timer.c", FileType::new(FileTypeValue::CSource));
    file.is_present = Some(BitExpression::new("true"));
    file.is_structural = Some(false);
    file.is_include_file = Some(include_file);
    file.logical_name = Some(logical_name);
    file.exported_name.push(ExportedName::new("timer_init"));
    let mut build_command = StringExpression::new("cc");
    build_command
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    build_command
        .extension_attributes
        .insert("irgen:commandSource", "file");
    let mut target_name = StringURIExpression::new("build/timer.o");
    target_name
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    target_name
        .extension_attributes
        .insert("irgen:targetSource", "file");
    file.build_command = Some(BuildCommand {
        command: Some(build_command),
        flags: Some(build_flags),
        replace_default_flags: Some(BitExpression::new("false")),
        target_name: Some(target_name),
    });
    file.dependency.push(Dependency::new("include"));
    let mut define = NameValuePair::new("TIMER_CHANNELS", "4");
    define.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:defineMetadata")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    file.define.push(define);
    file.image_type.push(ImageType::new("firmware"));
    file.description = Some("Timer driver".into());
    file.extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    file.extension_attributes.insert("irgen:kind", "driver");
    file.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:fileMetadata")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut builder = FileBuilder::new(FileType::new(FileTypeValue::CSource));
    builder.command = Some(StringExpression::new("cc"));
    builder.flags = Some(StringExpression::new("-Wall"));
    builder.replace_default_flags = Some(BitExpression::new("false"));

    let mut file_set = FileSet::new("software");
    file_set.add_file(file);
    file_set.default_file_builder.push(builder);
    file_set.dependency.push(Dependency::new("include"));
    file_set.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:fileSetMetadata")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.file_sets = Some(FileSets {
        file_set: vec![file_set],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:commandSource=\"file\""));
    assert!(xml.contains("irgen:flagsSource=\"file\""));
    assert!(xml.contains("irgen:targetSource=\"file\""));
    validate_xml("file-set-build-metadata", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should parse");
    let parsed_file_set = &parsed
        .file_sets
        .as_ref()
        .expect("component should retain file sets")
        .file_set[0];
    assert_eq!(
        parsed_file_set.file[0]
            .is_present
            .as_ref()
            .expect("file should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        parsed_file_set
            .vendor_extensions
            .as_ref()
            .expect("file set should retain vendor extensions")
            .element[0]
            .name,
        "acme:fileSetMetadata"
    );
    assert_eq!(
        parsed_file_set.file[0]
            .extension_attributes
            .attributes
            .get("irgen:kind")
            .map(String::as_str),
        Some("driver")
    );
    assert_eq!(
        parsed_file_set.file[0]
            .vendor_extensions
            .as_ref()
            .expect("file should retain vendor extensions")
            .element[0]
            .name,
        "acme:fileMetadata"
    );
    assert_eq!(
        parsed_file_set.file[0].define[0]
            .vendor_extensions
            .as_ref()
            .expect("define should retain vendor extensions")
            .element[0]
            .name,
        "acme:defineMetadata"
    );
    let build_command = parsed_file_set.file[0]
        .build_command
        .as_ref()
        .expect("file should retain build command");
    assert_eq!(
        build_command
            .command
            .as_ref()
            .expect("build command should retain command")
            .extension_attributes
            .attributes
            .get("irgen:commandSource")
            .map(String::as_str),
        Some("file")
    );
    assert_eq!(
        build_command
            .target_name
            .as_ref()
            .expect("build command should retain target name")
            .extension_attributes
            .attributes
            .get("irgen:targetSource")
            .map(String::as_str),
        Some("file")
    );
    assert_eq!(
        build_command
            .flags
            .as_ref()
            .expect("build command should retain flags")
            .extension_attributes
            .attributes
            .get("irgen:flagsSource")
            .map(String::as_str),
        Some("file")
    );
}

#[test]
fn file_set_function_ref_validates_against_official_2014_xsd() {
    let mut file = File::new("sw/timer.c", FileType::new(FileTypeValue::CSource));
    file.file_id = Some("timerDriver".into());

    let mut function = FileSetFunction::new("timerDriver");
    function.replicate = Some(true);
    function.entry_point = Some("timer_init".into());
    function.return_type = Some(FunctionReturnType::Int);
    let mut argument = FunctionArgument::new("channel", "0", FunctionDataType::UnsignedInt);
    argument.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:argumentMetadata")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    function.argument.push(argument);
    function.disabled = Some(BitExpression::new("false"));
    function.source_file.push(FunctionSourceFile::new(
        "generated/timer_glue.c",
        FileType::new(FileTypeValue::CSource),
    ));

    let mut file_set = FileSet::new("software");
    file_set.add_file(file);
    file_set.function.push(function);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.file_sets = Some(FileSets {
        file_set: vec![file_set],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("acme:argumentMetadata"));
    validate_xml("file-set-function-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should parse");
    let argument = &parsed
        .file_sets
        .as_ref()
        .expect("component should retain file sets")
        .file_set[0]
        .function[0]
        .argument[0];
    assert_eq!(
        argument
            .vendor_extensions
            .as_ref()
            .expect("argument should retain vendor extensions")
            .element[0]
            .name,
        "acme:argumentMetadata"
    );
}

#[test]
fn catalog_validates_against_official_2014_xsd() {
    let mut catalog = Catalog::new(
        "example.org".into(),
        "peripherals".into(),
        "catalog".into(),
        "1.0".into(),
    );
    catalog.components = Some(IpxactFiles {
        ipxact_file: vec![IpxactFile::new(
            LibraryRefType::new("example.org", "peripherals", "timer", "1.0"),
            StringURIExpression::new("timer.xml"),
        )],
    });
    catalog.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:catalog").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml = quick_xml::se::to_string(&catalog).expect("catalog should serialize");

    assert!(xml.starts_with("<ipxact:catalog"));
    assert!(xml.contains("xmlns:ipxact=\"http://www.accellera.org/XMLSchema/IPXACT/1685-2014\""));
    assert!(xml.contains("<acme:catalog xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("catalog", &xml);

    let parsed = Catalog::from_xml_str(&xml).expect("catalog should deserialize from its 2014 XML");
    assert_eq!(parsed, catalog);
}

#[test]
fn bus_definition_validates_and_roundtrips_against_official_2014_xsd() {
    let mut group_name = SystemGroupName::new("system");
    group_name.id = Some("system-group".into());

    let mut max_masters = UnsignedIntExpression::new("4");
    max_masters.minimum = Some(1);

    let mut bus_definition = BusDefinition::new("example.org", "buses", "apb", "1.0", true, true);
    bus_definition.id = Some("apb-bus".into());
    bus_definition.broadcast = Some(false);
    bus_definition.extends = Some(LibraryRefType::new(
        "example.org",
        "buses",
        "base-bus",
        "1.0",
    ));
    bus_definition.max_masters = Some(max_masters);
    bus_definition.max_slaves = Some(UnsignedIntExpression::new("16"));
    bus_definition.system_group_names = Some(SystemGroupNames {
        system_group_name: vec![group_name],
    });
    bus_definition.description = Some("APB bus definition".into());
    bus_definition.parameters = Some(Parameters {
        parameter: vec![Parameter::new("ADDR_WIDTH", "32")],
    });
    bus_definition.assertions = Some(Assertions {
        assertion: vec![Assertion::new("validWidth", "true")],
    });
    bus_definition.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:bus").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml = quick_xml::se::to_string(&bus_definition).expect("bus definition should serialize");
    assert!(xml.starts_with("<ipxact:busDefinition"));
    assert!(xml.contains("<ipxact:directConnection>true</ipxact:directConnection>"));
    assert!(xml.contains("<ipxact:extends vendor=\"example.org\" library=\"buses\" name=\"base-bus\" version=\"1.0\"/>"));
    assert!(xml.contains("<acme:bus xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("bus-definition", &xml);

    let parsed =
        BusDefinition::from_xml_str(&xml).expect("bus definition should deserialize from its XML");
    assert_eq!(parsed, bus_definition);
    assert_eq!(
        parsed.vendor_extensions.unwrap().element[0].name,
        "acme:bus"
    );
}

#[test]
fn abstraction_definition_validates_and_roundtrips_against_official_2014_xsd() {
    let clock = AbstractionPort::wire(
        "CLK",
        WireAbstraction {
            qualifier: Some(Qualifier {
                is_clock: Some(true),
                ..Qualifier::default()
            }),
            on_master: Some(WirePortMode {
                presence: Some(Presence::new(PresenceValue::Required)),
                width: Some(UnsignedPositiveIntExpression::new("1")),
                direction: Some(Direction::new(DirectionValue::In)),
                ..WirePortMode::default()
            }),
            ..WireAbstraction::default()
        },
    );

    let constraints = AbstractionDefPortConstraints {
        timing_constraint: vec![TimingConstraint::new(25.0, "CLK")],
        load_constraint: Some(LoadConstraint::new(CellSpecification::class(
            CellClass::Combinational,
        ))),
        ..AbstractionDefPortConstraints::default()
    };
    let mut system_address = OnSystem::new("system");
    system_address.mode.presence = Some(Presence::new(PresenceValue::Required));
    system_address.mode.width = Some(UnsignedPositiveIntExpression::new("32"));
    system_address.mode.direction = Some(Direction::new(DirectionValue::Out));
    let mut address = AbstractionPort::wire(
        "PADDR",
        WireAbstraction {
            qualifier: Some(Qualifier {
                is_address: Some(true),
                ..Qualifier::default()
            }),
            on_system: vec![system_address],
            on_master: Some(WirePortMode {
                presence: Some(Presence::new(PresenceValue::Required)),
                width: Some(UnsignedPositiveIntExpression::new("32")),
                direction: Some(Direction::new(DirectionValue::Out)),
                mode_constraints: Some(constraints),
                ..WirePortMode::default()
            }),
            driver: Some(WirePortDriver::DefaultValue(
                UnsignedBitVectorExpression::new("0"),
            )),
            ..WireAbstraction::default()
        },
    );
    address.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:port").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut payload = Payload::new(PayloadType::Generic);
    payload.name = Some("TLM2".into());
    let mut extension = PayloadExtension::new("timer_extension");
    extension.mandatory = Some(true);
    payload.extension = Some(extension);
    let mut protocol = Protocol::new(AbstractionProtocolType::new(ProtocolTypeValue::Tlm));
    protocol.payload = Some(payload);
    let transaction = AbstractionPort::transactional(
        "TLM",
        TransactionalAbstraction {
            on_master: Some(TransactionalPortMode {
                presence: Some(Presence::new(PresenceValue::Optional)),
                initiative: Some(Initiative::new(InitiativeValue::Requires)),
                kind: Some(PortKind::new("tlm_socket")),
                bus_width: Some(UnsignedPositiveIntExpression::new("32")),
                protocol: Some(protocol),
            }),
            ..TransactionalAbstraction::default()
        },
    );

    let mut definition = AbstractionDefinition::new(
        "example.org",
        "buses",
        "apb-rtl",
        "1.0",
        LibraryRefType::new("example.org", "buses", "apb", "1.0"),
    );
    definition.id = Some("apb-rtl".into());
    definition.extends = Some(LibraryRefType::new(
        "example.org",
        "buses",
        "base-rtl",
        "1.0",
    ));
    definition.ports.add(clock);
    definition.ports.add(address);
    definition.ports.add(transaction);
    definition.parameters = Some(Parameters {
        parameter: vec![Parameter::new("ADDR_WIDTH", "32")],
    });
    definition.assertions = Some(Assertions {
        assertion: vec![Assertion::new("validWidth", "true")],
    });
    definition.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:abstraction")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml =
        quick_xml::se::to_string(&definition).expect("abstraction definition should serialize");
    assert!(xml.starts_with("<ipxact:abstractionDefinition"));
    assert!(
        xml.contains("<ipxact:timingConstraint clockName=\"CLK\">25</ipxact:timingConstraint>")
    );
    assert!(xml.contains("<ipxact:protocolType>tlm</ipxact:protocolType>"));
    assert!(xml.contains("<acme:port xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("abstraction-definition", &xml);

    let parsed = AbstractionDefinition::from_xml_str(&xml)
        .expect("abstraction definition should deserialize from its XML");
    assert_eq!(parsed, definition);
    let AbstractionPortStyle::Wire(address_wire) = &parsed.ports.port[1].style else {
        panic!("expected address wire abstraction");
    };
    assert_eq!(
        address_wire
            .on_master
            .as_ref()
            .expect("wire should retain master mode")
            .width
            .as_ref()
            .expect("wire mode should retain width")
            .value,
        "32"
    );
    let AbstractionPortStyle::Transactional(transactional) = &parsed.ports.port[2].style else {
        panic!("expected transactional abstraction");
    };
    assert_eq!(
        transactional
            .on_master
            .as_ref()
            .expect("transactional port should retain master mode")
            .bus_width
            .as_ref()
            .expect("transactional mode should retain busWidth")
            .value,
        "32"
    );
    assert_eq!(
        parsed.vendor_extensions.unwrap().element[0].name,
        "acme:abstraction"
    );
}

#[test]
fn generator_chain_validates_and_roundtrips_against_official_2014_xsd() {
    let mut grouped_chains = GroupSelector::new("base-chain");
    grouped_chains.add("shared-chain");
    grouped_chains.multiple_group_selection_operator = Some(MultipleGroupSelectionOperator::And);
    let mut group_chain_selector = GeneratorChainSelector::groups(grouped_chains);
    group_chain_selector.unique = Some(true);

    let mut referenced_chain =
        ConfigurableLibraryRef::new("example.org", "generators", "soc-build", "1.0");
    referenced_chain.configurable_element_values = Some(ConfigurableElementValues {
        configurable_element_value: vec![ConfigurableElementValue::new("output-dir", "build")],
    });

    let component_selector = ComponentGeneratorSelector::new(GroupSelector::new("component-rtl"));

    let mut generator = ChainGenerator::new("emit-register-header", "bin/emit-register-header");
    generator.id = Some("emit-header".into());
    generator.phase = Some(RealExpression::new("1.0"));
    generator.parameters = Some(Parameters {
        parameter: vec![Parameter::new("HEADER_NAME", "timer.h")],
    });
    generator.api_type = Some(GeneratorApi::new(GeneratorApiType::Tgi2014Extended));
    generator.transport_methods = Some(TransportMethods::file());
    generator.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:generator").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut mode = Choice::new("BUILD_MODE");
    mode.add_enumeration(ChoiceEnumeration::new("release"));

    let mut chain = GeneratorChain::new("example.org", "generators", "timer-build", "1.0");
    chain.hidden = Some(false);
    chain.id = Some("timer-build-chain".into());
    chain.add(group_chain_selector);
    chain.add(GeneratorChainSelector::chain(referenced_chain));
    chain.add(component_selector);
    chain.add(generator);
    chain.chain_group.push(GeneratorGroup::new("timer"));
    chain.choices = Some(Choices { choice: vec![mode] });
    chain.parameters = Some(Parameters {
        parameter: vec![Parameter::new("OUTPUT_DIR", "build")],
    });
    chain.assertions = Some(Assertions {
        assertion: vec![Assertion::new("validOutput", "true")],
    });
    chain.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:chain").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml = quick_xml::se::to_string(&chain).expect("generator chain should serialize");
    assert!(xml.starts_with("<ipxact:generatorChain"));
    assert!(xml.contains("<ipxact:generatorChainSelector unique=\"true\">"));
    assert!(xml.contains("<ipxact:componentGeneratorSelector>"));
    assert!(xml.contains("<ipxact:generatorExe>bin/emit-register-header</ipxact:generatorExe>"));
    assert!(xml.contains("<acme:generator xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("generator-chain", &xml);

    let parsed = GeneratorChain::from_xml_str(&xml)
        .expect("generator chain should deserialize from its XML");
    assert_eq!(parsed, chain);
    assert_eq!(
        parsed.vendor_extensions.unwrap().element[0].name,
        "acme:chain"
    );
}

#[test]
fn design_validates_and_roundtrips_against_official_2014_xsd() {
    let mut timer_ref = ConfigurableLibraryRef::new("example.org", "peripherals", "timer", "1.0");
    let mut timer_width = ConfigurableElementValue::new("timer-width", "32");
    timer_width
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    timer_width
        .extension_attributes
        .insert("irgen:source", "design");
    timer_ref.configurable_element_values = Some(ConfigurableElementValues {
        configurable_element_value: vec![timer_width],
    });

    let mut instances = ComponentInstances::default();
    instances.add(ComponentInstance::new("timer0", timer_ref));
    instances.add(ComponentInstance::new(
        "monitor0",
        ConfigurableLibraryRef::new("example.org", "verification", "apb-monitor", "1.0"),
    ));

    let mut source = ActiveInterface::new("timer0", "apb");
    source.exclude_ports = Some(ExcludePorts {
        exclude_port: vec![ExcludePort::new("debug")],
    });
    let mut interconnections = Interconnections::default();
    interconnections.add(Interconnection::new(
        "timer_bus",
        source,
        ActiveInterface::new("monitor0", "apb"),
    ));

    let mut monitor =
        MonitorInterconnection::new("timer_monitor", MonitorInterface::new("timer0", "apb"));
    let mut monitoring_interface = MonitorInterface::new("monitor0", "apb");
    monitoring_interface.is_present = Some(BitExpression::new("1"));
    monitor.monitor_interface.push(monitoring_interface);
    interconnections.add(monitor);

    let mut internal_port = InternalPortReference::new("timer0", "irq");
    internal_port.part_select = Some(PartSelect::range("0", "0"));
    let port_references = PortReferences {
        internal_port_reference: vec![internal_port],
        external_port_reference: vec![ExternalPortReference::new("irq")],
    };
    let mut irq = AdHocConnection::new("irq_connection", port_references);
    let mut tied_value = TiedValue::new("default");
    tied_value
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    tied_value
        .extension_attributes
        .insert("irgen:tie", "defaulted");
    irq.tied_value = Some(tied_value);

    let mut design = Design::new("example.org", "systems", "timer-system", "1.0");
    design.id = Some("timer-system-design".into());
    design.component_instances = Some(instances);
    design.interconnections = Some(interconnections);
    design.ad_hoc_connections = Some(AdHocConnections {
        ad_hoc_connection: vec![irq],
    });
    design.parameters = Some(Parameters {
        parameter: vec![Parameter::new("CLOCK_HZ", "50000000")],
    });
    design.assertions = Some(Assertions {
        assertion: vec![Assertion::new("validClock", "true")],
    });
    design.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:design").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml = quick_xml::se::to_string(&design).expect("design should serialize");
    assert!(xml.starts_with("<ipxact:design"));
    assert!(xml.contains("irgen:source=\"design\""));
    assert!(xml.contains("<ipxact:monitorInterconnection>"));
    assert!(xml.contains("<ipxact:internalPortReference componentRef=\"timer0\" portRef=\"irq\">"));
    assert!(xml.contains("<acme:design xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("design", &xml);

    let parsed = Design::from_xml_str(&xml).expect("design should deserialize from its XML");
    assert_eq!(parsed, design);
    assert_eq!(
        parsed.vendor_extensions.unwrap().element[0].name,
        "acme:design"
    );
    assert_eq!(
        parsed
            .component_instances
            .as_ref()
            .expect("design should retain component instances")
            .component_instance[0]
            .component_ref
            .configurable_element_values
            .as_ref()
            .expect("component ref should retain configurable element values")
            .configurable_element_value[0]
            .extension_attributes
            .attributes
            .get("irgen:source")
            .map(String::as_str),
        Some("design")
    );
    assert_eq!(
        parsed
            .ad_hoc_connections
            .as_ref()
            .expect("design should retain ad-hoc connections")
            .ad_hoc_connection[0]
            .tied_value
            .as_ref()
            .expect("ad-hoc connection should retain tied value")
            .extension_attributes
            .attributes
            .get("irgen:tie")
            .map(String::as_str),
        Some("defaulted")
    );
}

#[test]
fn design_configuration_validates_and_roundtrips_against_official_2014_xsd() {
    let mut generator_chain =
        ConfigurableLibraryRef::new("example.org", "generators", "system-build", "1.0");
    generator_chain.configurable_element_values = Some(ConfigurableElementValues {
        configurable_element_value: vec![ConfigurableElementValue::new("output-dir", "build")],
    });

    let mut abstractor_ref =
        ConfigurableLibraryRef::new("example.org", "abstractors", "apb-width-adapter", "1.0");
    abstractor_ref.configurable_element_values = Some(ConfigurableElementValues {
        configurable_element_value: vec![ConfigurableElementValue::new("target-width", "32")],
    });
    let mut abstractors = AbstractorInstances::default();
    abstractors
        .interface_ref
        .push(InterfaceRef::new("timer0", "apb"));
    abstractors.add(AbstractorInstance::new(
        "width_adapter0",
        abstractor_ref,
        "rtl",
    ));
    let mut interconnection = InterconnectionConfiguration::new("timer_bus");
    interconnection.abstractor_instances.push(abstractors);

    let mut view = ViewConfiguration::new("timer0", "rtl");
    view.view.configurable_element_values = Some(ConfigurableElementValues {
        configurable_element_value: vec![ConfigurableElementValue::new("timer-width", "32")],
    });

    let mut configuration =
        DesignConfiguration::new("example.org", "systems", "timer-system-config", "1.0");
    configuration.id = Some("timer-system-config".into());
    configuration.design_ref = Some(LibraryRefType::new(
        "example.org",
        "systems",
        "timer-system",
        "1.0",
    ));
    configuration
        .generator_chain_configuration
        .push(generator_chain);
    configuration
        .interconnection_configuration
        .push(interconnection);
    configuration.view_configuration.push(view);
    configuration.parameters = Some(Parameters {
        parameter: vec![Parameter::new("BUILD_MODE", "release")],
    });
    configuration.assertions = Some(Assertions {
        assertion: vec![Assertion::new("validMode", "true")],
    });
    configuration.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:designConfig")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml =
        quick_xml::se::to_string(&configuration).expect("design configuration should serialize");
    assert!(xml.starts_with("<ipxact:designConfiguration"));
    assert!(xml.contains("<ipxact:generatorChainConfiguration vendor=\"example.org\" library=\"generators\" name=\"system-build\" version=\"1.0\">"));
    assert!(xml.contains("<ipxact:abstractorInstance>"));
    assert!(xml.contains("<ipxact:view viewRef=\"rtl\">"));
    assert!(xml.contains("<acme:designConfig xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("design-configuration", &xml);

    let parsed = DesignConfiguration::from_xml_str(&xml)
        .expect("design configuration should deserialize from its XML");
    assert_eq!(parsed, configuration);
    assert_eq!(
        parsed.vendor_extensions.unwrap().element[0].name,
        "acme:designConfig"
    );
}

#[test]
fn register_file_validates_against_official_2014_xsd() {
    let mut register_file = RegisterFile::new("channel", "0x0", "4");
    register_file.access_handles = Some(indexed_access_handle("u_regs.channel"));
    register_file.is_present = Some(BitExpression::new("true"));
    register_file.dim = vec![RegisterDim::new("2")];
    register_file.type_identifier = Some("channelFile".into());
    register_file.parameters = Some(Parameters {
        parameter: vec![Parameter::new("CHANNEL_STRIDE", "4")],
    });
    register_file.add_register(simple_register("STATUS", "0x0"));

    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.access_handles = Some(non_indexed_access_handle("u_regs"));
    block.is_present = Some(BitExpression::new("true"));
    block.parameters = Some(Parameters {
        parameter: vec![Parameter::new("BLOCK_KIND", "registers")],
    });
    block
        .register_data
        .push(RegisterData::RegisterFile(register_file));

    let mut map = MemoryMap::new("default");
    map.add_address_block(block);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    validate_xml(
        "register-file",
        &quick_xml::se::to_string(&component).expect("component should serialize"),
    );
}

#[test]
fn alternate_register_validates_against_official_2014_xsd() {
    let mut alternate_register =
        AlternateRegister::new("CONTROL_DEBUG", AlternateGroups::new("DEBUG"));
    alternate_register.access_handles = Some(indexed_access_handle("u_regs.control_debug"));
    alternate_register.is_present = Some(BitExpression::new("true"));
    alternate_register.parameters = Some(Parameters {
        parameter: vec![Parameter::new("DEBUG_VIEW", "true")],
    });
    let mut alternate_field = Field::new("DEBUG_VALUE", "0", "32");
    alternate_field.access_handles = Some(non_indexed_access_handle("u_regs.control_debug_value"));
    alternate_field.is_present = Some(BitExpression::new("true"));
    alternate_register.add_field(alternate_field);

    let mut register = simple_register("CONTROL", "0x0");
    register.access_handles = Some(indexed_access_handle("u_regs.control"));
    register.is_present = Some(BitExpression::new("true"));
    register.dim = vec![RegisterDim::new("1")];
    register.parameters = Some(Parameters {
        parameter: vec![Parameter::new("RESET_DOMAIN", "debug")],
    });
    register.alternate_registers = Some(AlternateRegisters {
        alternate_register: vec![alternate_register],
    });

    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.add_register(register);

    let mut map = MemoryMap::new("default");
    map.add_address_block(block);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    validate_xml(
        "alternate-register",
        &quick_xml::se::to_string(&component).expect("component should serialize"),
    );
}

#[test]
fn bank_validates_against_official_2014_xsd() {
    let mut banked_block = BankedAddressBlock::new("channel", "4", "32");
    banked_block.access_handles = Some(non_indexed_access_handle("u_regs.channel"));
    banked_block.is_present = Some(BitExpression::new("true"));
    banked_block.is_volatile = Some(false);
    banked_block.parameters = Some(Parameters {
        parameter: vec![Parameter::new("BANKED_BLOCK", "channel")],
    });
    banked_block.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:bankedBlock")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    banked_block.add_register(simple_register("STATUS", "0x0"));

    let mut bank = Bank::new("channels", "0x0", BankAlignment::Serial);
    bank.access_handles = Some(simple_access_handle("u_regs.channels"));
    bank.is_present = Some(BitExpression::new("true"));
    bank.is_volatile = Some(true);
    bank.parameters = Some(Parameters {
        parameter: vec![Parameter::new("BANK_LAYOUT", "serial")],
    });
    bank.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:bank").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    bank.add_address_block(banked_block);

    let mut map = MemoryMap::new("default");
    map.add_bank(bank);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("bank", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let MemoryMapEntry::Bank(bank) = &parsed
        .memory_maps
        .as_ref()
        .expect("component should retain memory maps")
        .memory_map[0]
        .entries[0]
    else {
        panic!("expected bank");
    };
    assert_eq!(bank.bank_alignment, BankAlignment::Serial);
    assert_eq!(bank.base_address.value, "0x0");
    assert_eq!(bank.is_volatile, Some(true));
    let BankEntry::AddressBlock(banked_block) = &bank.entries[0] else {
        panic!("expected banked address block");
    };
    assert_eq!(banked_block.is_volatile, Some(false));
}

#[test]
fn nested_bank_validates_against_official_2014_xsd() {
    let mut banked_block = BankedAddressBlock::new("channel", "4", "32");
    banked_block.access_handles = Some(non_indexed_access_handle("u_regs.nested_channel"));
    banked_block.is_present = Some(BitExpression::new("true"));
    banked_block.is_volatile = Some(false);
    banked_block.parameters = Some(Parameters {
        parameter: vec![Parameter::new("NESTED_BLOCK", "channel")],
    });
    banked_block.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:nestedBlock")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    banked_block.add_register(simple_register("STATUS", "0x0"));

    let mut nested_bank = BankedBank::new("nested", BankAlignment::Serial);
    nested_bank.access_handles = Some(simple_access_handle("u_regs.nested"));
    nested_bank.is_present = Some(BitExpression::new("true"));
    nested_bank.is_volatile = Some(true);
    nested_bank.parameters = Some(Parameters {
        parameter: vec![Parameter::new("NESTED_BANK", "true")],
    });
    nested_bank.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:nestedBank")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    nested_bank.add_address_block(banked_block);

    let mut bank = Bank::new("channels", "0x0", BankAlignment::Serial);
    bank.add_bank(nested_bank);

    let mut map = MemoryMap::new("default");
    map.add_bank(bank);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("nested-bank", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let MemoryMapEntry::Bank(bank) = &parsed
        .memory_maps
        .as_ref()
        .expect("component should retain memory maps")
        .memory_map[0]
        .entries[0]
    else {
        panic!("expected bank");
    };
    let BankEntry::Bank(nested_bank) = &bank.entries[0] else {
        panic!("expected nested bank");
    };
    assert_eq!(nested_bank.is_volatile, Some(true));
    let BankEntry::AddressBlock(banked_block) = &nested_bank.entries[0] else {
        panic!("expected nested banked address block");
    };
    assert_eq!(banked_block.is_volatile, Some(false));
}

#[test]
fn subspace_map_and_master_interface_validate_against_official_2014_xsd() {
    let mut subspace = SubspaceMap::new("forwarded", "master", "0x1000");
    subspace.is_present = Some(BitExpression::new("true"));
    subspace.parameters = Some(Parameters {
        parameter: vec![Parameter::new("WINDOW", "forwarded")],
    });
    subspace.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:subspace").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut map = MemoryMap::new("default");
    map.add_subspace_map(subspace);

    let mut component = Component::new("example.org", "peripherals", "bridge", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![master_bus_interface("master")],
    });
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("subspace-map", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let MemoryMapEntry::SubspaceMap(subspace) = &parsed
        .memory_maps
        .as_ref()
        .expect("component should retain memory maps")
        .memory_map[0]
        .entries[0]
    else {
        panic!("expected subspace map");
    };
    assert_eq!(subspace.base_address.value, "0x1000");
}

#[test]
fn slave_memory_map_ref_validates_against_official_2014_xsd() {
    let mut slave = Slave::memory_map_ref("default");
    let mut file_set_ref_group = SlaveFileSetRefGroup::with_group("drivers");
    file_set_ref_group.id = Some("slave-driver-files".into());
    let mut file_set_ref = FileSetRef::new("software");
    file_set_ref.is_present = Some(BitExpression::new("true"));
    file_set_ref_group.add(file_set_ref);
    slave.add_file_set_ref_group(file_set_ref_group);
    let slave_interface = BusInterface::new("slave", bus_type(), BusInterfaceMode::Slave(slave));

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![slave_interface],
    });
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![MemoryMap::new("default")],
    });
    component.file_sets = Some(FileSets {
        file_set: vec![FileSet::new("software")],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("<ipxact:fileSetRefGroup xml:id=\"slave-driver-files\">"));
    assert!(xml.contains("<ipxact:group>drivers</ipxact:group>"));
    validate_xml("slave-memory-map-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let BusInterfaceMode::Slave(slave) = &parsed
        .bus_interfaces
        .as_ref()
        .expect("component should retain bus interfaces")
        .bus_interface[0]
        .mode
    else {
        panic!("bus interface should remain slave");
    };
    let Some(SlaveTarget::MemoryMapRef(memory_map_ref)) = &slave.target else {
        panic!("slave should retain memory map ref target");
    };
    assert_eq!(memory_map_ref.memory_map_ref, "default");
    let file_set_ref_group = &slave.file_set_ref_group[0];
    assert_eq!(file_set_ref_group.id.as_deref(), Some("slave-driver-files"));
    assert_eq!(file_set_ref_group.group.as_deref(), Some("drivers"));
    assert_eq!(file_set_ref_group.file_set_ref[0].local_name, "software");
    assert_eq!(
        file_set_ref_group.file_set_ref[0]
            .is_present
            .as_ref()
            .expect("fileSetRef should retain isPresent")
            .value,
        "true"
    );
}

#[test]
fn banked_subspace_map_validates_against_official_2014_xsd() {
    let mut bank = Bank::new("channels", "0x0", BankAlignment::Serial);
    let mut subspace = BankedSubspaceMap::new("master").with_name("forwarded");
    subspace.is_present = Some(BitExpression::new("true"));
    subspace.parameters = Some(Parameters {
        parameter: vec![Parameter::new("BANKED_WINDOW", "forwarded")],
    });
    subspace.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:bankedSubspace")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    bank.add_subspace_map(subspace);

    let mut map = MemoryMap::new("default");
    map.add_bank(bank);

    let mut component = Component::new("example.org", "peripherals", "bridge", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![master_bus_interface("master")],
    });
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    validate_xml(
        "banked-subspace-map",
        &quick_xml::se::to_string(&component).expect("component should serialize"),
    );
}

#[test]
fn transparent_bridge_validates_against_official_2014_xsd() {
    let slave = Slave::transparent_bridge(TransparentBridge {
        master_ref: "master".into(),
        id: Some("bridge".into()),
    });
    let slave_interface = BusInterface::new("slave", bus_type(), BusInterfaceMode::Slave(slave));

    let mut component = Component::new("example.org", "peripherals", "bridge", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![master_bus_interface("master"), slave_interface],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("transparent-bridge", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let BusInterfaceMode::Slave(slave) = &parsed
        .bus_interfaces
        .as_ref()
        .expect("component should retain bus interfaces")
        .bus_interface[1]
        .mode
    else {
        panic!("bus interface should remain slave");
    };
    let Some(SlaveTarget::TransparentBridges(bridges)) = &slave.target else {
        panic!("slave should retain transparent bridge target");
    };
    assert_eq!(bridges[0].master_ref, "master");
}

#[test]
fn remaining_bus_interface_modes_validate_against_official_2014_xsd() {
    let mut system_interface = BusInterface::new(
        "system",
        bus_type(),
        BusInterfaceMode::System(System::new("cpu")),
    );
    system_interface.is_present = Some(BitExpression::new("true"));
    system_interface.connection_required = Some(true);
    system_interface.bits_in_lau = Some(UnsignedPositiveLongintExpression::new("8"));
    let mut bit_steering = BitSteeringExpression::new("on");
    bit_steering
        .extension_attributes
        .insert("irgen:source", "spreadsheet");
    system_interface.bit_steering = Some(bit_steering);
    system_interface.endianness = Some(Endianness::Little);
    system_interface.parameters = Some(Parameters {
        parameter: vec![Parameter::new("DATA_WIDTH", "32")],
    });
    system_interface
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    system_interface
        .extension_attributes
        .insert("irgen:role", "control");

    let interfaces = vec![
        system_interface,
        BusInterface::new(
            "mirroredMaster",
            bus_type(),
            BusInterfaceMode::MirroredMaster(MirroredMaster::default()),
        ),
        BusInterface::new(
            "mirroredSystem",
            bus_type(),
            BusInterfaceMode::MirroredSystem(MirroredSystem::new("cpu")),
        ),
        BusInterface::new(
            "monitor",
            bus_type(),
            BusInterfaceMode::Monitor(Monitor::new(MonitoredInterfaceMode::Master)),
        ),
    ];

    let mut component = Component::new("example.org", "peripherals", "interfaces", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: interfaces,
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("remaining-interface-modes", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    assert_eq!(
        parsed
            .bus_interfaces
            .as_ref()
            .expect("component should retain bus interfaces")
            .bus_interface[0]
            .extension_attributes
            .attributes
            .get("irgen:role")
            .map(String::as_str),
        Some("control")
    );
    assert_eq!(
        parsed
            .bus_interfaces
            .as_ref()
            .expect("component should retain bus interfaces")
            .bus_interface[0]
            .bit_steering
            .as_ref()
            .expect("bus interface should retain bitSteering")
            .extension_attributes
            .attributes
            .get("irgen:source")
            .map(String::as_str),
        Some("spreadsheet")
    );
}

#[test]
fn abstraction_port_map_refs_validate_against_official_2014_xsd() {
    let mut abstraction = AbstractionType::new(ConfigurableLibraryRef::new(
        "example.org",
        "buses",
        "apb-rtl",
        "1.0",
    ));
    abstraction.view_ref.push(AbstractionViewRef::new("rtl"));
    let mut address_port_map = PortMap::new(
        LogicalPort::new("PADDR"),
        PortMapTarget::PhysicalPort(PhysicalPort::new("paddr")),
    );
    address_port_map.is_present = Some(BitExpression::new("true"));
    let mut ready_tie_off = UnsignedPositiveIntExpression::new("1");
    ready_tie_off
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    ready_tie_off
        .extension_attributes
        .insert("irgen:tieSource", "default");
    abstraction.port_maps = Some(PortMaps {
        port_map: vec![
            address_port_map,
            PortMap::new(
                LogicalPort::new("PREADY"),
                PortMapTarget::LogicalTieOff(ready_tie_off),
            ),
        ],
    });

    let mut interface = BusInterface::new(
        "slave",
        bus_type(),
        BusInterfaceMode::Slave(Slave::default()),
    );
    interface.abstraction_types = Some(AbstractionTypes {
        abstraction_type: vec![abstraction],
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![interface],
    });
    component.model = Some(Model {
        views: Some(Views {
            view: vec![View::new("rtl")],
        }),
        instantiations: None,
        ports: Some(Ports {
            port: vec![Port::new(
                "paddr",
                PortStyle::Wire(WirePort::new(PortDirection::In)),
            )],
        }),
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:tieSource=\"default\""));
    validate_xml("abstraction-port-map-refs", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    assert_eq!(
        parsed
            .bus_interfaces
            .as_ref()
            .expect("component should retain bus interfaces")
            .bus_interface[0]
            .abstraction_types
            .as_ref()
            .expect("bus interface should retain abstraction types")
            .abstraction_type[0]
            .port_maps
            .as_ref()
            .expect("abstraction type should retain port maps")
            .port_map[0]
            .is_present
            .as_ref()
            .expect("port map should retain isPresent")
            .value,
        "true"
    );
    let PortMapTarget::LogicalTieOff(tie_off) = &parsed
        .bus_interfaces
        .as_ref()
        .expect("component should retain bus interfaces")
        .bus_interface[0]
        .abstraction_types
        .as_ref()
        .expect("bus interface should retain abstraction types")
        .abstraction_type[0]
        .port_maps
        .as_ref()
        .expect("abstraction type should retain port maps")
        .port_map[1]
        .target
    else {
        panic!("port map should retain logical tie-off");
    };
    assert_eq!(tie_off.value, "1");
    assert_eq!(
        tie_off
            .extension_attributes
            .attributes
            .get("irgen:tieSource")
            .map(String::as_str),
        Some("default")
    );
}

#[test]
fn mirrored_slave_base_addresses_validate_against_official_2014_xsd() {
    let mut remap_address = RemapAddress::new("0x1000");
    remap_address.state = Some("LOW_POWER".into());
    remap_address.id = Some("low-power-base".into());

    let mirrored_slave = MirroredSlave {
        base_addresses: Some(MirroredSlaveBaseAddresses {
            remap_address: vec![remap_address],
            range: UnsignedPositiveLongintExpression::new("0x100"),
        }),
    };

    let mut component = Component::new("example.org", "peripherals", "interfaces", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![BusInterface::new(
            "mirroredSlave",
            bus_type(),
            BusInterfaceMode::MirroredSlave(mirrored_slave),
        )],
    });
    component.remap_states = Some(RemapStates {
        remap_state: vec![RemapState::new("LOW_POWER")],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains(
        "<ipxact:remapAddress state=\"LOW_POWER\" xml:id=\"low-power-base\">0x1000</ipxact:remapAddress>"
    ));
    assert!(xml.contains("<ipxact:range>0x100</ipxact:range>"));
    validate_xml("mirrored-slave-base-addresses", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let BusInterfaceMode::MirroredSlave(parsed_slave) = &parsed
        .bus_interfaces
        .as_ref()
        .expect("component should retain bus interfaces")
        .bus_interface[0]
        .mode
    else {
        panic!("bus interface should retain mirrored-slave mode");
    };
    let parsed_bases = parsed_slave
        .base_addresses
        .as_ref()
        .expect("mirrored slave should retain base addresses");
    assert_eq!(parsed_bases.remap_address[0].value, "0x1000");
    assert_eq!(
        parsed_bases.remap_address[0].state.as_deref(),
        Some("LOW_POWER")
    );
    assert_eq!(
        parsed_bases.remap_address[0].id.as_deref(),
        Some("low-power-base")
    );
    assert_eq!(parsed_bases.range.value, "0x100");
}

#[test]
fn channel_interface_refs_validate_against_official_2014_xsd() {
    let mut channel = Channel::new("mirrorConnection");
    channel.id = Some("mirror-channel".into());
    channel.display_name = Some("Mirror Connection".into());
    channel.description = Some("Mirrored master/slave connection".into());
    channel.is_present = Some(BitExpression::new("true"));
    let mut master_ref = ChannelBusInterfaceRef::new("mirroredMaster");
    master_ref.id = Some("mirror-master-ref".into());
    master_ref.is_present = Some(BitExpression::new("true"));
    let mut slave_ref = ChannelBusInterfaceRef::new("mirroredSlave");
    slave_ref.id = Some("mirror-slave-ref".into());
    slave_ref.is_present = Some(BitExpression::new("true"));
    channel.add_bus_interface_ref(master_ref);
    channel.add_bus_interface_ref(slave_ref);

    let mut component = Component::new("example.org", "peripherals", "interfaces", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![
            BusInterface::new(
                "mirroredMaster",
                bus_type(),
                BusInterfaceMode::MirroredMaster(MirroredMaster::default()),
            ),
            BusInterface::new(
                "mirroredSlave",
                bus_type(),
                BusInterfaceMode::MirroredSlave(MirroredSlave::default()),
            ),
        ],
    });
    component.channels = Some(Channels {
        channel: vec![channel],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("<ipxact:channel xml:id=\"mirror-channel\">"));
    assert!(xml.contains("<ipxact:displayName>Mirror Connection</ipxact:displayName>"));
    assert!(xml.contains("<ipxact:busInterfaceRef xml:id=\"mirror-master-ref\">"));
    assert!(xml.contains("<ipxact:busInterfaceRef xml:id=\"mirror-slave-ref\">"));
    validate_xml("channel-interface-refs", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let parsed_channel = &parsed
        .channels
        .as_ref()
        .expect("component should retain channels")
        .channel[0];
    assert_eq!(parsed_channel.id.as_deref(), Some("mirror-channel"));
    assert_eq!(
        parsed_channel.display_name.as_deref(),
        Some("Mirror Connection")
    );
    assert_eq!(
        parsed_channel.description.as_deref(),
        Some("Mirrored master/slave connection")
    );
    assert_eq!(
        parsed_channel
            .is_present
            .as_ref()
            .expect("channel should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        parsed_channel.bus_interface_ref[0].id.as_deref(),
        Some("mirror-master-ref")
    );
    assert_eq!(
        parsed_channel.bus_interface_ref[1].id.as_deref(),
        Some("mirror-slave-ref")
    );
    assert_eq!(
        parsed_channel.bus_interface_ref[0]
            .is_present
            .as_ref()
            .expect("channel busInterfaceRef should retain isPresent")
            .value,
        "true"
    );
}

#[test]
fn indirect_interface_refs_validate_against_official_2014_xsd() {
    let mut address = Field::new("ADDRESS", "0", "8");
    address.field_id = Some("address-field".into());
    let mut data = Field::new("DATA", "8", "8");
    data.field_id = Some("data-field".into());

    let mut register = Register::new("INDIRECT", "0x0", "32");
    register.add_field(address);
    register.add_field(data);

    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.add_register(register);

    let mut map = MemoryMap::new("default");
    map.add_address_block(block);

    let mut indirect = IndirectInterface::new("indirect", "address-field", "data-field", "default");
    indirect.bits_in_lau = Some(UnsignedPositiveLongintExpression::new("8"));
    indirect.endianness = Some(Endianness::Big);
    indirect.parameters = Some(Parameters {
        parameter: vec![Parameter::new("WINDOW_COUNT", "1")],
    });
    indirect.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:indirectInterface")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let indirect_bridge = IndirectInterface::new(
        "indirectBridge",
        "address-field",
        "data-field",
        IndirectInterfaceTarget::TransparentBridges(vec![
            TransparentBridge {
                master_ref: "masterA".into(),
                id: Some("bridge-a".into()),
            },
            TransparentBridge {
                master_ref: "masterB".into(),
                id: Some("bridge-b".into()),
            },
        ]),
    );

    let mut component = Component::new("example.org", "peripherals", "indirect", "1.0");
    component.indirect_interfaces = Some(IndirectInterfaces {
        indirect_interface: vec![indirect, indirect_bridge],
    });
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("indirect-interface-refs", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let parsed_indirect = &parsed
        .indirect_interfaces
        .as_ref()
        .expect("component should retain indirect interfaces")
        .indirect_interface[0];
    assert_eq!(
        parsed_indirect.target,
        IndirectInterfaceTarget::MemoryMapRef("default".into())
    );
    assert_eq!(
        parsed_indirect
            .bits_in_lau
            .as_ref()
            .expect("indirect interface should retain bitsInLau")
            .value,
        "8"
    );
    assert_eq!(
        parsed_indirect
            .endianness
            .as_ref()
            .expect("indirect interface should retain endianness"),
        &Endianness::Big
    );
    assert_eq!(
        parsed_indirect
            .parameters
            .as_ref()
            .expect("indirect interface should retain parameters")
            .parameter[0]
            .name,
        "WINDOW_COUNT"
    );
    assert_eq!(
        parsed_indirect
            .vendor_extensions
            .as_ref()
            .expect("indirect interface should retain vendor extensions")
            .element[0]
            .name,
        "acme:indirectInterface"
    );

    let parsed_bridge_indirect = &parsed
        .indirect_interfaces
        .as_ref()
        .expect("component should retain indirect interfaces")
        .indirect_interface[1];
    let IndirectInterfaceTarget::TransparentBridges(bridges) = &parsed_bridge_indirect.target
    else {
        panic!("indirect bridge should retain transparent bridge choice");
    };
    assert_eq!(bridges.len(), 2);
    assert_eq!(bridges[0].master_ref, "masterA");
    assert_eq!(bridges[1].master_ref, "masterB");
}

#[test]
fn address_space_and_master_ref_validate_against_official_2014_xsd() {
    let mut address_space = AddressSpace::new("cpu", "0x1000", "32");
    address_space.is_present = Some(BitExpression::new("true"));
    address_space.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:addressSpace")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    let mut segment = Segment::new("peripherals", "0x100", "0x100");
    segment.is_present = Some(BitExpression::new("true"));
    segment.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:segment").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    address_space.segments = Some(Segments {
        segment: vec![segment],
    });

    let mut address_space_ref = AddressSpaceRef::new("cpu");
    let mut base_address = SignedLongintExpression::new("-0x1000");
    base_address
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    base_address
        .extension_attributes
        .insert("irgen:signedSource", "master");
    address_space_ref.base_address = Some(base_address);
    let master = Master {
        address_space_ref: Some(address_space_ref),
    };

    let mut component = Component::new("example.org", "peripherals", "cpu", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![BusInterface::new(
            "master",
            bus_type(),
            BusInterfaceMode::Master(master),
        )],
    });
    component.address_spaces = Some(AddressSpaces {
        address_space: vec![address_space],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:signedSource=\"master\""));
    validate_xml("address-space-master-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let address_space = &parsed
        .address_spaces
        .as_ref()
        .expect("component should retain address spaces")
        .address_space[0];
    assert_eq!(
        address_space
            .is_present
            .as_ref()
            .expect("address space should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(address_space.range.value, "0x1000");
    assert_eq!(address_space.width.value, "32");
    assert_eq!(
        address_space
            .vendor_extensions
            .as_ref()
            .expect("address space should retain vendor extensions")
            .element[0]
            .name,
        "acme:addressSpace"
    );
    let segment = &address_space
        .segments
        .as_ref()
        .expect("address space should retain segments")
        .segment[0];
    assert_eq!(
        segment
            .is_present
            .as_ref()
            .expect("segment should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(segment.address_offset.value, "0x100");
    assert_eq!(segment.range.value, "0x100");
    assert_eq!(
        segment
            .vendor_extensions
            .as_ref()
            .expect("segment should retain vendor extensions")
            .element[0]
            .name,
        "acme:segment"
    );
    let BusInterfaceMode::Master(master) = &parsed
        .bus_interfaces
        .as_ref()
        .expect("component should retain bus interfaces")
        .bus_interface[0]
        .mode
    else {
        panic!("expected master bus interface");
    };
    assert_eq!(
        master
            .address_space_ref
            .as_ref()
            .expect("master should retain address space ref")
            .base_address
            .as_ref()
            .expect("address space ref should retain signed baseAddress")
            .value,
        "-0x1000"
    );
    assert_eq!(
        master
            .address_space_ref
            .as_ref()
            .expect("master should retain address space ref")
            .base_address
            .as_ref()
            .expect("address space ref should retain signed baseAddress")
            .extension_attributes
            .attributes
            .get("irgen:signedSource")
            .map(String::as_str),
        Some("master")
    );
}

#[test]
fn local_memory_map_validates_against_official_2014_xsd() {
    let mut block = AddressBlock::new("localRegisters", "0x0", "4", "32");
    block.add_register(simple_register("STATUS", "0x0"));

    let mut local_memory_map = LocalMemoryMap::new("local");
    local_memory_map.is_present = Some(BitExpression::new("true"));
    local_memory_map.add_address_block(block);

    let mut address_space = AddressSpace::new("cpu", "0x1000", "32");
    address_space.local_memory_map = Some(local_memory_map);

    let mut component = Component::new("example.org", "peripherals", "cpu", "1.0");
    component.address_spaces = Some(AddressSpaces {
        address_space: vec![address_space],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("local-memory-map", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    assert_eq!(
        parsed
            .address_spaces
            .as_ref()
            .expect("component should retain address spaces")
            .address_space[0]
            .local_memory_map
            .as_ref()
            .expect("address space should retain local memory map")
            .is_present
            .as_ref()
            .expect("local memory map should retain isPresent")
            .value,
        "true"
    );
}

#[test]
fn nested_local_bank_validates_against_official_2014_xsd() {
    let mut block = BankedAddressBlock::new("localRegisters", "4", "32");
    block.access_handles = Some(non_indexed_access_handle("u_cpu.local_registers"));
    block.is_present = Some(BitExpression::new("true"));
    block.parameters = Some(Parameters {
        parameter: vec![Parameter::new("LOCAL_BLOCK", "registers")],
    });
    block.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:localBlock")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    block.add_register(simple_register("STATUS", "0x0"));

    let mut nested_bank = LocalBankedBank::new("nested", BankAlignment::Serial);
    nested_bank.access_handles = Some(simple_access_handle("u_cpu.nested"));
    nested_bank.is_present = Some(BitExpression::new("true"));
    nested_bank.parameters = Some(Parameters {
        parameter: vec![Parameter::new("LOCAL_NESTED_BANK", "true")],
    });
    nested_bank.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:localNestedBank")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    nested_bank.add_address_block(block);

    let mut bank = LocalBank::new("channels", "0x0", BankAlignment::Serial);
    bank.access_handles = Some(simple_access_handle("u_cpu.channels"));
    bank.is_present = Some(BitExpression::new("true"));
    bank.parameters = Some(Parameters {
        parameter: vec![Parameter::new("LOCAL_BANK", "channels")],
    });
    bank.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:localBank").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    bank.add_bank(nested_bank);

    let mut local_memory_map = LocalMemoryMap::new("local");
    local_memory_map.add_bank(bank);

    let mut address_space = AddressSpace::new("cpu", "0x1000", "32");
    address_space.local_memory_map = Some(local_memory_map);

    let mut component = Component::new("example.org", "peripherals", "cpu", "1.0");
    component.address_spaces = Some(AddressSpaces {
        address_space: vec![address_space],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("nested-local-bank", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let local_bank = parsed
        .address_spaces
        .as_ref()
        .expect("component should retain address spaces")
        .address_space[0]
        .local_memory_map
        .as_ref()
        .expect("address space should retain local memory map")
        .entries
        .first()
        .and_then(|entry| match entry {
            LocalMemoryMapEntry::Bank(bank) => Some(bank.as_ref()),
            _ => None,
        })
        .expect("local bank should remain");
    assert_eq!(local_bank.bank_alignment, BankAlignment::Serial);
    assert_eq!(local_bank.base_address.value, "0x0");
}

#[test]
fn field_enumerations_and_write_constraint_validate_against_official_2014_xsd() {
    let mut field = Field::new("MODE", "0", "2");
    field.access = Some(Access::ReadWrite);
    let mut idle = EnumeratedValue::new("IDLE", "0");
    idle.usage = Some(EnumeratedValueUsage::Read);
    idle.value
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    idle.value
        .extension_attributes
        .insert("irgen:enumSource", "manual");
    let mut run = EnumeratedValue::new("RUN", "1");
    run.usage = Some(EnumeratedValueUsage::ReadWrite);
    field.enumerated_values = Some(EnumeratedValues {
        enumerated_value: vec![idle, run],
    });
    field.modified_write_value =
        Some(ModifiedWriteValue::new(ModifiedWriteValueKind::Modify).with_modify("customWrite"));
    field.write_value_constraint = Some(WriteValueConstraint::range("0", "3"));
    field.read_action = Some(ReadAction::new(ReadActionKind::Modify).with_modify("customRead"));
    let mut reset = Reset::new("0");
    reset.mask = Some(UnsignedBitVectorExpression::new("3"));
    field.resets = Some(Resets { reset: vec![reset] });
    field.testable = Some(Testable::new(true).with_constraint(TestConstraint::Restore));
    field.reserved = Some(BitExpression::new("false"));

    let mut mirror_field = Field::new("MIRROR", "2", "1");
    mirror_field.write_value_constraint = Some(WriteValueConstraint::write_as_read(true));

    let mut enum_only_field = Field::new("ENUM_ONLY", "3", "1");
    enum_only_field.write_value_constraint =
        Some(WriteValueConstraint::use_enumerated_values(true));

    let mut register = Register::new("CONTROL", "0x0", "32");
    register.add_field(field);
    register.add_field(mirror_field);
    register.add_field(enum_only_field);

    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.add_register(register);

    let mut map = MemoryMap::new("default");
    map.add_address_block(block);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:enumSource=\"manual\""));
    validate_xml("field-enumerations", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let parsed_register = parsed
        .memory_maps
        .as_ref()
        .expect("memory maps should remain")
        .memory_map
        .first()
        .expect("memory map should remain")
        .entries
        .first()
        .and_then(|entry| match entry {
            MemoryMapEntry::AddressBlock(block) => block.register_data.first(),
            _ => None,
        })
        .and_then(|entry| match entry {
            RegisterData::Register(register) => Some(register),
            _ => None,
        })
        .expect("register should remain");
    assert_eq!(parsed_register.address_offset.value, "0x0");
    assert_eq!(parsed_register.size.value, "32");

    let parsed_field = parsed_register.field.first().expect("field should remain");
    assert_eq!(parsed_field.bit_offset.value, "0");
    assert_eq!(parsed_field.bit_width.value, "2");
    assert_eq!(parsed_field.access, Some(Access::ReadWrite));
    let enumerated_values = parsed_field
        .enumerated_values
        .as_ref()
        .expect("field should retain enumerated values");
    assert_eq!(
        enumerated_values.enumerated_value[0].usage,
        Some(EnumeratedValueUsage::Read)
    );
    assert_eq!(
        enumerated_values.enumerated_value[1].usage,
        Some(EnumeratedValueUsage::ReadWrite)
    );
    let modified_write_value = parsed_field
        .modified_write_value
        .as_ref()
        .expect("field should retain modifiedWriteValue");
    assert_eq!(modified_write_value.value, ModifiedWriteValueKind::Modify);
    assert_eq!(modified_write_value.modify.as_deref(), Some("customWrite"));
    let read_action = parsed_field
        .read_action
        .as_ref()
        .expect("field should retain readAction");
    assert_eq!(read_action.value, ReadActionKind::Modify);
    assert_eq!(read_action.modify.as_deref(), Some("customRead"));
    let testable = parsed_field
        .testable
        .as_ref()
        .expect("field should retain testable policy");
    assert!(testable.value);
    assert_eq!(testable.test_constraint, Some(TestConstraint::Restore));
    let enumerations = parsed_field
        .enumerated_values
        .as_ref()
        .expect("field should retain enumerated values");
    assert_eq!(enumerations.enumerated_value[0].value.value, "0");
    assert_eq!(
        enumerations.enumerated_value[0]
            .value
            .extension_attributes
            .attributes
            .get("irgen:enumSource")
            .map(String::as_str),
        Some("manual")
    );
    assert_eq!(enumerations.enumerated_value[1].value.value, "1");
    let constraint = parsed_field
        .write_value_constraint
        .as_ref()
        .expect("field should retain write value constraint");
    let WriteValueConstraintChoice::Range(range) = &constraint.choice else {
        panic!("write constraint should retain range choice");
    };
    assert_eq!(range.minimum.value, "0");
    assert_eq!(range.maximum.value, "3");
    let reset = &parsed_field
        .resets
        .as_ref()
        .expect("field should retain resets")
        .reset[0];
    assert_eq!(reset.value.value, "0");
    assert_eq!(
        reset.mask.as_ref().expect("reset should retain mask").value,
        "3"
    );
    assert_eq!(
        parsed_field
            .reserved
            .as_ref()
            .expect("field reserved flag should remain")
            .value,
        "false"
    );

    let mirror_constraint = parsed_register.field[1]
        .write_value_constraint
        .as_ref()
        .expect("mirror field should retain write constraint");
    assert!(matches!(
        mirror_constraint.choice,
        WriteValueConstraintChoice::WriteAsRead { value: true }
    ));

    let enum_only_constraint = parsed_register.field[2]
        .write_value_constraint
        .as_ref()
        .expect("enum-only field should retain write constraint");
    assert!(matches!(
        enum_only_constraint.choice,
        WriteValueConstraintChoice::UseEnumeratedValues { value: true }
    ));
}

#[test]
fn memory_remap_validates_against_official_2014_xsd() {
    let mut remapped_block = AddressBlock::new("lowPowerRegisters", "0x1000", "4", "32");
    remapped_block.add_register(simple_register("STATUS", "0x0"));

    let mut memory_remap = MemoryRemap::new("lowPower", "LOW_POWER");
    memory_remap.id = Some("low-power-remap".into());
    memory_remap.display_name = Some("Low Power Remap".into());
    memory_remap.description = Some("Alternate map selected in low-power state".into());
    memory_remap.is_present = Some(BitExpression::new("true"));
    memory_remap.add_address_block(remapped_block);

    let mut map = MemoryMap::new("default");
    map.is_present = Some(BitExpression::new("true"));
    map.add_memory_remap(memory_remap);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.remap_states = Some(RemapStates {
        remap_state: vec![RemapState::new("LOW_POWER")],
    });
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("<ipxact:memoryRemap state=\"LOW_POWER\" xml:id=\"low-power-remap\">"));
    assert!(xml.contains("<ipxact:displayName>Low Power Remap</ipxact:displayName>"));
    validate_xml("memory-remap", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let map = &parsed
        .memory_maps
        .as_ref()
        .expect("component should retain memory maps")
        .memory_map[0];
    assert_eq!(
        map.is_present
            .as_ref()
            .expect("memory map should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(map.memory_remap[0].id.as_deref(), Some("low-power-remap"));
    assert_eq!(
        map.memory_remap[0].display_name.as_deref(),
        Some("Low Power Remap")
    );
    assert_eq!(
        map.memory_remap[0].description.as_deref(),
        Some("Alternate map selected in low-power state")
    );
    assert_eq!(
        map.memory_remap[0]
            .is_present
            .as_ref()
            .expect("memory remap should retain isPresent")
            .value,
        "true"
    );
}

#[test]
fn wire_port_and_remap_port_ref_validate_against_official_2014_xsd() {
    let mut wire = WirePort::new(PortDirection::In);
    wire.vectors = Some(PortVectors {
        vector: vec![PortVector::new("7", "0")],
    });

    let mut remap_state = RemapState::new("LOW_POWER");
    remap_state.display_name = Some("Low Power".into());
    remap_state.description = Some("Low-power remap selected by control port".into());
    let mut remap_port = RemapPort::new("low_power", "1");
    let mut port_index = UnsignedIntExpression::new("0");
    port_index
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    port_index
        .extension_attributes
        .insert("irgen:indexSource", "remap");
    remap_port.port_index = Some(port_index);
    remap_port
        .value
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    remap_port
        .value
        .extension_attributes
        .insert("irgen:valueSource", "remap");
    remap_state.remap_ports = Some(RemapPorts {
        remap_port: vec![remap_port],
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.remap_states = Some(RemapStates {
        remap_state: vec![remap_state],
    });
    component.model = Some(Model {
        ports: Some(Ports {
            port: vec![Port::new("low_power", PortStyle::Wire(wire))],
        }),
        ..Model::default()
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:indexSource=\"remap\""));
    assert!(xml.contains("<ipxact:displayName>Low Power</ipxact:displayName>"));
    assert!(xml.contains("irgen:valueSource=\"remap\""));
    validate_xml("wire-port-remap-ref", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let remap_state = &parsed
        .remap_states
        .as_ref()
        .expect("component should retain remap states")
        .remap_state[0];
    assert_eq!(remap_state.display_name.as_deref(), Some("Low Power"));
    assert_eq!(
        remap_state.description.as_deref(),
        Some("Low-power remap selected by control port")
    );
    let parsed_remap_port = &parsed
        .remap_states
        .as_ref()
        .expect("component should retain remap states")
        .remap_state[0]
        .remap_ports
        .as_ref()
        .expect("remap state should retain remap ports")
        .remap_port[0];
    assert_eq!(
        parsed_remap_port
            .port_index
            .as_ref()
            .expect("remap port should retain portIndex")
            .value,
        "0"
    );
    assert_eq!(
        parsed_remap_port
            .port_index
            .as_ref()
            .expect("remap port should retain portIndex")
            .extension_attributes
            .attributes
            .get("irgen:indexSource")
            .map(String::as_str),
        Some("remap")
    );
    assert_eq!(parsed_remap_port.value.value, "1");
    assert_eq!(
        parsed_remap_port
            .value
            .extension_attributes
            .attributes
            .get("irgen:valueSource")
            .map(String::as_str),
        Some("remap")
    );
}

#[test]
fn port_arrays_and_access_handles_validate_against_official_2014_xsd() {
    let mut slice = Slice::new(PathSegment::new("debug_bus").with_index("1"));
    slice.range = Some(PortRange::new("7", "0"));

    let mut handle = LeafAccessHandle::new(Slices { slice: vec![slice] });
    handle.force = Some(false);
    handle.view_ref.push(AccessViewRef::new("rtl"));
    handle.indices = Some(Indices {
        index: vec![
            UnsignedIntExpression::new("1"),
            UnsignedIntExpression::new("0"),
        ],
    });

    let mut port = Port::new("debug", PortStyle::Wire(WirePort::new(PortDirection::In)));
    port.is_present = Some(BitExpression::new("true"));
    port.arrays = Some(ConfigurableArrays {
        array: vec![
            ConfigurableArray::new("1", "0"),
            ConfigurableArray::new("3", "0"),
        ],
    });
    port.access = Some(PortAccess {
        port_access_type: Some(SimplePortAccess::Pointer),
        access_handles: Some(AccessHandles {
            access_handle: vec![handle],
        }),
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.model = Some(Model {
        views: Some(Views {
            view: vec![View::new("rtl")],
        }),
        ports: Some(Ports { port: vec![port] }),
        ..Model::default()
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    validate_xml("port-arrays-access-handles", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    let parsed_port = &parsed
        .model
        .as_ref()
        .expect("component should retain model")
        .ports
        .as_ref()
        .expect("model should retain ports")
        .port[0];
    let arrays = &parsed_port
        .arrays
        .as_ref()
        .expect("port should retain configurable arrays")
        .array;
    assert_eq!(arrays[0].left.value, "1");
    assert_eq!(arrays[0].right.value, "0");
    assert_eq!(arrays[1].left.value, "3");
    assert_eq!(arrays[1].right.value, "0");
    let indices = &parsed_port
        .access
        .as_ref()
        .expect("port should retain access")
        .access_handles
        .as_ref()
        .expect("port access should retain access handles")
        .access_handle[0]
        .indices
        .as_ref()
        .expect("access handle should retain indices")
        .index;
    assert_eq!(indices[0].value, "1");
    assert_eq!(indices[1].value, "0");
}

#[test]
fn component_root_clock_reset_and_assertions_validate_against_official_2014_xsd() {
    let mut reset = Reset::new("0");
    reset.id = Some("soft-reset-value".into());
    reset.reset_type_ref = Some("SOFT".into());
    reset.mask = Some(UnsignedBitVectorExpression::new("1"));
    reset
        .value
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    reset
        .value
        .extension_attributes
        .insert("irgen:resetSource", "spec");
    reset
        .mask
        .as_mut()
        .expect("reset mask should be present")
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    reset
        .mask
        .as_mut()
        .expect("reset mask should be present")
        .extension_attributes
        .insert("irgen:maskSource", "spec");
    let mut field = Field::new("ENABLE", "0", "1");
    field.resets = Some(Resets { reset: vec![reset] });
    let mut register = Register::new("CONTROL", "0x0", "32");
    register.add_field(field);
    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.add_register(register);
    let mut map = MemoryMap::new("default");
    map.add_address_block(block);

    let mut reset_type = ResetType::new("SOFT");
    reset_type.id = Some("soft-reset-type".into());
    reset_type.display_name = Some("Software Reset".into());
    reset_type.description = Some("Reset asserted by software control".into());
    reset_type.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:reset").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut clock = OtherClockDriver::new("systemClock", "10", "0", "1", "5");
    clock.id = Some("system-clock-driver".into());
    clock.clock_source = Some("clockGenerator.output".into());
    clock.clock_period.minimum = Some(1.0);
    clock.clock_period.maximum = Some(20.0);
    clock.clock_period.units = Some(DelayUnit::Nanoseconds);
    clock
        .clock_period
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    clock
        .clock_period
        .extension_attributes
        .insert("irgen:clockDomain", "system");

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![map],
    });
    component.other_clock_drivers = Some(OtherClockDrivers {
        other_clock_driver: vec![clock],
    });
    component.reset_types = Some(ResetTypes {
        reset_type: vec![reset_type],
    });
    let mut assertion_expr = BitExpression::new("true");
    assertion_expr
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    assertion_expr
        .extension_attributes
        .insert("irgen:assertSource", "policy");
    let mut assertion = Assertion::new("validReset", assertion_expr);
    assertion.id = Some("valid-reset-assertion".into());
    assertion.display_name = Some("Valid Reset Policy".into());
    assertion.description = Some("Reset policy must be consistent".into());
    component.assertions = Some(Assertions {
        assertion: vec![assertion],
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains(
        "<ipxact:otherClockDriver clockName=\"systemClock\" clockSource=\"clockGenerator.output\" xml:id=\"system-clock-driver\">"
    ));
    assert!(xml.contains("irgen:clockDomain=\"system\""));
    assert!(xml.contains("<ipxact:reset resetTypeRef=\"SOFT\" xml:id=\"soft-reset-value\">"));
    assert!(xml.contains("irgen:resetSource=\"spec\""));
    assert!(xml.contains("irgen:maskSource=\"spec\""));
    assert!(xml.contains("<ipxact:resetType xml:id=\"soft-reset-type\">"));
    assert!(xml.contains("<ipxact:displayName>Software Reset</ipxact:displayName>"));
    assert!(xml.contains("<ipxact:assertion xml:id=\"valid-reset-assertion\">"));
    assert!(xml.contains("<ipxact:displayName>Valid Reset Policy</ipxact:displayName>"));
    assert!(xml.contains("irgen:assertSource=\"policy\""));
    validate_xml("component-root-clock-reset-assertions", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component metadata should deserialize");
    let clock = &parsed
        .other_clock_drivers
        .as_ref()
        .expect("component should retain independent clock drivers")
        .other_clock_driver[0];
    assert_eq!(clock.id.as_deref(), Some("system-clock-driver"));
    assert_eq!(clock.clock_source.as_deref(), Some("clockGenerator.output"));
    assert_eq!(clock.clock_period.minimum, Some(1.0));
    assert_eq!(clock.clock_period.maximum, Some(20.0));
    assert_eq!(clock.clock_period.units, Some(DelayUnit::Nanoseconds));
    assert_eq!(
        clock
            .clock_period
            .extension_attributes
            .attributes
            .get("irgen:clockDomain")
            .map(String::as_str),
        Some("system")
    );
    let MemoryMapEntry::AddressBlock(block) = &parsed
        .memory_maps
        .as_ref()
        .expect("component should retain memory maps")
        .memory_map[0]
        .entries[0]
    else {
        panic!("expected address block");
    };
    let RegisterData::Register(register) = &block.register_data[0] else {
        panic!("expected register");
    };
    let reset = &register.field[0]
        .resets
        .as_ref()
        .expect("field should retain resets")
        .reset[0];
    assert_eq!(reset.id.as_deref(), Some("soft-reset-value"));
    assert_eq!(reset.reset_type_ref.as_deref(), Some("SOFT"));
    assert_eq!(
        reset
            .value
            .extension_attributes
            .attributes
            .get("irgen:resetSource")
            .map(String::as_str),
        Some("spec")
    );
    assert_eq!(
        reset
            .mask
            .as_ref()
            .expect("reset should retain mask")
            .extension_attributes
            .attributes
            .get("irgen:maskSource")
            .map(String::as_str),
        Some("spec")
    );
    let reset_type = &parsed
        .reset_types
        .as_ref()
        .expect("component should retain reset types")
        .reset_type[0];
    assert_eq!(reset_type.id.as_deref(), Some("soft-reset-type"));
    assert_eq!(reset_type.name, "SOFT");
    assert_eq!(reset_type.display_name.as_deref(), Some("Software Reset"));
    assert_eq!(
        reset_type.description.as_deref(),
        Some("Reset asserted by software control")
    );
    let assertion = &parsed
        .assertions
        .as_ref()
        .expect("component should retain assertions")
        .assertion[0];
    assert_eq!(assertion.id.as_deref(), Some("valid-reset-assertion"));
    assert_eq!(assertion.name, "validReset");
    assert_eq!(
        assertion.display_name.as_deref(),
        Some("Valid Reset Policy")
    );
    assert_eq!(
        assertion.description.as_deref(),
        Some("Reset policy must be consistent")
    );
    assert_eq!(assertion.assert.value, "true");
    assert_eq!(
        assertion
            .assert
            .extension_attributes
            .attributes
            .get("irgen:assertSource")
            .map(String::as_str),
        Some("policy")
    );
    assert_eq!(parsed, component);
    let roundtrip_xml =
        quick_xml::se::to_string(&parsed).expect("component metadata should reserialize");
    assert!(roundtrip_xml.contains("<acme:reset xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml(
        "component-root-clock-reset-assertions-roundtrip",
        &roundtrip_xml,
    );
}

#[test]
fn transactional_port_validates_against_official_2014_xsd() {
    let mut transactional = TransactionalPort::new(PortInitiative::Requires);
    transactional.kind = Some(PortKind::new("tlm_socket"));
    transactional.bus_width = Some(UnsignedIntExpression::new("32"));
    let mut protocol_type = AbstractionProtocolType::new(ProtocolTypeValue::Custom);
    protocol_type.custom = Some("example_protocol".into());
    let mut payload = Payload::new(PayloadType::Specific);
    payload.name = Some("request".into());
    let mut extension = PayloadExtension::new("security_tag");
    extension.mandatory = Some(true);
    payload.extension = Some(extension);
    payload.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:payload").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });
    let mut protocol = Protocol::new(protocol_type);
    protocol.payload = Some(payload);
    transactional.protocol = Some(protocol);

    let mut type_parameter = ModuleParameter::new("DATA_BYTES", "4");
    type_parameter.parameter_id = Some("data-bytes-param".into());
    type_parameter.data_type = Some("unsigned".into());
    type_parameter.usage_type = Some(ModuleParameterUsage::Typed);
    type_parameter
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    type_parameter
        .extension_attributes
        .insert("irgen:typeParamSource", "port");
    type_parameter.is_present = Some(BitExpression::new("true"));
    type_parameter.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:typeParameter")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut service_type_name = ServiceTypeName::new("addr_type");
    service_type_name.implicit = Some(true);
    let mut service_type_def = ServiceTypeDef::new(service_type_name);
    service_type_def.id = Some("addr-service-type".into());
    let mut service_type_definition = TypeDefinition::new("addr_types.hpp");
    service_type_definition.id = Some("addr-types-include".into());
    service_type_def
        .type_definition
        .push(service_type_definition);

    let mut trans_type_name = TransactionalTypeName::new("tlm_initiator_socket");
    trans_type_name.exact = Some(false);
    let mut trans_type_def = TransTypeDef::new();
    trans_type_def.id = Some("transport-type".into());
    trans_type_def.type_name = Some(trans_type_name);
    let mut type_definition = TypeDefinition::new("tlm_socket.hpp");
    type_definition.id = Some("tlm-socket-include".into());
    trans_type_def.type_definition.push(type_definition);
    trans_type_def.type_parameters = Some(TypeParameters {
        type_parameter: vec![type_parameter],
        service_type_def: vec![service_type_def],
    });
    trans_type_def.view_ref.push(TypeDefViewRef::new("rtl"));

    let mut trans_type_defs = TransTypeDefs::default();
    trans_type_defs.add(trans_type_def);
    transactional.trans_type_defs = Some(Box::new(trans_type_defs));

    transactional.connection = Some(PortConnection {
        max_connections: Some(UnsignedIntExpression::new("4")),
        min_connections: Some(UnsignedIntExpression::new("1")),
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.model = Some(Model {
        views: Some(Views {
            view: vec![View::new("rtl")],
        }),
        ports: Some(Ports {
            port: vec![Port::new(
                "transport",
                PortStyle::Transactional(transactional),
            )],
        }),
        ..Model::default()
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(
        xml.contains(
            "<ipxact:protocolType custom=\"example_protocol\">custom</ipxact:protocolType>"
        )
    );
    assert!(xml.contains("<ipxact:extension mandatory=\"true\">security_tag</ipxact:extension>"));
    assert!(xml.contains("<ipxact:transTypeDefs>"));
    assert!(
        xml.contains("<ipxact:typeName exact=\"false\">tlm_initiator_socket</ipxact:typeName>")
    );
    assert!(xml.contains("<ipxact:typeName implicit=\"true\">addr_type</ipxact:typeName>"));
    assert!(xml.contains("irgen:typeParamSource=\"port\""));
    assert!(xml.contains("<ipxact:isPresent>true</ipxact:isPresent>"));
    assert!(xml.contains("acme:typeParameter"));
    assert!(xml.contains("<ipxact:typeDefinition xml:id=\"tlm-socket-include\">"));
    assert!(xml.contains("<ipxact:serviceTypeDef xml:id=\"addr-service-type\">"));
    assert!(xml.contains("<ipxact:viewRef>rtl</ipxact:viewRef>"));
    validate_xml("transactional-port", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component protocol should deserialize");
    let PortStyle::Transactional(transactional) = &parsed
        .model
        .as_ref()
        .expect("component should retain model")
        .ports
        .as_ref()
        .expect("model should retain ports")
        .port[0]
        .style
    else {
        panic!("expected transactional component port");
    };
    assert_eq!(
        transactional
            .bus_width
            .as_ref()
            .expect("transactional port should retain busWidth")
            .value,
        "32"
    );
    let connection = transactional
        .connection
        .as_ref()
        .expect("transactional port should retain connection bounds");
    assert_eq!(
        connection
            .max_connections
            .as_ref()
            .expect("transactional port should retain maxConnections")
            .value,
        "4"
    );
    assert_eq!(
        connection
            .min_connections
            .as_ref()
            .expect("transactional port should retain minConnections")
            .value,
        "1"
    );
    let type_def = &transactional
        .trans_type_defs
        .as_ref()
        .expect("transactional port should retain type definitions")
        .trans_type_def[0];
    assert_eq!(
        type_def.type_definition[0].id.as_deref(),
        Some("tlm-socket-include")
    );
    let type_parameter = &type_def
        .type_parameters
        .as_ref()
        .expect("transactional type definition should retain type parameters")
        .type_parameter[0];
    assert_eq!(
        type_parameter.parameter_id.as_deref(),
        Some("data-bytes-param")
    );
    assert_eq!(
        type_parameter
            .extension_attributes
            .attributes
            .get("irgen:typeParamSource")
            .map(String::as_str),
        Some("port")
    );
    assert_eq!(
        type_parameter
            .is_present
            .as_ref()
            .expect("type parameter should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        type_parameter
            .vendor_extensions
            .as_ref()
            .expect("type parameter should retain vendor extensions")
            .element[0]
            .name,
        "acme:typeParameter"
    );
    let service_type_def = &type_def
        .type_parameters
        .as_ref()
        .expect("transactional type definition should retain type parameters")
        .service_type_def[0];
    assert_eq!(service_type_def.id.as_deref(), Some("addr-service-type"));
    assert_eq!(
        service_type_def.type_definition[0].id.as_deref(),
        Some("addr-types-include")
    );
    let roundtrip_xml =
        quick_xml::se::to_string(&parsed).expect("component protocol should reserialize");
    assert!(roundtrip_xml.contains("<acme:payload xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("transactional-port-roundtrip", &roundtrip_xml);
}

#[test]
fn model_view_instantiation_refs_validate_against_official_2014_xsd() {
    let mut view = View::new("rtl");
    view.is_present = Some(BitExpression::new("true"));
    view.env_identifier
        .push(EnvironmentIdentifier::new("verilog:*Synthesis:"));
    view.component_instantiation_ref = Some("rtlComponent".into());
    view.design_instantiation_ref = Some("rtlDesign".into());
    view.design_configuration_instantiation_ref = Some("rtlConfig".into());

    let mut design_configuration = DesignConfigurationInstantiation::new(
        "rtlConfig",
        ConfigurableLibraryRef::new("example.org", "designs", "timer-config", "1.0"),
    );
    design_configuration.language = Some(Language::new("verilog"));
    design_configuration.parameters = Some(Parameters {
        parameter: vec![Parameter::new("IMPLEMENTATION", "rtl")],
    });
    design_configuration.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:designConfigurationInstantiation")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut design_instantiation = DesignInstantiation::new(
        "rtlDesign",
        ConfigurableLibraryRef::new("example.org", "designs", "timer-design", "1.0"),
    );
    design_instantiation.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:designInstantiation")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut language = Language::new("verilog");
    language.strict = Some(true);
    let mut module_parameter = ModuleParameter::new("WIDTH", "32");
    module_parameter.parameter_id = Some("rtlWidth".into());
    module_parameter.data_type = Some("int".into());
    module_parameter.usage_type = Some(ModuleParameterUsage::Typed);
    module_parameter
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    module_parameter
        .extension_attributes
        .insert("irgen:moduleUi", "dropdown");
    module_parameter.vectors = Some(PortVectors {
        vector: vec![PortVector::new("31", "0")],
    });
    module_parameter.arrays = Some(ConfigurableArrays {
        array: vec![ConfigurableArray::new("1", "0")],
    });
    module_parameter.is_present = Some(BitExpression::new("true"));
    let mut component_instantiation = ComponentInstantiation::new("rtlComponent");
    component_instantiation.language = Some(language);
    component_instantiation.module_parameters = Some(ModuleParameters {
        module_parameter: vec![module_parameter],
    });
    component_instantiation
        .default_file_builder
        .push(FileBuilder::new(FileType::new(
            FileTypeValue::SystemVerilogSource,
        )));
    component_instantiation
        .file_set_ref
        .push(FileSetRef::new("rtlSources"));
    component_instantiation.parameters = Some(Parameters {
        parameter: vec![Parameter::new("SYNTHESIS", "true")],
    });

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.file_sets = Some(FileSets {
        file_set: vec![FileSet::new("rtlSources")],
    });
    component.model = Some(Model {
        views: Some(Views { view: vec![view] }),
        instantiations: Some(Instantiations {
            instantiation: vec![
                Instantiation::Component(component_instantiation),
                Instantiation::Design(design_instantiation),
                Instantiation::DesignConfiguration(design_configuration),
            ],
        }),
        ports: None,
    });

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains("irgen:moduleUi=\"dropdown\""));
    validate_xml("model-view-instantiations", &xml);

    let parsed = Component::from_xml_str(&xml).expect("component should deserialize");
    assert_eq!(
        parsed
            .model
            .as_ref()
            .expect("component should retain model")
            .views
            .as_ref()
            .expect("model should retain views")
            .view[0]
            .is_present
            .as_ref()
            .expect("view should retain isPresent")
            .value,
        "true"
    );
    let instantiations = &parsed
        .model
        .as_ref()
        .expect("component should retain model")
        .instantiations
        .as_ref()
        .expect("model should retain instantiations")
        .instantiation;
    let Instantiation::Design(design) = &instantiations[1] else {
        panic!("expected design instantiation");
    };
    let Instantiation::Component(component_instantiation) = &instantiations[0] else {
        panic!("expected component instantiation");
    };
    assert_eq!(
        component_instantiation
            .module_parameters
            .as_ref()
            .expect("component instantiation should retain module parameters")
            .module_parameter[0]
            .extension_attributes
            .attributes
            .get("irgen:moduleUi")
            .map(String::as_str),
        Some("dropdown")
    );
    assert_eq!(
        design
            .vendor_extensions
            .as_ref()
            .expect("design instantiation should retain vendor extensions")
            .element[0]
            .name,
        "acme:designInstantiation"
    );
    let Instantiation::DesignConfiguration(configuration) = &instantiations[2] else {
        panic!("expected design configuration instantiation");
    };
    assert_eq!(
        configuration
            .vendor_extensions
            .as_ref()
            .expect("design configuration instantiation should retain vendor extensions")
            .element[0]
            .name,
        "acme:designConfigurationInstantiation"
    );
}

#[test]
fn structured_vendor_extensions_validate_against_official_2014_xsd() {
    let mut metadata = VendorExtension::new("acme:metadata")
        .with_attribute("xmlns:acme", "urn:example:acme")
        .with_attribute("acme:enabled", "true");
    metadata.add_child(VendorExtension::new("acme:label").with_text("timer"));
    let extensions = VendorExtensions {
        element: vec![metadata],
    };

    let mut interface = master_bus_interface("master");
    interface.vendor_extensions = Some(extensions.clone());

    let mut parameter = Parameter::new("FEATURE", "true");
    parameter.vendor_extensions = Some(extensions.clone());

    let mut module_parameter = ModuleParameter::new("WIDTH", "32");
    module_parameter.vendor_extensions = Some(extensions.clone());
    let mut component_instantiation = ComponentInstantiation::new("rtlComponent");
    component_instantiation.module_parameters = Some(ModuleParameters {
        module_parameter: vec![module_parameter],
    });

    let mut port = Port::new("debug", PortStyle::Wire(WirePort::new(PortDirection::In)));
    port.vendor_extensions = Some(extensions.clone());

    let mut enumerated = EnumeratedValue::new("Enabled", "1");
    enumerated.vendor_extensions = Some(extensions.clone());
    let mut field = Field::new("ENABLE", "0", "1");
    field.access_handles = Some(non_indexed_access_handle("u_regs.enable"));
    field.is_present = Some(BitExpression::new("true"));
    field.type_identifier = Some("enableField".into());
    field.is_volatile = Some(false);
    field.enumerated_values = Some(EnumeratedValues {
        enumerated_value: vec![enumerated],
    });
    field.parameters = Some(Parameters {
        parameter: vec![Parameter::new("FIELD_KIND", "enable")],
    });
    field.vendor_extensions = Some(extensions.clone());
    let mut register = Register::new("CONTROL", "0x0", "32");
    register.access_handles = Some(indexed_access_handle("u_regs.control"));
    register.is_present = Some(BitExpression::new("true"));
    register.dim = vec![RegisterDim::new("1")];
    register.type_identifier = Some("controlRegister".into());
    register.is_volatile = Some(true);
    register.parameters = Some(Parameters {
        parameter: vec![Parameter::new("REGISTER_KIND", "control")],
    });
    register.vendor_extensions = Some(extensions.clone());
    register.add_field(field);
    let mut alternate_register =
        AlternateRegister::new("CONTROL_DEBUG", AlternateGroups::new("DEBUG"));
    alternate_register.access_handles = Some(indexed_access_handle("u_regs.control_debug"));
    alternate_register.is_present = Some(BitExpression::new("true"));
    alternate_register.type_identifier = Some("controlDebugRegister".into());
    alternate_register.is_volatile = Some(false);
    alternate_register.parameters = Some(Parameters {
        parameter: vec![Parameter::new("ALTERNATE_KIND", "debug")],
    });
    alternate_register.vendor_extensions = Some(extensions.clone());
    alternate_register.add_field(Field::new("DEBUG_ENABLE", "0", "1"));
    register.alternate_registers = Some(AlternateRegisters {
        alternate_register: vec![alternate_register],
    });
    let mut register_file = RegisterFile::new("channel", "0x0", "4");
    register_file.access_handles = Some(indexed_access_handle("u_regs.channel"));
    register_file.is_present = Some(BitExpression::new("true"));
    register_file.dim = vec![RegisterDim::new("2")];
    register_file.type_identifier = Some("channelFile".into());
    register_file.parameters = Some(Parameters {
        parameter: vec![Parameter::new("REGISTER_FILE_KIND", "channel")],
    });
    register_file.vendor_extensions = Some(extensions.clone());
    register_file.add_register(register);
    let mut block = AddressBlock::new("registers", "0x0", "4", "32");
    block.access_handles = Some(non_indexed_access_handle("u_regs"));
    block.is_present = Some(BitExpression::new("true"));
    block.type_identifier = Some("registerBlock".into());
    block.is_volatile = Some(true);
    block.parameters = Some(Parameters {
        parameter: vec![Parameter::new("BLOCK_KIND", "registers")],
    });
    block.vendor_extensions = Some(extensions.clone());
    block
        .register_data
        .push(RegisterData::RegisterFile(register_file));
    let mut memory_map = MemoryMap::new("default");
    memory_map.is_present = Some(BitExpression::new("true"));
    memory_map.address_unit_bits = Some(UnsignedPositiveLongintExpression::new("8"));
    memory_map.vendor_extensions = Some(extensions.clone());
    memory_map.add_address_block(block);

    let mut component = Component::new("example.org", "peripherals", "timer", "1.0");
    component.bus_interfaces = Some(BusInterfaces {
        bus_interface: vec![interface],
    });
    component.model = Some(Model {
        instantiations: Some(Instantiations {
            instantiation: vec![Instantiation::Component(component_instantiation)],
        }),
        ports: Some(Ports { port: vec![port] }),
        ..Model::default()
    });
    component.parameters = Some(Parameters {
        parameter: vec![parameter],
    });
    component.memory_maps = Some(MemoryMaps {
        memory_map: vec![memory_map],
    });
    component.vendor_extensions = Some(extensions);

    let xml = quick_xml::se::to_string(&component).expect("component should serialize");
    assert!(xml.contains(
        "<acme:metadata acme:enabled=\"true\" xmlns:acme=\"urn:example:acme\"><acme:label>timer</acme:label></acme:metadata>"
    ));
    validate_xml("structured-vendor-extensions", &xml);

    let parsed: VendorExtensions = quick_xml::de::from_str(
        "<vendorExtensions><acme:metadata xmlns:acme=\"urn:example:acme\" acme:enabled=\"true\"><acme:label>timer</acme:label></acme:metadata></vendorExtensions>",
    )
    .expect("vendor extensions should deserialize");
    let metadata = &parsed.element[0];
    assert_eq!(metadata.name, "metadata");
    assert_eq!(metadata.attributes["enabled"], "true");
    assert_eq!(metadata.children[0].name, "label");
    assert_eq!(metadata.children[0].text.as_deref(), Some("timer"));

    let parsed =
        Component::from_xml_str(&xml).expect("component vendor extensions should deserialize");
    let metadata = &parsed
        .vendor_extensions
        .as_ref()
        .expect("component should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(metadata.attributes["acme:enabled"], "true");
    assert_eq!(metadata.children[0].name, "acme:label");
    let bus_interfaces = parsed
        .bus_interfaces
        .as_ref()
        .expect("component should retain its bus interfaces");
    let metadata = &bus_interfaces.bus_interface[0]
        .vendor_extensions
        .as_ref()
        .expect("bus interface should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    let parameters = parsed
        .parameters
        .as_ref()
        .expect("component should retain its parameters");
    let metadata = &parameters.parameter[0]
        .vendor_extensions
        .as_ref()
        .expect("parameter should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    let model = parsed
        .model
        .as_ref()
        .expect("component should retain its model");
    let metadata = &model
        .ports
        .as_ref()
        .expect("model should retain its ports")
        .port[0]
        .vendor_extensions
        .as_ref()
        .expect("port should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    let Instantiation::Component(instantiation) = &model
        .instantiations
        .as_ref()
        .expect("model should retain its instantiations")
        .instantiation[0]
    else {
        panic!("expected component instantiation");
    };
    let metadata = &instantiation
        .module_parameters
        .as_ref()
        .expect("instantiation should retain module parameters")
        .module_parameter[0]
        .vendor_extensions
        .as_ref()
        .expect("module parameter should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    let memory_maps = parsed
        .memory_maps
        .as_ref()
        .expect("component should retain its memory maps");
    let memory_map = &memory_maps.memory_map[0];
    assert_eq!(
        memory_map
            .is_present
            .as_ref()
            .expect("memory map should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        memory_map
            .address_unit_bits
            .as_ref()
            .expect("memory map should retain addressUnitBits")
            .value,
        "8"
    );
    let metadata = &memory_map
        .vendor_extensions
        .as_ref()
        .expect("memory map should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    let MemoryMapEntry::AddressBlock(block) = &memory_map.entries[0] else {
        panic!("expected address block");
    };
    assert_eq!(
        block
            .access_handles
            .as_ref()
            .expect("address block should retain access handles")
            .access_handle[0]
            .slices
            .slice[0]
            .path_segments
            .path_segment[0]
            .path_segment_name,
        "u_regs"
    );
    assert_eq!(
        block
            .is_present
            .as_ref()
            .expect("address block should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(block.base_address.value, "0x0");
    assert_eq!(block.range.value, "4");
    assert_eq!(block.width.value, "32");
    assert_eq!(block.type_identifier.as_deref(), Some("registerBlock"));
    assert_eq!(block.is_volatile, Some(true));
    let metadata = &block
        .vendor_extensions
        .as_ref()
        .expect("address block should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(
        block
            .parameters
            .as_ref()
            .expect("address block should retain parameters")
            .parameter[0]
            .name,
        "BLOCK_KIND"
    );
    let RegisterData::RegisterFile(register_file) = &block.register_data[0] else {
        panic!("expected register file");
    };
    assert_eq!(
        register_file
            .access_handles
            .as_ref()
            .expect("register file should retain access handles")
            .access_handle[0]
            .path_segments
            .path_segment[0]
            .path_segment_name,
        "u_regs.channel"
    );
    assert_eq!(
        register_file
            .is_present
            .as_ref()
            .expect("register file should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(register_file.dim[0].value, "2");
    assert_eq!(register_file.address_offset.value, "0x0");
    assert_eq!(register_file.range.value, "4");
    assert_eq!(
        register_file.type_identifier.as_deref(),
        Some("channelFile")
    );
    let metadata = &register_file
        .vendor_extensions
        .as_ref()
        .expect("register file should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(
        register_file
            .parameters
            .as_ref()
            .expect("register file should retain parameters")
            .parameter[0]
            .name,
        "REGISTER_FILE_KIND"
    );
    let RegisterData::Register(register) = &register_file.register_data[0] else {
        panic!("expected register");
    };
    assert_eq!(
        register
            .access_handles
            .as_ref()
            .expect("register should retain access handles")
            .access_handle[0]
            .path_segments
            .path_segment[0]
            .path_segment_name,
        "u_regs.control"
    );
    assert_eq!(
        register
            .is_present
            .as_ref()
            .expect("register should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(register.dim[0].value, "1");
    assert_eq!(register.address_offset.value, "0x0");
    assert_eq!(register.type_identifier.as_deref(), Some("controlRegister"));
    assert_eq!(register.size.value, "32");
    assert_eq!(register.is_volatile, Some(true));
    let metadata = &register
        .vendor_extensions
        .as_ref()
        .expect("register should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(
        register
            .parameters
            .as_ref()
            .expect("register should retain parameters")
            .parameter[0]
            .name,
        "REGISTER_KIND"
    );
    let metadata = &register
        .alternate_registers
        .as_ref()
        .expect("register should retain alternate registers")
        .alternate_register[0]
        .vendor_extensions
        .as_ref()
        .expect("alternate register should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(
        register
            .alternate_registers
            .as_ref()
            .expect("register should retain alternate registers")
            .alternate_register[0]
            .parameters
            .as_ref()
            .expect("alternate register should retain parameters")
            .parameter[0]
            .name,
        "ALTERNATE_KIND"
    );
    assert_eq!(
        register
            .alternate_registers
            .as_ref()
            .expect("register should retain alternate registers")
            .alternate_register[0]
            .access_handles
            .as_ref()
            .expect("alternate register should retain access handles")
            .access_handle[0]
            .path_segments
            .path_segment[0]
            .path_segment_name,
        "u_regs.control_debug"
    );
    assert_eq!(
        register
            .alternate_registers
            .as_ref()
            .expect("register should retain alternate registers")
            .alternate_register[0]
            .is_present
            .as_ref()
            .expect("alternate register should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(
        register
            .alternate_registers
            .as_ref()
            .expect("register should retain alternate registers")
            .alternate_register[0]
            .type_identifier
            .as_deref(),
        Some("controlDebugRegister")
    );
    assert_eq!(
        register
            .alternate_registers
            .as_ref()
            .expect("register should retain alternate registers")
            .alternate_register[0]
            .is_volatile,
        Some(false)
    );
    let field = &register.field[0];
    assert_eq!(
        field
            .access_handles
            .as_ref()
            .expect("field should retain access handles")
            .access_handle[0]
            .slices
            .slice[0]
            .path_segments
            .path_segment[0]
            .path_segment_name,
        "u_regs.enable"
    );
    assert_eq!(
        field
            .is_present
            .as_ref()
            .expect("field should retain isPresent")
            .value,
        "true"
    );
    assert_eq!(field.type_identifier.as_deref(), Some("enableField"));
    assert_eq!(field.is_volatile, Some(false));
    let metadata = &field
        .vendor_extensions
        .as_ref()
        .expect("field should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(
        field
            .parameters
            .as_ref()
            .expect("field should retain parameters")
            .parameter[0]
            .name,
        "FIELD_KIND"
    );
    let metadata = &field
        .enumerated_values
        .as_ref()
        .expect("field should retain enumerated values")
        .enumerated_value[0]
        .vendor_extensions
        .as_ref()
        .expect("enumerated value should retain vendor extensions")
        .element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(parsed, component);

    let roundtrip_xml =
        quick_xml::se::to_string(&parsed).expect("component vendor extensions should reserialize");
    assert!(roundtrip_xml.contains(
        "<acme:metadata acme:enabled=\"true\" xmlns:acme=\"urn:example:acme\"><acme:label>timer</acme:label></acme:metadata>"
    ));
    validate_xml("structured-vendor-extensions-roundtrip", &roundtrip_xml);

    let exact = VendorExtensions::from_xml_str(
        "<ipxact:vendorExtensions xmlns:ipxact=\"http://www.accellera.org/XMLSchema/IPXACT/1685-2014\"><acme:metadata xmlns:acme=\"urn:example:acme\" acme:enabled=\"true\"><acme:label>timer &amp; counter</acme:label></acme:metadata></ipxact:vendorExtensions>",
    )
    .expect("qualified vendor extensions should parse");
    let metadata = &exact.element[0];
    assert_eq!(metadata.name, "acme:metadata");
    assert_eq!(metadata.attributes["xmlns:acme"], "urn:example:acme");
    assert_eq!(metadata.attributes["acme:enabled"], "true");
    assert_eq!(metadata.children[0].name, "acme:label");
    assert_eq!(
        metadata.children[0].text.as_deref(),
        Some("timer & counter")
    );
}

#[test]
fn abstractor_validates_and_roundtrips_against_official_2014_xsd() {
    let mut abstraction_type = AbstractionType::new(ConfigurableLibraryRef::new(
        "example.org",
        "buses",
        "apb-rtl",
        "1.0",
    ));
    abstraction_type
        .view_ref
        .push(AbstractionViewRef::new("rtl"));
    abstraction_type.port_maps = Some(PortMaps {
        port_map: vec![PortMap::new(
            LogicalPort::new("PADDR"),
            PortMapTarget::PhysicalPort(PhysicalPort::new("paddr")),
        )],
    });

    let mut initiator = AbstractorInterface::new("initiator");
    initiator.abstraction_types = Some(AbstractionTypes {
        abstraction_type: vec![abstraction_type.clone()],
    });
    initiator
        .extension_attributes
        .insert("xmlns:irgen", "urn:irgen:test");
    initiator
        .extension_attributes
        .insert("irgen:side", "initiator");
    initiator.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:interface").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut target = AbstractorInterface::new("target");
    target.abstraction_types = Some(AbstractionTypes {
        abstraction_type: vec![abstraction_type],
    });

    let mut view = AbstractorView::new("rtl");
    view.component_instantiation_ref = Some("rtlImplementation".into());

    let mut wire = AbstractorWirePort::new(PortDirection::Out);
    wire.vectors = Some(PortVectors {
        vector: vec![PortVector::new("31", "0")],
    });
    wire.drivers = Some(Drivers {
        driver: vec![Driver::default_value("0")],
    });

    let mut generator = ComponentGenerator::new("generateAdapter", "tools/generate-adapter");
    generator.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:generator").with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let mut abstractor = Abstractor::new(
        "example.org",
        "abstractors",
        "apb-adapter",
        "1.0",
        AbstractorMode::new(AbstractorModeValue::Direct),
        LibraryRefType::new("example.org", "buses", "apb", "1.0"),
        AbstractorInterfaces::new(initiator, target),
    );
    abstractor.id = Some("apb-adapter".into());
    abstractor.model = Some(AbstractorModel {
        views: Some(AbstractorViews { view: vec![view] }),
        instantiations: Some(AbstractorInstantiations {
            component_instantiation: vec![ComponentInstantiation::new("rtlImplementation")],
        }),
        ports: Some(AbstractorPorts {
            port: vec![AbstractorPort::new(
                "paddr",
                AbstractorPortStyle::Wire(wire),
            )],
        }),
    });
    abstractor.abstractor_generators = Some(AbstractorGenerators {
        abstractor_generator: vec![generator],
    });
    abstractor.parameters = Some(Parameters {
        parameter: vec![Parameter::new("WIDTH", "32")],
    });
    abstractor.assertions = Some(Assertions {
        assertion: vec![Assertion::new("validWidth", "true")],
    });
    abstractor.vendor_extensions = Some(VendorExtensions {
        element: vec![
            VendorExtension::new("acme:abstractor")
                .with_attribute("xmlns:acme", "urn:example:acme"),
        ],
    });

    let xml = quick_xml::se::to_string(&abstractor).expect("abstractor should serialize");
    assert!(xml.starts_with("<ipxact:abstractor"));
    assert!(xml.contains("irgen:side=\"initiator\""));
    assert!(xml.contains("<ipxact:name>initiator</ipxact:name>"));
    assert!(xml.contains("<ipxact:physicalPort><ipxact:name>paddr</ipxact:name>"));
    assert!(xml.contains("<ipxact:abstractorGenerator>"));
    assert!(xml.contains("<acme:generator xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("abstractor", &xml);

    let parsed =
        Abstractor::from_xml_str(&xml).expect("abstractor should deserialize from its XML");
    assert_eq!(parsed, abstractor);
    assert_eq!(
        parsed.abstractor_interfaces.abstractor_interface[0]
            .extension_attributes
            .attributes
            .get("irgen:side")
            .map(String::as_str),
        Some("initiator")
    );
    let generator_extension = &parsed
        .abstractor_generators
        .as_ref()
        .expect("abstractor should retain generators")
        .abstractor_generator[0]
        .vendor_extensions
        .as_ref()
        .expect("generator should retain vendor extensions")
        .element[0];
    assert_eq!(generator_extension.name, "acme:generator");

    let roundtrip_xml =
        quick_xml::se::to_string(&parsed).expect("abstractor should reserialize after parsing");
    assert!(roundtrip_xml.contains("<acme:abstractor xmlns:acme=\"urn:example:acme\"/>"));
    validate_xml("abstractor-roundtrip", &roundtrip_xml);
}

fn validate_xml(name: &str, xml: &str) {
    if Command::new("xmllint").arg("--version").output().is_err() {
        eprintln!("skipping official XSD validation because xmllint is not installed");
        return;
    }

    let output = temp_xml_path(name);
    fs::write(&output, xml).expect("temporary XML should be writable");

    let schema = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../model/tests/fixtures/ipxact-1685-2014/index.xsd");
    let validation = Command::new("xmllint")
        .args(["--noout", "--schema"])
        .arg(schema)
        .arg(&output)
        .output()
        .expect("xmllint should run");

    fs::remove_file(&output).expect("temporary XML should be removable");

    assert!(
        validation.status.success(),
        "official schema validation failed:\n{}",
        String::from_utf8_lossy(&validation.stderr)
    );
}

fn temp_xml_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ip-xact-v2014-{name}-{}.xml", std::process::id()))
}

#[test]
fn numeric_expression_serializes_schema_attributes() {
    let mut expression = NumericExpression::new("4");
    expression.minimum = Some(1);
    expression.maximum = Some(32);

    let xml = quick_xml::se::to_string(&expression).expect("expression should serialize");

    assert_eq!(
        xml,
        "<NumericExpression minimum=\"1\" maximum=\"32\">4</NumericExpression>"
    );
}

#[test]
fn generated_port_protocol_wrapper_serializes_custom_as_attribute() {
    let protocol_type = PortProtocolType {
        custom: Some("example_protocol".into()),
        value: ProtocolTypeType::Custom,
    };

    let xml = quick_xml::se::to_string(&protocol_type).expect("protocol type should serialize");

    assert_eq!(
        xml,
        "<PortProtocolType custom=\"example_protocol\">custom</PortProtocolType>"
    );
}
