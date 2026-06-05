use super::*;
use irgen_model::base::{Block as BaseBlock, Component, Field as BaseField};
use irgen_model::base::{Register as BaseRegister, RegisterFile as BaseRegisterFile};

#[test]
fn serializes_block_registers_and_fields_from_base_model() {
    let component = Component::new(
        "example.com".into(),
        "IP".into(),
        "example".into(),
        "1.0".into(),
        vec![BaseBlock::new(
            "regs".into(),
            "0x0".into(),
            "0x20".into(),
            "32".into(),
            vec![BaseRegister::new(
                "status".into(),
                "0x4".into(),
                "32".into(),
                vec![BaseField::new(
                    "ready".into(),
                    "0".into(),
                    "1".into(),
                    "RO".into(),
                    "0x1".into(),
                    "ready flag".into(),
                )],
            )],
        )],
    );

    let ralf = serialize_ralf(&component).unwrap();

    assert_eq!(
        ralf,
        "block regs {\n  bytes 4;\n  register status @'h4 {\n    bytes 4;\n    field ready (ready) @0 {\n      bits 1;\n      access ro;\n      hard_reset 'h1;\n      doc { ready flag }\n    }\n  }\n}\n"
    );
}

#[test]
fn serializes_register_file_arrays_from_base_model() {
    let component = Component::new(
        "example.com".into(),
        "IP".into(),
        "example".into(),
        "1.0".into(),
        vec![BaseBlock::new_with_register_files(
            "regs".into(),
            "0x0".into(),
            "0x100".into(),
            "32".into(),
            vec![],
            vec![BaseRegisterFile::new(
                "lane".into(),
                "0x10".into(),
                "0x20".into(),
                "4".into(),
                vec![BaseRegister::new(
                    "control".into(),
                    "0x0".into(),
                    "32".into(),
                    vec![BaseField::new(
                        "enable".into(),
                        "0".into(),
                        "1".into(),
                        "RW".into(),
                        "0".into(),
                        "".into(),
                    )],
                )],
            )],
        )],
    );

    let ralf = serialize_ralf(&component).unwrap();

    assert!(ralf.contains("regfile lane[4] @'h10 +'h20 {"));
    assert!(ralf.contains("register control @'h0 {"));
    assert!(ralf.contains("field enable (enable) @0 {"));
    assert!(ralf.contains("access rw;"));
}

#[test]
fn maps_base_field_hdl_paths_to_ralf_instances() {
    let component = Component::new(
        "example.com".into(),
        "IP".into(),
        "example".into(),
        "1.0".into(),
        vec![BaseBlock::new(
            "regs".into(),
            "0x0".into(),
            "0x20".into(),
            "32".into(),
            vec![BaseRegister::new(
                "status".into(),
                "0x4".into(),
                "32".into(),
                vec![
                    BaseField::new_with_hdl_path(
                        "ready".into(),
                        "0".into(),
                        "1".into(),
                        "RO".into(),
                        "0".into(),
                        "".into(),
                        Some("u_status.ready_q".into()),
                    ),
                    BaseField::new(
                        "reserved0".into(),
                        "1".into(),
                        "1".into(),
                        "RO".into(),
                        "0".into(),
                        "".into(),
                    ),
                ],
            )],
        )],
    );

    let ralf = serialize_ralf(&component).unwrap();

    assert!(ralf.contains("field ready (u_status.ready_q) @0 {"));
    assert!(ralf.contains("field reserved0 @1 {"));
    assert!(!ralf.contains("field reserved0 (reserved0)"));
}

