//! IP-XACT 2022 types
//!
//! The IEEE 1685-2022 standard adds new types beyond 2014.
//! This module implements the new types and re-uses compatible types.

mod access_policies;
mod access_properties;
mod access_restrictions;
mod additional_types;
mod arrays;
mod bit_stride;
mod domain_type;
mod field_access_policies;
mod field_definitions;
mod memory_map_definitions;
mod port_packets;
mod port_slices;
mod register_definitions;
mod stride;

pub mod vendor_extensions;

// Re-export from v2009 and v2014 for compatibility
pub use crate::v2009::types::{
    AbstractionDefinition, AbstractionPort, AbstractionPorts, AdHocConnection, AdHocConnections,
    AddressBlock, AddressSpace, AddressSpaceRef, AddressSpaces, Bank as AddressSpaceBank,
    Bank as MemoryBank, BankedAddressSpace, BaseAddress, BitWidth, BusDefinition, BusInterface,
    BusInterfaces, Channel, Channels, Choice, Choices, Component, ComponentGenerators,
    ComponentInstance, ComponentInstances, ComponentRef, ConfigurableElementValue,
    ConfigurableElementValues, Design, Driver, EnumeratedValue, EnumeratedValues, Field, File,
    FileSet, Generator, GeneratorTypeRef, HierConnection, HierConnections, Interconnection,
    Interconnections, InterfaceMode, LibraryRef, LocalMemoryMap, LogicalPort, MasterDetails,
    MasterMode, MemoryMap, MemoryMapEntry, MemoryMaps, Model, ModelConnections, ModifiedWriteValue,
    NameValuePair, OnSystem, ParameterValue, Parameters, PhysicalPort, Port, PortMap, PortMaps,
    PortReference, Ports, Protocol, ProtocolType, ProtocolTypeDef, Qualifier, ReadAction, Register,
    RemapState, RemapStates, SlaveDetails, SlaveMode, SubspaceMap, SystemDetails, SystemMode,
    TransactionTypeDef, TransactionalAbstraction, TransactionalPort, TransactionalProtocol,
    TransactionalType, UsageType, Vector, View, Views, WireAbstraction, WirePort, WireTypeDef,
    WireTypeDefs, WriteValueConstraint,
};

// Re-export v2014 types
pub use crate::v2014::types::{
    Assertion, Assertions, Catalog, ComponentInstantiation, ConfigurableArrays, IpxactFile,
    IpxactFiles, LibraryRefType, StringExpression, StringURIExpression,
};

// Export v2022-specific types
pub use access_policies::AccessPolicies;
pub use access_properties::AccessPropertiesType;
pub use access_restrictions::AccessRestrictions;
pub use additional_types::*;
pub use arrays::{Array, Arrays};
pub use bit_stride::BitStride;
pub use domain_type::{DomainType, DomainTypes};
pub use field_access_policies::FieldAccessPolicies;
pub use field_definitions::FieldDefinitions;
pub use memory_map_definitions::MemoryMapDefinitions;
pub use port_packets::{PortPacket, PortPackets};
pub use port_slices::{PortSlice, PortSlices};
pub use register_definitions::RegisterDefinitions;
pub use stride::Stride;
