//! IP-XACT 2014 types
//!
//! The IEEE 1685-2014 standard adds new types beyond 2009.
//! This module implements IEEE 1685-2014-specific types.

mod abstraction_definition;
mod abstractor;
mod additional_types;
mod assertion;
mod assertions;
mod bus_definition;
mod catalog;
mod component;
mod component_instantiation;
mod configurable_arrays;
mod design;
mod design_configuration;
mod generator_chain;
mod ipxact_file;
mod ipxact_files;
mod library_ref;
mod string_expression;

pub mod vendor_extensions;

// Export v2014-specific types
pub use abstraction_definition::{
    AbstractionDefPortConstraints, AbstractionDefinition, AbstractionPort, AbstractionPortStyle,
    AbstractionPorts, Direction, DirectionValue, DriverType, Initiative, InitiativeValue, OnSystem,
    Payload, PayloadExtension, PayloadType, Presence, PresenceValue, Protocol,
    ProtocolType as AbstractionProtocolType, ProtocolTypeValue, Qualifier, RequiresDriver,
    TransactionalAbstraction, TransactionalOnSystem, TransactionalPortMode, TransactionalQualifier,
    WireAbstraction, WirePortDriver, WirePortMode,
};
pub use abstractor::{
    Abstractor, AbstractorGenerators, AbstractorInstantiations, AbstractorInterface,
    AbstractorInterfaces, AbstractorMode, AbstractorModeValue, AbstractorModel, AbstractorPort,
    AbstractorPortStyle, AbstractorPorts, AbstractorView, AbstractorViews, AbstractorWirePort,
};
pub use additional_types::*;
pub use assertion::Assertion;
pub use assertions::Assertions;
pub use bus_definition::{BusDefinition, SystemGroupName, SystemGroupNames, UnsignedIntExpression};
pub use catalog::Catalog;
pub use component::{
    AbstractionType, AbstractionTypes, AbstractionViewRef, Access, AccessHandles, AccessViewRef,
    AddressBlock, AddressSpace, AddressSpaceRef, AddressSpaces, AlternateGroup, AlternateGroups,
    AlternateRegister, AlternateRegisters, Bank, BankAlignment, BankEntry, BankedAddressBlock,
    BankedBank, BankedSubspaceMap, BitExpression, BitSteeringExpression, BuildCommand, BuildFlags,
    BusInterface, BusInterfaceMode, BusInterfaces, CellClass, CellFunction, CellFunctionValue,
    CellSpecification, CellSpecificationKind, CellStrength, Channel, ChannelBusInterfaceRef,
    Channels, Choice, ChoiceEnumeration, Choices, ClockDriver, ClockEdge, ClockTimeExpression,
    Component, ComponentGenerator, ComponentGenerators, ConfigurableElementValue,
    ConfigurableElementValues, ConfigurableLibraryRef, ConstraintSet, ConstraintSetRef,
    ConstraintSets, Cpu, Cpus, DelayType, DelayUnit, Dependency, DesignConfigurationInstantiation,
    DesignInstantiation, DriveConstraint, Driver, DriverKind, Drivers, Endianness, EnumeratedValue,
    EnumeratedValueUsage, EnumeratedValues, EnvironmentIdentifier, ExecutableImage, ExportedName,
    Field, File, FileBuilder, FileSet, FileSetFunction, FileSetGroup, FileSetRef, FileSetRefGroup,
    FileSets, FileType, FileTypeValue, FunctionArgument, FunctionDataType, FunctionReturnType,
    FunctionSourceFile, GeneratorApi, GeneratorApiType, GeneratorGroup, GeneratorRef,
    GeneratorScope, ImageType, IncludeFile, IndexedAccessHandle, IndexedAccessHandles, Indices,
    IndirectInterface, IndirectInterfaceTarget, IndirectInterfaces, Instantiation, Instantiations,
    Language, LanguageFileBuilder, LanguageLinker, LanguageTools, LeafAccessHandle,
    LinkerCommandFile, LoadConstraint, LocalBank, LocalBankEntry, LocalBankedBank, LocalMemoryMap,
    LocalMemoryMapEntry, LogicalName, LogicalPort, Master, MemoryMap, MemoryMapEntry, MemoryMapRef,
    MemoryMaps, MemoryRemap, MemoryUsage, MirroredMaster, MirroredSlave,
    MirroredSlaveBaseAddresses, MirroredSystem, Model, ModifiedWriteValue, ModifiedWriteValueKind,
    Monitor, MonitoredInterfaceMode, NameValuePair, NonIndexedAccessHandles,
    NonIndexedLeafAccessHandle, NumericExpression, OtherClockDriver, OtherClockDrivers, Parameter,
    ParameterExpression, ParameterFormat, ParameterPrefix, ParameterResolve, ParameterSign,
    ParameterUnit, Parameters, PathSegment, PathSegments, PhysicalPort, Port, PortAccess,
    PortConnection, PortDirection, PortInitiative, PortKind, PortMap, PortMapTarget, PortMaps,
    PortRange, PortStyle, PortVector, PortVectors, Ports, ReadAction, ReadActionKind,
    RealExpression, Register, RegisterData, RegisterDim, RegisterFile, RemapAddress, RemapPort,
    RemapPorts, RemapState, RemapStates, Reset, ResetType, ResetTypes, Resets, Segment, Segments,
    ServiceTypeDef, ServiceTypeName, Shared, SimpleAccessHandle, SimpleAccessHandles,
    SimplePortAccess, SingleShotDriver, Slave, SlaveFileSetRefGroup, SlaveTarget, Slice, Slices,
    SubspaceMap, System, TestConstraint, Testable, TimingConstraint, TransTypeDef, TransTypeDefs,
    TransactionalPort, TransactionalTypeName, TransparentBridge, TransportMethod,
    TransportMethodType, TransportMethods, TypeDefViewRef, TypeDefinition, TypeParameter,
    TypeParameters, View, Views, WhiteboxElement, WhiteboxElementRef, WhiteboxElementRefs,
    WhiteboxElements, WhiteboxType, WirePort, WireTypeDef, WireTypeDefinition, WireTypeDefs,
    WireTypeName, WireTypeViewRef, WriteValueConstraint, WriteValueConstraintChoice,
    WriteValueRange,
};
pub use component_instantiation::{
    ComponentInstantiation, ModuleParameter, ModuleParameterUsage, ModuleParameters,
};
pub use configurable_arrays::{ConfigurableArray, ConfigurableArrays};
pub use design::{
    ActiveInterface, AdHocConnection, AdHocConnections, ComponentInstance, ComponentInstances,
    Design, ExcludePort, ExcludePorts, ExternalPortReference, HierInterface, Interconnection,
    InterconnectionEntry, Interconnections, InternalPortReference, MonitorInterconnection,
    MonitorInterface, PartSelect, PortReferences, TiedValue,
};
pub use design_configuration::{
    AbstractorInstance, AbstractorInstances, DesignConfiguration, InterconnectionConfiguration,
    InterfaceRef, ViewConfiguration, ViewSelection,
};
pub use generator_chain::{
    ChainGenerator, ComponentGeneratorSelector, GeneratorChain, GeneratorChainEntry,
    GeneratorChainSelection, GeneratorChainSelector, GroupSelector, MultipleGroupSelectionOperator,
};
pub use ipxact_file::IpxactFile;
pub use ipxact_files::IpxactFiles;
pub use library_ref::LibraryRefType;
pub use string_expression::{StringExpression, StringURIExpression};
pub use vendor_extensions::{VendorExtension, VendorExtensions};