#[test]
fn serializes_full_ralf_document_model() {
    let mut register = Register {
        name: "flags".into(),
        bytes: Some("4".into()),
        fields: vec![FieldInstance {
            name: "done".into(),
            offset: Some("'h0".into()),
            definition: Some(Field {
                name: "done".into(),
                bits: Some("1".into()),
                access: Some(Access::W1c),
                hard_reset: Some("'h0".into()),
                enum_values: vec![
                    EnumValue {
                        name: "IDLE".into(),
                        value: Some("0".into()),
                    },
                    EnumValue {
                        name: "DONE".into(),
                        value: Some("1".into()),
                    },
                ],
                cover: vec![CoverDirective {
                    include: true,
                    kind: CoverKind::FieldValues,
                }],
                ..Field::default()
            }),
            ..FieldInstance::default()
        }],
        constraints: vec![Constraint {
            name: "valid".into(),
            body: Some("done.value inside {0, 1};".into()),
        }],
        crosses: vec![Cross {
            items: vec!["done".into(), "done".into()],
            label: Some("done_cross".into()),
        }],
        user_codes: vec![UserCode {
            lang: Some("SV".into()),
            scope: Some("new".into()),
            body: "this.done.configure();".into(),
        }],
        add_reg_callbacks: vec![AddRegCallback {
            target: Some("done".into()),
            callback_class: "broadcast_field_cb".into(),
            args: Some("{DATA0, DATA1}".into()),
            external_cb_class: false,
        }],
        ..Register::default()
    };
    register.attributes.entries.push(Attribute {
        name: "NO_RAL_TESTS".into(),
        value: "1".into(),
    });

    let document = Document {
        items: vec![
            TopLevelItem::Register(register),
            TopLevelItem::RegisterCallback(CallbackClass {
                name: "broadcast_reg_cb".into(),
                var_declarations: Some("uvm_reg regs[$];".into()),
                new_method: Some(CallbackNewMethod {
                    args: Some("uvm_reg regs[$] = '{}'".into()),
                    body: Some("this.regs = regs;".into()),
                }),
                pre_write_method: None,
                post_write_method: Some("foreach (regs[i]) regs[i].set(rw.value[0]);".into()),
                pre_read_method: None,
                post_read_method: None,
            }),
            TopLevelItem::FieldCallback(CallbackClass {
                name: "broadcast_field_cb".into(),
                var_declarations: None,
                new_method: None,
                pre_write_method: None,
                post_write_method: Some("value = rw.value[0];".into()),
                pre_read_method: None,
                post_read_method: None,
            }),
            TopLevelItem::Memory(Memory {
                name: "buffer".into(),
                size: Some("64k".into()),
                bits: Some("8".into()),
                access: Some(Access::Ro),
                initial: Some(Initial::Literal {
                    value: "0".into(),
                    step: Some(Step::Increment),
                }),
                ..Memory::default()
            }),
            TopLevelItem::VirtualRegister(VirtualRegister {
                name: "vflags".into(),
                bytes: Some("4".into()),
                fields: vec![VirtualFieldInstance {
                    name: "done".into(),
                    bits: Some("1".into()),
                    ..VirtualFieldInstance::default()
                }],
                ..VirtualRegister::default()
            }),
            TopLevelItem::System(System {
                name: "soc".into(),
                body: HierarchyBody {
                    bytes: Some("4".into()),
                    endian: Some(Endian::Little),
                    blocks: vec![BlockInstance {
                        name: "regs".into(),
                        array: Some(Array::Count("2".into())),
                        offset: "'hf0000".into(),
                        increment: Some("'h1000".into()),
                        ..BlockInstance::default()
                    }],
                    ..HierarchyBody::default()
                },
                user_codes: vec![UserCode {
                    lang: Some("SV".into()),
                    scope: None,
                    body: "function void poke(); endfunction".into(),
                }],
                ..System::default()
            }),
        ],
    };

    let ralf = serialize_document(&document);

    assert!(ralf.contains("register flags {"));
    assert!(ralf.contains("attributes {"));
    assert!(ralf.contains("field done @'h0 {"));
    assert!(ralf.contains("enum { IDLE=0, DONE=1 }"));
    assert!(ralf.contains("constraint valid {"));
    assert!(ralf.contains("cross done done {"));
    assert!(ralf.contains("user_code lang=SV (new) {"));
    assert!(ralf.contains("add_reg_cb done broadcast_field_cb ({DATA0, DATA1});"));
    assert!(ralf.contains("register_cb broadcast_reg_cb {"));
    assert!(ralf.contains("field_cb_class broadcast_field_cb {"));
    assert!(ralf.contains("memory buffer {"));
    assert!(ralf.contains("initial 0++;"));
    assert!(ralf.contains("virtual register vflags {"));
    assert!(ralf.contains("system soc {"));
    assert!(ralf.contains("block regs[2] @'hf0000 +'h1000;"));
    assert!(ralf.contains("user_code lang=SV {"));
}

#[test]
fn serializes_multi_domain_block_model() {
    let document = Document {
        items: vec![TopLevelItem::Block(Block {
            name: "bridge".into(),
            default_map_name: Some("frontdoor".into()),
            user_codes: vec![UserCode {
                lang: Some("SV".into()),
                scope: Some("new".into()),
                body: "default_map.set_auto_predict(1);".into(),
            }],
            add_reg_callbacks: vec![AddRegCallback {
                target: Some("ahb_flags".into()),
                callback_class: "broadcast_reg_cb".into(),
                args: Some("{DATA0, DATA1}".into()),
                external_cb_class: true,
            }],
            domains: vec![
                Domain {
                    name: "ahb".into(),
                    body: AddressableBody {
                        bytes: Some("4".into()),
                        registers: vec![RegisterInstance {
                            name: "flags".into(),
                            rename: Some("ahb_flags".into()),
                            access: Some(InstanceAccess::Read),
                            ..RegisterInstance::default()
                        }],
                        ..AddressableBody::default()
                    },
                    ..Domain::default()
                },
                Domain {
                    name: "pci".into(),
                    body: AddressableBody {
                        bytes: Some("4".into()),
                        regfiles: vec![RegFileInstance {
                            name: "chan".into(),
                            array: Some(Array::Count("16".into())),
                            offset: Some("'h100".into()),
                            increment: Some("'h10".into()),
                            ..RegFileInstance::default()
                        }],
                        ..AddressableBody::default()
                    },
                    ..Domain::default()
                },
            ],
            ..Block::default()
        })],
    };

    let ralf = serialize_document(&document);

    assert!(ralf.contains("domain ahb {"));
    assert!(ralf.contains("register flags=ahb_flags read;"));
    assert!(ralf.contains("domain pci {"));
    assert!(ralf.contains("regfile chan[16] @'h100 +'h10;"));
    assert!(ralf.contains("default_map_name frontdoor;"));
    assert!(ralf.contains("user_code lang=SV (new) {"));
    assert!(
        ralf.contains("add_reg_cb ahb_flags broadcast_reg_cb ({DATA0, DATA1}) external_cb_class;")
    );
}
