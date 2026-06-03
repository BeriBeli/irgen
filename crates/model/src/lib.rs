pub mod attr;
pub mod base;
pub mod error;

// use regex::Regex;

use attr::{extract_access_value, extract_modified_write_value, extract_read_action_value};
use error::Error;
use ip_xact::v2014::types as ipxact;

pub fn serialize_ipxact_xml(base: &base::Component) -> Result<String, Error> {
    Ok(quick_xml::se::to_string(&ipxact::Component::try_from(
        base,
    )?)?)
}

impl TryFrom<&base::Component> for ipxact::Component {
    type Error = Error;

    fn try_from(base: &base::Component) -> Result<Self, Error> {
        // not considering removing reserved registers (some reserved's access is rw)
        // let re = Regex::new(r"^(rsvd|reserved)\d*$")?;

        let mut memory_map = ipxact::MemoryMap::new(base.name());
        for blk in base.blks() {
            let mut address_block =
                ipxact::AddressBlock::new(blk.name(), blk.offset(), blk.range(), blk.size());

            for reg in blk.regs() {
                let mut register = ipxact::Register::new(reg.name(), reg.offset(), reg.size());

                for field in reg.fields() {
                    // .filter(|field| {
                    //     !re.is_match(field.name())
                    // })
                    let mut ipxact_field =
                        ipxact::Field::new(field.name(), field.offset(), field.width());
                    ipxact_field.description = non_empty_string(field.desc());
                    ipxact_field.access = Some(access_kind(&extract_access_value(field.attr())?)?);
                    ipxact_field.modified_write_value = extract_modified_write_value(field.attr())?
                        .map(|value| {
                            modified_write_value_kind(&value).map(ipxact::ModifiedWriteValue::new)
                        })
                        .transpose()?;
                    ipxact_field.read_action = extract_read_action_value(field.attr())?
                        .map(|value| read_action_kind(&value).map(ipxact::ReadAction::new))
                        .transpose()?;
                    ipxact_field.resets = Some(ipxact::Resets {
                        reset: vec![ipxact::Reset::new(field.reset())],
                    });

                    register.add_field(ipxact_field);
                }

                address_block.add_register(register);
            }

            memory_map.add_address_block(address_block);
        }

        let mut memory_maps = ipxact::MemoryMaps::default();
        memory_maps.add(memory_map);

        let mut component =
            ipxact::Component::new(base.vendor(), base.library(), base.name(), base.version());
        component.memory_maps = Some(memory_maps);

        Ok(component)
    }
}

fn non_empty_string(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

fn access_kind(value: &str) -> Result<ipxact::Access, Error> {
    match value {
        "read-only" => Ok(ipxact::Access::ReadOnly),
        "write-only" => Ok(ipxact::Access::WriteOnly),
        "read-write" => Ok(ipxact::Access::ReadWrite),
        "writeOnce" => Ok(ipxact::Access::WriteOnce),
        "read-writeOnce" => Ok(ipxact::Access::ReadWriteOnce),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn modified_write_value_kind(value: &str) -> Result<ipxact::ModifiedWriteValueKind, Error> {
    match value {
        "oneToClear" => Ok(ipxact::ModifiedWriteValueKind::OneToClear),
        "oneToSet" => Ok(ipxact::ModifiedWriteValueKind::OneToSet),
        "oneToToggle" => Ok(ipxact::ModifiedWriteValueKind::OneToToggle),
        "zeroToClear" => Ok(ipxact::ModifiedWriteValueKind::ZeroToClear),
        "zeroToSet" => Ok(ipxact::ModifiedWriteValueKind::ZeroToSet),
        "zeroToToggle" => Ok(ipxact::ModifiedWriteValueKind::ZeroToToggle),
        "clear" => Ok(ipxact::ModifiedWriteValueKind::Clear),
        "set" => Ok(ipxact::ModifiedWriteValueKind::Set),
        "modify" => Ok(ipxact::ModifiedWriteValueKind::Modify),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn read_action_kind(value: &str) -> Result<ipxact::ReadActionKind, Error> {
    match value {
        "clear" => Ok(ipxact::ReadActionKind::Clear),
        "set" => Ok(ipxact::ReadActionKind::Set),
        "modify" => Ok(ipxact::ReadActionKind::Modify),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}
