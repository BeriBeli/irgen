//! IP-XACT 2022 types
//!
//! The IEEE 1685-2022 standard adds new types beyond 2014.
//! This module implements the new types and re-uses compatible types.

mod access_policies;
mod access_properties;
mod access_restrictions;
mod memory_map_definitions;
mod register_definitions;
mod field_definitions;
mod field_access_policies;
mod port_packets;
mod port_slices;
mod domain_type;
mod arrays;
mod bit_stride;
mod stride;
mod additional_types;

pub mod vendor_extensions;

// Re-export from v2009 and v2014 for compatibility
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

// Re-export v2014 types
pub use crate::v2014::types::{
    Catalog, IpxactFiles, IpxactFile,
    StringExpression, StringURIExpression, LibraryRefType,
    Assertion, Assertions, ComponentInstantiation, ConfigurableArrays,
};

// Export v2022-specific types
pub use access_policies::AccessPolicies;
pub use access_properties::AccessPropertiesType;
pub use access_restrictions::AccessRestrictions;
pub use memory_map_definitions::MemoryMapDefinitions;
pub use register_definitions::RegisterDefinitions;
pub use field_definitions::FieldDefinitions;
pub use field_access_policies::FieldAccessPolicies;
pub use port_packets::{PortPackets, PortPacket};
pub use port_slices::{PortSlices, PortSlice};
pub use domain_type::{DomainType, DomainTypes};
pub use arrays::{Arrays, Array};
pub use bit_stride::BitStride;
pub use stride::Stride;
pub use additional_types::*;
