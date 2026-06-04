//! IP-XACT 2009 types

mod additional_types;
mod address_block;
mod address_space;
mod bus_definition;
mod bus_interface;
mod component;
mod component_containers;
mod design;
mod file_set;
mod memory_map;
mod name_value_pair;
mod parameters;
mod register;
pub mod vendor_extensions;

pub use additional_types::{
    AbstractionDefPortConstraintsType, AbstractorBusInterfaceType, AbstractorGenerators,
    AbstractorModeType, AbstractorModelType, AbstractorPortType, AbstractorPortWireType,
    AbstractorType, AbstractorViewType, AddrSpaceRefType, AddressBankType, BankAlignmentType,
    BankedBankType, BankedBlockType, BankedSubspaceType, CellClassValueType, CellFunctionValueType,
    CellSpecification, CellStrengthValueType, ClockDriver, ClockDriverType, ComponentGenerator,
    ConstraintSet, ConstraintSets, DataTypeType, DefaultValue, DelayValueType, DelayValueUnitType,
    DesignConfiguration, DriveConstraint, EdgeValueType, ExecutableImage, FileBuilderType,
    FileSetRef, FormatType, GeneratorChain, GeneratorSelectorType, GroupSelector, HierInterface,
    InstanceGeneratorType, Interface, LoadConstraint, MemoryMapRefType, MemoryRemapType,
    MonitorInterconnection, NameValueTypeType, Phase, PortAccessType, PortDeclarationType,
    PortTransactionalType, PortWireType, RangeTypeType, RequiresDriver, ResolveType,
    ResolvedLibraryRefType, ServiceType, ServiceTypeDef, ServiceTypeDefs, SingleShotDriver,
    SubspaceRefType, TimingConstraint, TransTypeDef, ValueMaskConfigType, WhiteboxElementRefType,
};
pub use address_block::{BaseAddress, UsageType};
pub use address_space::{AddressSpace, AddressSpaceRef, Bank, BankedAddressSpace, LocalMemoryMap};
pub use bus_definition::{
    AbstractionDefinition, AbstractionPort, AbstractionPorts, BusDefinition, OnSystem,
    ProtocolTypeDef, Qualifier, TransactionTypeDef, TransactionalAbstraction,
    TransactionalProtocol, WireAbstraction,
};
pub use bus_interface::{
    BusInterface, InterfaceMode, LibraryRef, LogicalPort, MasterDetails, MasterMode, PhysicalPort,
    PortMap, PortMaps, SlaveDetails, SlaveMode, SystemDetails, SystemMode, Vector,
};
pub use component::{
    AddressBlock, Component, Cpu, Cpus, Field, MemoryMap, MemoryMaps, OtherClockDriver,
    OtherClocks, Register, RegisterFile, WhiteboxElementType, WhiteboxElements,
};
pub use component_containers::{
    AddressSpaces, BusInterfaces, Channel, Channels, Driver, Model, ModelConnections, Port, Ports,
    Protocol, ProtocolType, RemapState, RemapStates, TransactionalPort, TransactionalType, View,
    Views, WirePort, WireTypeDef, WireTypeDefs,
};
pub use design::{
    AdHocConnection, AdHocConnections, ComponentInstance, ComponentInstances, ComponentRef,
    ConfigurableElementValue, ConfigurableElementValues, Design, HierConnection, HierConnections,
    Interconnection, Interconnections, PortReference,
};
pub use file_set::{
    Choice, Choices, ComponentGenerators, File, FileSet, FileSets, Generator, GeneratorTypeRef,
};
pub use memory_map::{Bank as MemoryBank, MemoryMapEntry, SubspaceMap};
pub use name_value_pair::{NameValuePair, ParameterValue};
pub use parameters::Parameters;
pub use register::{
    BitWidth, EnumeratedValue, EnumeratedValues, ModifiedWriteValue, ReadAction,
    WriteValueConstraint,
};
pub use vendor_extensions::VendorExtensions;
