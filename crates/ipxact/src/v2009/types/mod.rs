//! IP-XACT 2009 types

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
mod additional_types;
mod parameters;
mod register;
pub mod vendor_extensions;

pub use address_block::{AddressBlock, BaseAddress, RegisterFile, UsageType};
pub use vendor_extensions::VendorExtensions;
pub use address_space::{AddressSpace, AddressSpaceRef, Bank, BankedAddressSpace, LocalMemoryMap};
pub use bus_definition::{
    AbstractionDefinition, AbstractionPort, AbstractionPorts, BusDefinition, OnSystem, ProtocolTypeDef,
    Qualifier, TransactionTypeDef, TransactionalAbstraction, TransactionalProtocol, WireAbstraction,
};
pub use bus_interface::{
    BusInterface, InterfaceMode, LibraryRef, LogicalPort, MasterDetails, MasterMode, PhysicalPort,
    PortMap, PortMaps, SlaveDetails, SlaveMode, SystemDetails, SystemMode, Vector,
};
pub use component::{
    Component, Cpu, Cpus, OtherClockDriver, OtherClocks, WhiteboxElementType, WhiteboxElements,
};
pub use component_containers::{
    BusInterfaces, Channels, Channel, Model, ModelConnections, Ports, Port, RemapStates, RemapState,
    Views, View, WirePort, WireTypeDefs, WireTypeDef, Driver, TransactionalPort, TransactionalType,
    Protocol, ProtocolType, MemoryMaps, AddressSpaces,
};
pub use design::{
    AdHocConnection, AdHocConnections, ComponentInstance, ComponentInstances, ComponentRef,
    ConfigurableElementValue, ConfigurableElementValues, Design, HierConnection, HierConnections,
    Interconnection, Interconnections, PortReference,
};
pub use file_set::{Choice, Choices, ComponentGenerators, File, FileSet, FileSets, Generator, GeneratorTypeRef};
pub use memory_map::{Bank as MemoryBank, MemoryMap, MemoryMapEntry, SubspaceMap};
pub use name_value_pair::{NameValuePair, ParameterValue};
pub use parameters::Parameters;
pub use register::{
    BitWidth, EnumeratedValue, EnumeratedValues, Field, ModifiedWriteValue, ReadAction, Register,
    WriteValueConstraint,
};
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
