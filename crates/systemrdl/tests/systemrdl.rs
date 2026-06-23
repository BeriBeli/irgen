use irgen_snapsheet::model::{
    Block as BaseBlock, Component as BaseComponent, Field as BaseField, Register as BaseRegister,
    RegisterFile as BaseRegisterFile,
};
use irgen_systemrdl::*;

#[test]
fn serializes_core_systemrdl_model() {
    let field = Component {
        kind: ComponentKind::Field,
        name: "enable_t".into(),
        parameters: Vec::new(),
        properties: vec![
            PropertyAssignment::value("sw", Expression::Identifier("rw".into())),
            PropertyAssignment::value("hw", Expression::Identifier("r".into())),
            PropertyAssignment::value("desc", Expression::String("Enable bit".into())),
        ],
        children: Vec::new(),
        instances: Vec::new(),
    };
    let mut field_inst = Instance::new(field, "enable");
    field_inst.range = Some(BitRange {
        msb: Expression::Number("0".into()),
        lsb: None,
    });
    field_inst.reset = Some(Expression::Number("0".into()));

    let mut reg = Component::new(ComponentKind::Reg, "control_t");
    reg.properties.push(PropertyAssignment::value(
        "regwidth",
        Expression::Number("32".into()),
    ));
    reg.instances.push(field_inst);

    let mut reg_inst = Instance::new(reg, "control");
    reg_inst.address = Some(Expression::Number("0x4".into()));

    let mut addrmap = Component::new(ComponentKind::AddrMap, "top");
    addrmap
        .children
        .push(ComponentChild::Property(PropertyAssignment::bool(
            "littleendian",
        )));
    addrmap
        .children
        .push(ComponentChild::Constraint(Constraint {
            name: Some("legal".into()),
            body: "control.enable == 0;".into(),
        }));
    addrmap.instances.push(reg_inst);

    let document = Document {
        package: Some("csr_pkg".into()),
        imports: vec![Import {
            path: "common_pkg".into(),
            wildcard: true,
        }],
        declarations: vec![
            Declaration::Enum(EnumDecl {
                name: "mode_e".into(),
                variants: vec![EnumVariant {
                    name: "IDLE".into(),
                    value: Some(Expression::Number("0".into())),
                }],
            }),
            Declaration::Struct(StructDecl {
                name: "meta_t".into(),
                fields: vec![StructField {
                    ty: "string".into(),
                    name: "owner".into(),
                }],
            }),
            Declaration::Property(PropertyDecl {
                name: "rtl_path".into(),
                ty: PropertyType::String,
                component_kinds: vec![ComponentKind::Reg, ComponentKind::Field],
                default: None,
            }),
            Declaration::Component(addrmap),
        ],
    };

    let rdl = serialize_document(&document);

    assert!(rdl.contains("package csr_pkg;"));
    assert!(rdl.contains("import common_pkg::*;"));
    assert!(rdl.contains("enum mode_e {"));
    assert!(rdl.contains("struct meta_t {"));
    assert!(rdl.contains("property rtl_path"));
    assert!(rdl.contains("addrmap top {"));
    assert!(rdl.contains("littleendian;"));
    assert!(rdl.contains("constraint legal {"));
    assert!(rdl.contains("reg control_t {"));
    assert!(rdl.contains("field enable_t {"));
    assert!(rdl.contains("enable[0] = 0;"));
    assert!(rdl.contains("control @ 0x4;"));
}

#[test]
fn converts_base_component_to_systemrdl() {
    let component = BaseComponent::new(
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
                        "0x1".into(),
                        "ready flag".into(),
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

    let rdl = serialize_systemrdl(&component).unwrap();

    assert!(rdl.contains("addrmap example {"));
    assert!(rdl.contains("addrmap regs {"));
    assert!(rdl.contains("reg status {"));
    assert!(rdl.contains("field ready {"));
    assert!(rdl.contains("hdl_path_slice = '{\"u_status.ready_q\"};"));
    assert!(!rdl.contains("desc ="));
    assert!(!rdl.contains("hdl_path_slice = '{\"reserved0\"};"));
    assert!(rdl.contains("sw = r;"));
    assert!(rdl.contains("ready[0:0] = 0x1;"));
    assert!(rdl.contains("status @ 0x4;"));
    assert!(rdl.contains("regs @ 0x0;"));
}

#[test]
fn converts_register_file_arrays() {
    let component = BaseComponent::new(
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
                "0x100".into(),
                "4".into(),
                vec![
                    BaseRegister::new(
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
                    ),
                    BaseRegister::new(
                        "tail".into(),
                        "0xec".into(),
                        "32".into(),
                        vec![BaseField::new(
                            "done".into(),
                            "0".into(),
                            "1".into(),
                            "RO".into(),
                            "0".into(),
                            "".into(),
                        )],
                    ),
                ],
            )],
        )],
    );

    let rdl = serialize_systemrdl(&component).unwrap();

    assert!(rdl.contains("regfile lane {"));
    assert!(rdl.contains("lane[4] @ 0x0 += 0x100;"));
    assert!(!rdl.contains("lane[4] @ 0x10 += 0x100;"));
    assert!(!rdl.contains("lane_last"));
    assert!(rdl.contains("control @ 0x10;"));
    assert!(rdl.contains("tail @ 0xfc;"));
}

#[test]
fn preserves_absolute_register_file_array_offsets_when_stride_is_before_base() {
    let component = BaseComponent::new(
        "example.com".into(),
        "IP".into(),
        "example".into(),
        "1.0".into(),
        vec![BaseBlock::new_with_register_files(
            "regs".into(),
            "0x0".into(),
            "0x10000".into(),
            "32".into(),
            vec![],
            vec![BaseRegisterFile::new(
                "table".into(),
                "0x8000".into(),
                "0x10".into(),
                "4".into(),
                vec![BaseRegister::new(
                    "entry".into(),
                    "0x0".into(),
                    "32".into(),
                    vec![BaseField::new(
                        "value".into(),
                        "0".into(),
                        "32".into(),
                        "RW".into(),
                        "0".into(),
                        "".into(),
                    )],
                )],
            )],
        )],
    );

    let rdl = serialize_systemrdl(&component).unwrap();

    assert!(rdl.contains("table[4] @ 0x8000 += 0x10;"));
    assert!(rdl.contains("entry @ 0x0;"));
    assert!(!rdl.contains("table[4] @ 0x0 += 0x10;"));
    assert!(!rdl.contains("entry @ 0x8000;"));
}
