//! IP-XACT 2014 types
//!
//! The IEEE 1685-2014 standard adds new types beyond 2009.
//! This module implements the new types and re-uses v2009 types where compatible.

mod catalog;
mod ipxact_files;
mod ipxact_file;
mod string_expression;
mod library_ref;
mod assertion;
mod assertions;
mod component_instantiation;
mod configurable_arrays;
mod additional_types;

pub mod vendor_extensions;

// Re-export from v2009 for compatibility
pub use crate::v2009::types::{
    AddressBlock, BaseAddress, UsageType,
    AddressSpace, AddressSpaceRef, Bank as AddressSpaceBank, BankedAddressSpace, LocalMemoryMap,
    AbstractionDefinition, AbstractionPort, AbstractionPorts, BusDefinition,
    OnSystem, ProtocolTypeDef, Qualifier, TransactionTypeDef,
    TransactionalAbstraction, TransactionalProtocol, WireAbstraction,
    BusInterface, InterfaceMode, LibraryRef, LogicalPort,
    MasterDetails, MasterMode, PhysicalPort, PortMap, PortMaps,
    SlaveDetails, SlaveMode, SystemDetails, SystemMode, Vector,
    Component,
    BusInterfaces, Channels, Channel, Model, ModelConnections, Ports, Port,
    RemapStates, RemapState, Views, View, WirePort, WireTypeDefs, WireTypeDef,
    Driver, TransactionalPort, TransactionalType, Protocol, ProtocolType,
    MemoryMaps, AddressSpaces,
    AdHocConnection, AdHocConnections, ComponentInstance, ComponentInstances,
    ComponentRef, ConfigurableElementValue, ConfigurableElementValues, Design,
    HierConnection, HierConnections, Interconnection, Interconnections, PortReference,
    Choice, Choices, ComponentGenerators, File, FileSet, Generator, GeneratorTypeRef,
    Bank as MemoryBank, MemoryMap, MemoryMapEntry, SubspaceMap,
    NameValuePair, ParameterValue, Parameters,
    BitWidth, EnumeratedValue, EnumeratedValues, Field, ModifiedWriteValue,
    ReadAction, Register, WriteValueConstraint,
};

// Export v2014-specific types
pub use catalog::Catalog;
pub use ipxact_files::IpxactFiles;
pub use ipxact_file::IpxactFile;
pub use string_expression::{StringExpression, StringURIExpression};
pub use library_ref::LibraryRefType;
pub use assertion::Assertion;
pub use assertions::Assertions;
pub use component_instantiation::ComponentInstantiation;
pub use configurable_arrays::ConfigurableArrays;
pub use additional_types::*;
