use std::collections::BTreeMap;

use askama::Template;

use crate::model::{
    AccessPolicy, AddressBlock, AlternateRegister, Component, EnumeratedValue, Field, ModeRef,
    Register, RegisterFile, SubspaceMap,
};
use crate::{Error, Result};

#[derive(Template)]
#[template(path = "package.sv", escape = "none")]
struct PackageTemplate<'a> {
    guard: &'a str,
    register_classes: &'a [RegisterClass],
    register_file_classes: &'a [RegisterFileClass],
    block_classes: &'a [BlockClass],
    alias_classes: &'a [AliasClass],
}

#[derive(Debug)]
struct RegisterClass {
    class_name: String,
    default_name: String,
    size_bits: u64,
    metadata_params: Vec<ConstraintParamView>,
    fields: Vec<FieldView>,
}

#[derive(Debug)]
struct AliasClass {
    class_name: String,
    base_class_name: String,
    default_name: String,
}

#[derive(Debug)]
struct FieldView {
    var_name: String,
    create_name: String,
    enum_type_name: String,
    enum_msb: u64,
    has_enum_values: bool,
    enum_values: Vec<EnumValueView>,
    has_constraint_params: bool,
    constraint_params: Vec<ConstraintParamView>,
    has_policy_params: bool,
    policy_params: Vec<ConstraintParamView>,
    width: u64,
    lsb: u64,
    access: String,
    volatile: &'static str,
    reset_literal: String,
    has_reset: &'static str,
    is_rand: &'static str,
    extra_resets: Vec<ResetView>,
}

#[derive(Debug)]
struct ResetView {
    value_literal: String,
    kind: String,
}

#[derive(Debug)]
struct EnumValueView {
    name: String,
    literal: String,
    usage: String,
    has_usage: bool,
}

#[derive(Debug)]
struct ConstraintParamView {
    name: String,
    type_expr: String,
    value_expr: String,
}

#[derive(Debug)]
struct BlockClass {
    class_name: String,
    default_name: String,
    maps: Vec<MapInstance>,
    metadata_params: Vec<ConstraintParamView>,
    memories: Vec<MemoryInstance>,
    reg_files: Vec<RegisterFileInstance>,
    instances: Vec<RegisterInstance>,
    arrays: Vec<RegisterArray>,
    child_blocks: Vec<ChildBlockInstance>,
    submaps: Vec<SubmapInstance>,
}

#[derive(Debug)]
struct RegisterFileClass {
    class_name: String,
    default_name: String,
    declarations: Vec<String>,
    build_code: String,
    map_code: String,
}

#[derive(Debug)]
struct ChildBlockInstance {
    var_name: String,
    class_name: String,
    create_name: String,
    map_var_name: String,
    offset_literal: String,
    hdl_path_expr: String,
}

#[derive(Debug)]
struct SubmapInstance {
    var_name: String,
    class_name: String,
    create_name: String,
    map_var_name: String,
    offset_literal: String,
}

#[derive(Debug)]
struct MapInstance {
    var_name: String,
    create_name: String,
    n_bytes: u64,
    byte_addressing: &'static str,
    is_default: bool,
}

#[derive(Debug)]
struct MemoryInstance {
    var_name: String,
    create_name: String,
    map_var_name: String,
    size_words: u64,
    width_bits: u64,
    offset_literal: String,
    rights: String,
    hdl_path_expr: String,
}

#[derive(Debug)]
struct RegisterFileInstance {
    var_name: String,
    class_name: String,
    declaration_suffix: String,
    build_code: String,
}

#[derive(Debug)]
struct RegisterInstance {
    var_name: String,
    class_name: String,
    create_name: String,
    configure_args: String,
    map_var_name: String,
    offset_literal: String,
    rights: String,
    hdl_slices: Vec<HdlSlice>,
}

#[derive(Debug)]
struct RegisterArray {
    var_name: String,
    class_name: String,
    declaration_suffix: String,
    build_code: String,
}

#[derive(Debug)]
struct HdlSlice {
    path_expr: String,
    offset: i64,
    size: i64,
    first: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct MapLayout {
    n_bytes: u64,
    byte_addressing: bool,
}

#[derive(Debug, Clone)]
struct MapContext {
    var_name: String,
    layout: MapLayout,
}

pub fn serialize_uvm_reg(component: &Component) -> Result<String> {
    let guard = format!("RAL_{}_SV", ident(&component.name).to_ascii_uppercase());
    let register_classes = register_classes(component)?;
    let register_file_classes = register_file_classes(component)?;
    let block_classes = block_classes(component)?;
    let alias_classes = vec![AliasClass {
        class_name: format!("ral_sys_{}", ident(&component.name)),
        base_class_name: format!("uvmreg_block_{}", ident(&component.name)),
        default_name: component.name.clone(),
    }];
    Ok(PackageTemplate {
        guard: &guard,
        register_classes: &register_classes,
        register_file_classes: &register_file_classes,
        block_classes: &block_classes,
        alias_classes: &alias_classes,
    }
    .render()?)
}

fn register_classes(component: &Component) -> Result<Vec<RegisterClass>> {
    let mut classes = Vec::new();
    for block in &component.blocks {
        register_classes_for_block(component, block, &mut classes)?;
    }
    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            register_classes_for_block(component, block, &mut classes)?;
        }
    }
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &address_space.blocks {
            register_classes_for_block(&scoped_component, block, &mut classes)?;
        }
    }
    Ok(classes)
}

fn register_file_classes(component: &Component) -> Result<Vec<RegisterFileClass>> {
    let mut classes = Vec::new();
    for block in &component.blocks {
        register_file_classes_for_block(component, block, &mut classes)?;
    }
    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            register_file_classes_for_block(component, block, &mut classes)?;
        }
    }
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &address_space.blocks {
            register_file_classes_for_block(&scoped_component, block, &mut classes)?;
        }
    }
    Ok(classes)
}

fn register_file_classes_for_block(
    component: &Component,
    block: &AddressBlock,
    classes: &mut Vec<RegisterFileClass>,
) -> Result<()> {
    for register_file in &block.register_files {
        classes.push(register_file_class(component, block, register_file)?);
    }
    Ok(())
}

fn register_file_class(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
) -> Result<RegisterFileClass> {
    let class_name = register_file_class_name(component, block, register_file);
    let mut declarations = Vec::new();
    let mut build_lines = Vec::new();
    let mut map_lines = Vec::new();
    let mut used_names = Vec::new();
    let layout = map_layout_for_block(block)?;

    for register in &register_file.registers {
        let register_dims = if is_array_dim(&register.dim)? {
            parse_dims("register dim", &register.dims)?
        } else {
            Vec::new()
        };
        let var_name = unique_ident(&register.name, &mut used_names);
        let class_name =
            register_file_register_class_name(component, block, register_file, register);
        declarations.push(format!(
            "    rand {class_name} {var_name}{};",
            array_declaration_suffix(&register_dims)
        ));
        let hdl_slices = hdl_slices(
            register,
            register
                .hdl_path
                .as_ref()
                .or(register_file.hdl_path.as_ref())
                .or(block.hdl_path.as_ref()),
        )?;
        build_lines.extend(register_file_member_build_lines(
            &var_name,
            &class_name,
            &register.name,
            &register_dims,
            &hdl_slices,
        ));
        let register_offset = map_offset_units(
            block,
            "register addressOffset",
            &register.address_offset,
            layout,
        )?;
        let offset_groups = if register_dims.is_empty() {
            Vec::new()
        } else {
            vec![ArrayOffsetGroup {
                first_dimension: 0,
                dimension_count: register_dims.len(),
                stride: register_stride(block, register, layout)?,
            }]
        };
        map_lines.extend(register_file_member_map_lines(
            &var_name,
            &register_dims,
            register_offset,
            &offset_groups,
            &register_rights(block, register),
        ));

        for alternate in &register.alternate_registers {
            let var_name = unique_ident(&alternate.name, &mut used_names);
            let class_name = register_file_alternate_class_name(
                component,
                block,
                register_file,
                register,
                alternate,
            );
            declarations.push(format!(
                "    rand {class_name} {var_name}{};",
                array_declaration_suffix(&register_dims)
            ));
            let hdl_slices = hdl_slices_from_fields(
                &alternate.fields,
                alternate
                    .hdl_path
                    .as_ref()
                    .or(register.hdl_path.as_ref())
                    .or(register_file.hdl_path.as_ref())
                    .or(block.hdl_path.as_ref()),
            )?;
            build_lines.extend(register_file_member_build_lines(
                &var_name,
                &class_name,
                &alternate.name,
                &register_dims,
                &hdl_slices,
            ));
            let rights =
                register_rights_from_fields(block, alternate.access.as_deref(), &alternate.fields);
            map_lines.extend(register_file_member_map_lines(
                &var_name,
                &register_dims,
                register_offset,
                &offset_groups,
                &rights,
            ));
        }
    }

    Ok(RegisterFileClass {
        class_name,
        default_name: register_file.name.clone(),
        declarations,
        build_code: build_lines.join("\n"),
        map_code: map_lines.join("\n"),
    })
}

fn register_classes_for_block(
    component: &Component,
    block: &AddressBlock,
    classes: &mut Vec<RegisterClass>,
) -> Result<()> {
    for register in &block.registers {
        classes.push(register_class(
            &component.name,
            block,
            &[block.name.as_str(), register.name.as_str()],
            &register.name,
            &register.size,
            &register.description,
            register.volatile.as_deref(),
            register.access.as_deref(),
            &register.fields,
            access_policy_metadata_params("ACCESS_POLICY", &register.access_policies),
        )?);
        for alternate in &register.alternate_registers {
            classes.push(register_class(
                &component.name,
                block,
                &[
                    block.name.as_str(),
                    register.name.as_str(),
                    alternate.name.as_str(),
                ],
                &alternate.name,
                &register.size,
                &alternate.description,
                alternate.volatile.as_deref(),
                alternate.access.as_deref(),
                &alternate.fields,
                {
                    let mut params = alternate_metadata_params(alternate);
                    params.extend(access_policy_metadata_params(
                        "ACCESS_POLICY",
                        &alternate.access_policies,
                    ));
                    params
                },
            )?);
        }
    }
    for register_file in &block.register_files {
        for register in &register_file.registers {
            classes.push(register_class(
                &component.name,
                block,
                &[
                    block.name.as_str(),
                    register_file.name.as_str(),
                    register.name.as_str(),
                ],
                &register.name,
                &register.size,
                &register.description,
                register.volatile.as_deref(),
                register.access.as_deref(),
                &register.fields,
                access_policy_metadata_params("ACCESS_POLICY", &register.access_policies),
            )?);
            for alternate in &register.alternate_registers {
                classes.push(register_class(
                    &component.name,
                    block,
                    &[
                        block.name.as_str(),
                        register_file.name.as_str(),
                        register.name.as_str(),
                        alternate.name.as_str(),
                    ],
                    &alternate.name,
                    &register.size,
                    &alternate.description,
                    alternate.volatile.as_deref(),
                    alternate.access.as_deref(),
                    &alternate.fields,
                    {
                        let mut params = alternate_metadata_params(alternate);
                        params.extend(access_policy_metadata_params(
                            "ACCESS_POLICY",
                            &alternate.access_policies,
                        ));
                        params
                    },
                )?);
            }
        }
    }
    Ok(())
}

fn register_class(
    component_name: &str,
    block: &AddressBlock,
    path_parts: &[&str],
    default_name: &str,
    size: &str,
    description: &str,
    register_volatile: Option<&str>,
    register_access: Option<&str>,
    fields: &[Field],
    mut metadata_params: Vec<ConstraintParamView>,
) -> Result<RegisterClass> {
    let size_bits = parse_u64("register size", size)?;
    let class_name = format!(
        "uvmreg_reg_{}_{}",
        ident(component_name),
        path_parts
            .iter()
            .map(|part| ident(part))
            .collect::<Vec<_>>()
            .join("_")
    );
    let mut used_field_names = Vec::new();
    let mut used_enum_value_names = Vec::new();
    let fields = fields
        .iter()
        .map(|field| {
            field_view(
                block,
                register_volatile,
                register_access,
                field,
                &mut used_field_names,
                &mut used_enum_value_names,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    if let Some(param) = description_param("REGISTER_DESCRIPTION", description) {
        metadata_params.insert(0, param);
    }

    Ok(RegisterClass {
        class_name,
        default_name: default_name.into(),
        size_bits,
        metadata_params,
        fields,
    })
}

fn field_view(
    block: &AddressBlock,
    register_volatile: Option<&str>,
    register_access: Option<&str>,
    field: &Field,
    used_names: &mut Vec<String>,
    used_enum_value_names: &mut Vec<String>,
) -> Result<FieldView> {
    let width = parse_u64("field bitWidth", &field.bit_width)?;
    let default_reset_index = default_reset_index(field);
    let reset_value = default_reset_index
        .map(|index| parse_u64("field reset", &field.resets[index].value))
        .transpose()?
        .unwrap_or(0);
    let extra_resets = extra_reset_views(field, width, default_reset_index)?;
    let access = uvm_access(effective_access(block, register_access, field), field);
    let var_name = unique_ident(&field.name, used_names);
    let enum_values = enum_value_views(
        &field.name,
        &field.enumerated_values,
        width,
        used_enum_value_names,
    )?;
    let param_prefix = var_name.to_ascii_uppercase();
    let constraint_params = constraint_param_views(&param_prefix, field, width)?;
    let policy_params = policy_param_views(&param_prefix, field, width)?;
    Ok(FieldView {
        enum_type_name: format!("{var_name}_e"),
        enum_msb: width.saturating_sub(1),
        has_enum_values: !enum_values.is_empty(),
        enum_values,
        has_constraint_params: !constraint_params.is_empty(),
        constraint_params,
        has_policy_params: !policy_params.is_empty(),
        policy_params,
        var_name,
        create_name: sv_string(&field.name),
        width,
        lsb: parse_u64("field bitOffset", &field.bit_offset)?,
        access: sv_string(&access),
        volatile: sv_bool_literal(inherited_volatile(block, register_volatile, field)),
        reset_literal: format!("{width}'h{reset_value:x}"),
        has_reset: sv_bool_literal(default_reset_index.is_some()),
        is_rand: sv_bool_literal(is_writable_access(&access)),
        extra_resets,
    })
}

fn default_reset_index(field: &Field) -> Option<usize> {
    field.resets.iter().position(|reset| {
        reset
            .reset_type
            .as_deref()
            .map(|reset_type| reset_type.eq_ignore_ascii_case("HARD"))
            .unwrap_or(true)
    })
}

fn extra_reset_views(
    field: &Field,
    width: u64,
    default_reset_index: Option<usize>,
) -> Result<Vec<ResetView>> {
    field
        .resets
        .iter()
        .enumerate()
        .filter(|(index, _)| Some(*index) != default_reset_index)
        .map(|(_, reset)| {
            Ok(ResetView {
                value_literal: bit_literal("field reset", width, &reset.value)?,
                kind: sv_string(reset.reset_type.as_deref().unwrap_or("HARD")),
            })
        })
        .collect()
}

fn constraint_param_views(
    prefix: &str,
    field: &Field,
    width: u64,
) -> Result<Vec<ConstraintParamView>> {
    let Some(constraint) = &field.write_value_constraint else {
        return Ok(Vec::new());
    };

    let mut params = Vec::new();
    if let Some(write_as_read) = &constraint.write_as_read {
        params.push(ConstraintParamView {
            name: format!("{prefix}_WRITE_AS_READ"),
            type_expr: "bit".into(),
            value_expr: sv_bool_expr(write_as_read),
        });
    }
    if let Some(use_enumerated_values) = &constraint.use_enumerated_values {
        params.push(ConstraintParamView {
            name: format!("{prefix}_USE_ENUMERATED_VALUES"),
            type_expr: "bit".into(),
            value_expr: sv_bool_expr(use_enumerated_values),
        });
    }
    if let Some(minimum) = &constraint.minimum {
        params.push(ConstraintParamView {
            name: format!("{prefix}_MINIMUM"),
            type_expr: format!("bit [{}:0]", width.saturating_sub(1)),
            value_expr: bit_literal("writeValueConstraint minimum", width, minimum)?,
        });
    }
    if let Some(maximum) = &constraint.maximum {
        params.push(ConstraintParamView {
            name: format!("{prefix}_MAXIMUM"),
            type_expr: format!("bit [{}:0]", width.saturating_sub(1)),
            value_expr: bit_literal("writeValueConstraint maximum", width, maximum)?,
        });
    }
    Ok(params)
}

fn policy_param_views(prefix: &str, field: &Field, width: u64) -> Result<Vec<ConstraintParamView>> {
    let mut params = Vec::new();
    if let Some(param) = description_param(&format!("{prefix}_DESCRIPTION"), &field.description) {
        params.push(param);
    }
    params.extend(access_policy_metadata_params(
        &format!("{prefix}_ACCESS_POLICY"),
        &field.access_policies,
    ));
    for (index, reset) in field.resets.iter().enumerate() {
        if let Some(reset_type) = &reset.reset_type {
            params.push(ConstraintParamView {
                name: format!("{prefix}_RESET_{index}_KIND"),
                type_expr: "string".into(),
                value_expr: sv_string(reset_type),
            });
        }
        if let Some(mask) = &reset.mask {
            params.push(ConstraintParamView {
                name: format!("{prefix}_RESET_{index}_MASK"),
                type_expr: format!("bit [{}:0]", width.saturating_sub(1)),
                value_expr: bit_literal("field reset mask", width, mask)?,
            });
        }
    }
    if let Some(testable) = &field.testable {
        params.push(ConstraintParamView {
            name: format!("{prefix}_TESTABLE"),
            type_expr: "bit".into(),
            value_expr: sv_bool_expr(&testable.value),
        });
        if let Some(test_constraint) = &testable.test_constraint {
            params.push(ConstraintParamView {
                name: format!("{prefix}_TEST_CONSTRAINT"),
                type_expr: "string".into(),
                value_expr: sv_string(test_constraint),
            });
        }
    }
    if let Some(reserved) = &field.reserved {
        params.push(ConstraintParamView {
            name: format!("{prefix}_RESERVED"),
            type_expr: "bit".into(),
            value_expr: sv_bool_expr(reserved),
        });
    }
    for (index, restriction) in field.access_restrictions.iter().enumerate() {
        let mode_refs = restriction
            .mode_refs
            .iter()
            .map(|mode_ref| match &mode_ref.priority {
                Some(priority) => format!("{}:{}", mode_ref.name, priority),
                None => mode_ref.name.clone(),
            })
            .collect::<Vec<_>>()
            .join(",");
        if !mode_refs.is_empty() {
            params.push(ConstraintParamView {
                name: format!("{prefix}_ACCESS_RESTRICTION_{index}_MODES"),
                type_expr: "string".into(),
                value_expr: sv_string(&mode_refs),
            });
        }
        if let Some(read_mask) = &restriction.read_access_mask {
            params.push(ConstraintParamView {
                name: format!("{prefix}_ACCESS_RESTRICTION_{index}_READ_MASK"),
                type_expr: format!("bit [{}:0]", width.saturating_sub(1)),
                value_expr: bit_literal("accessRestriction readAccessMask", width, read_mask)?,
            });
        }
        if let Some(write_mask) = &restriction.write_access_mask {
            params.push(ConstraintParamView {
                name: format!("{prefix}_ACCESS_RESTRICTION_{index}_WRITE_MASK"),
                type_expr: format!("bit [{}:0]", width.saturating_sub(1)),
                value_expr: bit_literal("accessRestriction writeAccessMask", width, write_mask)?,
            });
        }
    }
    for (index, broadcast) in field.broadcasts.iter().enumerate() {
        let target = broadcast
            .target
            .iter()
            .map(|segment| format!("{}={}", segment.kind, segment.name))
            .collect::<Vec<_>>()
            .join(".");
        params.push(ConstraintParamView {
            name: format!("{prefix}_BROADCAST_{index}"),
            type_expr: "string".into(),
            value_expr: sv_string(&target),
        });
    }
    Ok(params)
}

fn enum_value_views(
    field_name: &str,
    values: &[EnumeratedValue],
    width: u64,
    used_names: &mut Vec<String>,
) -> Result<Vec<EnumValueView>> {
    values
        .iter()
        .map(|value| {
            let parsed = parse_u64("enumeratedValue value", &value.value)?;
            let usage = value.usage.clone().unwrap_or_default();
            Ok(EnumValueView {
                name: unique_const_ident(&format!("{field_name}_{}", value.name), used_names),
                literal: format!("{width}'h{parsed:x}"),
                has_usage: !usage.is_empty(),
                usage,
            })
        })
        .collect()
}

fn block_classes(component: &Component) -> Result<Vec<BlockClass>> {
    let mut classes = Vec::new();
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &scoped_component.blocks {
            classes.push(address_block_class(&scoped_component, block)?);
        }
        classes.push(block_class(&scoped_component, Vec::new())?);
    }
    for block in &component.blocks {
        classes.push(address_block_class(component, block)?);
    }
    classes.push(block_class(component, submap_instances(component)?)?);
    Ok(classes)
}

fn scoped_component(component: &Component, suffix: &str, blocks: &[AddressBlock]) -> Component {
    Component {
        vendor: component.vendor.clone(),
        library: component.library.clone(),
        name: format!("{}_{}", component.name, suffix),
        version: component.version.clone(),
        address_spaces: Vec::new(),
        blocks: blocks.to_vec(),
        subspace_maps: Vec::new(),
        memory_remaps: Vec::new(),
    }
}

fn block_class(component: &Component, submaps: Vec<SubmapInstance>) -> Result<BlockClass> {
    let class_name = format!("uvmreg_block_{}", ident(&component.name));
    let mut memories = Vec::new();
    let mut reg_files = Vec::new();
    let mut instances = Vec::new();
    let mut arrays = Vec::new();
    let mut used_names = Vec::new();
    let maps = map_instances(component)?;
    let layouts = map_layouts(component, &maps);
    let child_blocks = child_block_instances(component, &layouts)?;

    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            let map = layouts
                .get(&block.map_name)
                .ok_or_else(|| Error::MissingElement("memoryMap"))?;
            block_instances(
                component,
                block,
                map.layout,
                &map.var_name,
                &mut memories,
                &mut reg_files,
                &mut instances,
                &mut arrays,
                &mut used_names,
                true,
                true,
            )?;
        }
    }

    Ok(BlockClass {
        class_name,
        default_name: component.name.clone(),
        maps,
        metadata_params: component_metadata_params(component),
        memories,
        reg_files,
        instances,
        arrays,
        child_blocks,
        submaps,
    })
}

fn address_block_class(component: &Component, block: &AddressBlock) -> Result<BlockClass> {
    let class_name = format!(
        "uvmreg_block_{}_{}",
        ident(&component.name),
        ident(&block.name)
    );
    let mut memories = Vec::new();
    let mut reg_files = Vec::new();
    let mut instances = Vec::new();
    let mut arrays = Vec::new();
    let mut used_names = Vec::new();
    let layout = map_layout_for_block(block)?;
    let maps = vec![MapInstance {
        var_name: "default_map".into(),
        create_name: sv_string("default_map"),
        n_bytes: layout.n_bytes,
        byte_addressing: sv_bool_literal(layout.byte_addressing),
        is_default: true,
    }];

    block_instances(
        component,
        block,
        layout,
        "default_map",
        &mut memories,
        &mut reg_files,
        &mut instances,
        &mut arrays,
        &mut used_names,
        false,
        false,
    )?;

    Ok(BlockClass {
        class_name,
        default_name: block.name.clone(),
        maps,
        metadata_params: block_metadata_params(block),
        memories,
        reg_files,
        instances,
        arrays,
        child_blocks: Vec::new(),
        submaps: Vec::new(),
    })
}

fn child_block_instances(
    component: &Component,
    layouts: &BTreeMap<String, MapContext>,
) -> Result<Vec<ChildBlockInstance>> {
    let mut children = Vec::new();
    let mut used_names = Vec::new();
    for block in &component.blocks {
        let map = layouts
            .get(&block.map_name)
            .ok_or_else(|| Error::MissingElement("memoryMap"))?;
        let offset = map_offset_units(
            block,
            "addressBlock baseAddress",
            &block.base_address,
            map.layout,
        )?;
        children.push(ChildBlockInstance {
            var_name: unique_ident(&block.name, &mut used_names),
            class_name: format!(
                "uvmreg_block_{}_{}",
                ident(&component.name),
                ident(&block.name)
            ),
            create_name: sv_string(&block.name),
            map_var_name: map.var_name.clone(),
            offset_literal: addr_literal(offset),
            hdl_path_expr: block
                .hdl_path
                .as_ref()
                .map(|path| hdl_path_expr(None, path))
                .unwrap_or_else(|| sv_string("")),
        });
    }
    Ok(children)
}

fn submap_instances(component: &Component) -> Result<Vec<SubmapInstance>> {
    let mut submaps = Vec::new();
    let mut used_names = Vec::new();
    let maps = map_instances(component)?;
    let layouts = map_layouts(component, &maps);
    let address_space_names = component
        .address_spaces
        .iter()
        .map(|address_space| address_space.name.as_str())
        .collect::<Vec<_>>();

    for subspace in component.subspace_maps.iter().chain(
        component
            .memory_remaps
            .iter()
            .flat_map(|remap| remap.subspace_maps.iter()),
    ) {
        let Some(address_space_ref) = subspace.address_space_ref.as_deref() else {
            continue;
        };
        if !address_space_names.contains(&address_space_ref) {
            continue;
        }
        let map = layouts
            .get(&subspace.map_name)
            .ok_or_else(|| Error::MissingElement("memoryMap"))?;
        let offset = map_offset_units_for_address_unit_bits(
            "subspaceMap baseAddress",
            &subspace.base_address,
            map.layout,
            parse_u64("memoryMap addressUnitBits", &subspace.address_unit_bits)?,
        )?;
        let var_name = unique_ident(&subspace.name, &mut used_names);
        submaps.push(SubmapInstance {
            class_name: format!(
                "uvmreg_block_{}_{}",
                ident(&component.name),
                ident(address_space_ref)
            ),
            create_name: sv_string(&subspace.name),
            map_var_name: map.var_name.clone(),
            offset_literal: addr_literal(offset),
            var_name,
        });
    }

    Ok(submaps)
}

fn block_instances(
    component: &Component,
    block: &AddressBlock,
    layout: MapLayout,
    map_var_name: &str,
    memories: &mut Vec<MemoryInstance>,
    reg_files: &mut Vec<RegisterFileInstance>,
    instances: &mut Vec<RegisterInstance>,
    arrays: &mut Vec<RegisterArray>,
    used_names: &mut Vec<String>,
    include_block_base: bool,
    prefix_with_block: bool,
) -> Result<()> {
    let block_base = if include_block_base {
        map_offset_units(
            block,
            "addressBlock baseAddress",
            &block.base_address,
            layout,
        )?
    } else {
        0
    };
    if is_memory_block(block) {
        memories.push(memory_instance(
            block,
            block_base,
            map_var_name,
            used_names,
        )?);
    }
    for register in &block.registers {
        let offset = block_base
            + map_offset_units(
                block,
                "register addressOffset",
                &register.address_offset,
                layout,
            )?;
        if is_array_dim(&register.dim)? {
            arrays.push(register_array(
                component,
                block,
                register,
                offset,
                layout,
                map_var_name,
                used_names,
                prefix_with_block,
            )?);
            for alternate in &register.alternate_registers {
                arrays.push(alternate_register_array(
                    component,
                    block,
                    register,
                    alternate,
                    offset,
                    layout,
                    map_var_name,
                    used_names,
                    prefix_with_block,
                )?);
            }
        } else {
            instances.push(register_instance(
                component,
                block,
                register,
                offset,
                map_var_name,
                used_names,
                prefix_with_block,
            )?);
            for alternate in &register.alternate_registers {
                instances.push(alternate_register_instance(
                    component,
                    block,
                    register,
                    alternate,
                    offset,
                    map_var_name,
                    used_names,
                    prefix_with_block,
                )?);
            }
        }
    }
    for register_file in &block.register_files {
        reg_files.push(register_file_instance_view(
            component,
            block,
            register_file,
            block_base,
            layout,
            map_var_name,
            used_names,
            prefix_with_block,
        )?);
    }
    Ok(())
}

fn register_class_name(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: Option<&AlternateRegister>,
) -> String {
    match alternate {
        Some(alternate) => alternate_register_class_name(component, block, register, alternate),
        None => format!(
            "uvmreg_reg_{}_{}_{}",
            ident(&component.name),
            ident(&block.name),
            ident(&register.name)
        ),
    }
}

fn alternate_register_class_name(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: &AlternateRegister,
) -> String {
    format!(
        "uvmreg_reg_{}_{}_{}_{}",
        ident(&component.name),
        ident(&block.name),
        ident(&register.name),
        ident(&alternate.name)
    )
}

fn register_file_register_class_name(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    register: &Register,
) -> String {
    format!(
        "uvmreg_reg_{}_{}_{}_{}",
        ident(&component.name),
        ident(&block.name),
        ident(&register_file.name),
        ident(&register.name)
    )
}

fn register_file_class_name(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
) -> String {
    format!(
        "uvmreg_regfile_{}_{}_{}",
        ident(&component.name),
        ident(&block.name),
        ident(&register_file.name)
    )
}

fn register_file_alternate_class_name(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    register: &Register,
    alternate: &AlternateRegister,
) -> String {
    format!(
        "uvmreg_reg_{}_{}_{}_{}_{}",
        ident(&component.name),
        ident(&block.name),
        ident(&register_file.name),
        ident(&register.name),
        ident(&alternate.name)
    )
}

fn component_metadata_params(component: &Component) -> Vec<ConstraintParamView> {
    let mut params = Vec::new();
    for (index, block) in all_blocks(component).enumerate() {
        params.push(ConstraintParamView {
            name: format!("BLOCK_{index}_NAME"),
            type_expr: "string".into(),
            value_expr: sv_string(&block.name),
        });
        if let Some(param) =
            description_param(&format!("BLOCK_{index}_DESCRIPTION"), &block.description)
        {
            params.push(param);
        }
        params.extend(access_policy_metadata_params(
            &format!("BLOCK_{index}_ACCESS_POLICY"),
            &block.access_policies,
        ));
        for (file_index, register_file) in block.register_files.iter().enumerate() {
            params.push(ConstraintParamView {
                name: format!("BLOCK_{index}_REGISTER_FILE_{file_index}_NAME"),
                type_expr: "string".into(),
                value_expr: sv_string(&register_file.name),
            });
            if let Some(param) = description_param(
                &format!("BLOCK_{index}_REGISTER_FILE_{file_index}_DESCRIPTION"),
                &register_file.description,
            ) {
                params.push(param);
            }
        }
    }
    params.extend(subspace_metadata_params(
        "SUBSPACE",
        &component.subspace_maps,
    ));
    for (index, remap) in component.memory_remaps.iter().enumerate() {
        params.push(ConstraintParamView {
            name: format!("REMAP_{index}_NAME"),
            type_expr: "string".into(),
            value_expr: sv_string(&remap.name),
        });
        let mode_refs = mode_refs_string(&remap.mode_refs);
        if !mode_refs.is_empty() {
            params.push(ConstraintParamView {
                name: format!("REMAP_{index}_MODES"),
                type_expr: "string".into(),
                value_expr: sv_string(&mode_refs),
            });
        }
        params.extend(subspace_metadata_params(
            &format!("REMAP_{index}_SUBSPACE"),
            &remap.subspace_maps,
        ));
    }
    params
}

fn block_metadata_params(block: &AddressBlock) -> Vec<ConstraintParamView> {
    let mut params = Vec::new();
    params.push(ConstraintParamView {
        name: "ADDRESS_BLOCK_NAME".into(),
        type_expr: "string".into(),
        value_expr: sv_string(&block.name),
    });
    if let Some(param) = description_param("ADDRESS_BLOCK_DESCRIPTION", &block.description) {
        params.push(param);
    }
    params.extend(access_policy_metadata_params(
        "ADDRESS_BLOCK_ACCESS_POLICY",
        &block.access_policies,
    ));
    for (file_index, register_file) in block.register_files.iter().enumerate() {
        params.push(ConstraintParamView {
            name: format!("REGISTER_FILE_{file_index}_NAME"),
            type_expr: "string".into(),
            value_expr: sv_string(&register_file.name),
        });
        if let Some(param) = description_param(
            &format!("REGISTER_FILE_{file_index}_DESCRIPTION"),
            &register_file.description,
        ) {
            params.push(param);
        }
    }
    params
}

fn subspace_metadata_params(
    prefix: &str,
    subspace_maps: &[SubspaceMap],
) -> Vec<ConstraintParamView> {
    let mut params = Vec::new();
    for (index, subspace) in subspace_maps.iter().enumerate() {
        params.push(ConstraintParamView {
            name: format!("{prefix}_{index}_NAME"),
            type_expr: "string".into(),
            value_expr: sv_string(&subspace.name),
        });
        params.push(ConstraintParamView {
            name: format!("{prefix}_{index}_BASE"),
            type_expr: "string".into(),
            value_expr: sv_string(&subspace.base_address),
        });
        params.push(ConstraintParamView {
            name: format!("{prefix}_{index}_ADDRESS_UNIT_BITS"),
            type_expr: "int unsigned".into(),
            value_expr: subspace.address_unit_bits.clone(),
        });
        params.push(ConstraintParamView {
            name: format!("{prefix}_{index}_INITIATOR"),
            type_expr: "string".into(),
            value_expr: sv_string(&subspace.initiator_ref),
        });
        if let Some(segment_ref) = &subspace.segment_ref {
            params.push(ConstraintParamView {
                name: format!("{prefix}_{index}_SEGMENT"),
                type_expr: "string".into(),
                value_expr: sv_string(segment_ref),
            });
        }
    }
    params
}

fn is_array_dim(dim: &str) -> Result<bool> {
    Ok(parse_u64("array dim", dim)? > 1)
}

fn parse_dims(field: &'static str, dims: &[String]) -> Result<Vec<u64>> {
    dims.iter().map(|dim| parse_u64(field, dim)).collect()
}

fn map_instances(component: &Component) -> Result<Vec<MapInstance>> {
    let mut map_names = Vec::new();
    for block in all_blocks(component) {
        if !map_names.contains(&block.map_name) {
            map_names.push(block.map_name.clone());
        }
    }
    for subspace in &component.subspace_maps {
        if !map_names.contains(&subspace.map_name) {
            map_names.push(subspace.map_name.clone());
        }
    }
    for remap in &component.memory_remaps {
        if !map_names.contains(&remap.map_name) {
            map_names.push(remap.map_name.clone());
        }
        for subspace in &remap.subspace_maps {
            if !map_names.contains(&subspace.map_name) {
                map_names.push(subspace.map_name.clone());
            }
        }
    }

    let mut used_map_vars = Vec::new();
    map_names
        .iter()
        .enumerate()
        .map(|(index, map_name)| {
            let layout = map_layout(component, map_name)?;
            let (var_name, create_name, is_default) = if index == 0 {
                ("default_map".into(), "default_map".into(), true)
            } else {
                (
                    unique_ident(&format!("{map_name}_map"), &mut used_map_vars),
                    map_name.clone(),
                    false,
                )
            };
            Ok(MapInstance {
                var_name,
                create_name: sv_string(&create_name),
                n_bytes: layout.n_bytes,
                byte_addressing: sv_bool_literal(layout.byte_addressing),
                is_default,
            })
        })
        .collect()
}

fn map_layouts(component: &Component, maps: &[MapInstance]) -> BTreeMap<String, MapContext> {
    let mut map_names = Vec::new();
    for block in all_blocks(component) {
        if !map_names.contains(&block.map_name) {
            map_names.push(block.map_name.clone());
        }
    }
    for subspace in &component.subspace_maps {
        if !map_names.contains(&subspace.map_name) {
            map_names.push(subspace.map_name.clone());
        }
    }
    for remap in &component.memory_remaps {
        if !map_names.contains(&remap.map_name) {
            map_names.push(remap.map_name.clone());
        }
        for subspace in &remap.subspace_maps {
            if !map_names.contains(&subspace.map_name) {
                map_names.push(subspace.map_name.clone());
            }
        }
    }

    map_names
        .into_iter()
        .zip(maps.iter())
        .map(|(map_name, map)| {
            (
                map_name,
                MapContext {
                    var_name: map.var_name.clone(),
                    layout: MapLayout {
                        n_bytes: map.n_bytes,
                        byte_addressing: map.byte_addressing == "1'b1",
                    },
                },
            )
        })
        .collect()
}

fn all_blocks(component: &Component) -> impl Iterator<Item = &AddressBlock> {
    component.blocks.iter().chain(
        component
            .memory_remaps
            .iter()
            .flat_map(|remap| remap.blocks.iter()),
    )
}

fn map_layout(component: &Component, map_name: &str) -> Result<MapLayout> {
    let mut n_bytes = 4;
    let mut address_unit_bits = Vec::new();
    for block in all_blocks(component).filter(|block| block.map_name == map_name) {
        n_bytes = n_bytes.max(width_bytes(block)?);
        address_unit_bits.push(parse_u64(
            "memoryMap addressUnitBits",
            &block.address_unit_bits,
        )?);
    }

    let uniform_address_unit_bits = address_unit_bits
        .first()
        .copied()
        .filter(|first| address_unit_bits.iter().all(|bits| bits == first));
    let byte_addressing = match uniform_address_unit_bits {
        Some(8) | None => true,
        Some(bits) => address_unit_bytes(bits)? != n_bytes,
    };

    Ok(MapLayout {
        n_bytes,
        byte_addressing,
    })
}

fn map_layout_for_block(block: &AddressBlock) -> Result<MapLayout> {
    let n_bytes = width_bytes(block)?;
    let address_unit_bits = parse_u64("memoryMap addressUnitBits", &block.address_unit_bits)?;
    let byte_addressing = match address_unit_bits {
        8 => true,
        bits => address_unit_bytes(bits)? != n_bytes,
    };

    Ok(MapLayout {
        n_bytes,
        byte_addressing,
    })
}

fn address_unit_bytes(bits: u64) -> Result<u64> {
    if bits % 8 == 0 {
        Ok((bits / 8).max(1))
    } else {
        Err(Error::InvalidNumber {
            field: "memoryMap addressUnitBits",
            value: bits.to_string(),
        })
    }
}

fn map_offset_units(
    block: &AddressBlock,
    field: &'static str,
    value: &str,
    layout: MapLayout,
) -> Result<u64> {
    let address_unit_bits = parse_u64("memoryMap addressUnitBits", &block.address_unit_bits)?;
    map_offset_units_for_address_unit_bits(field, value, layout, address_unit_bits)
}

fn map_offset_units_for_address_unit_bits(
    field: &'static str,
    value: &str,
    layout: MapLayout,
    address_unit_bits: u64,
) -> Result<u64> {
    let parsed = parse_u64(field, value)?;
    if layout.byte_addressing {
        Ok(parsed * address_unit_bytes(address_unit_bits)?)
    } else {
        Ok(parsed)
    }
}

fn range_bytes(block: &AddressBlock, field: &'static str, value: &str) -> Result<u64> {
    Ok(parse_u64(field, value)?
        * address_unit_bytes(parse_u64(
            "memoryMap addressUnitBits",
            &block.address_unit_bits,
        )?)?)
}

fn memory_instance(
    block: &AddressBlock,
    offset: u64,
    map_var_name: &str,
    used_names: &mut Vec<String>,
) -> Result<MemoryInstance> {
    let width_bits = parse_u64("addressBlock width", &block.width)?;
    Ok(MemoryInstance {
        var_name: unique_ident(&block.name, used_names),
        create_name: sv_string(&block.name),
        map_var_name: map_var_name.into(),
        size_words: memory_size_words(block)?,
        width_bits,
        offset_literal: addr_literal(offset),
        rights: sv_string(&memory_rights(block)),
        hdl_path_expr: block
            .hdl_path
            .as_ref()
            .map(|path| hdl_path_expr(None, path))
            .unwrap_or_else(|| sv_string("")),
    })
}

fn register_file_instance_view(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    block_base: u64,
    layout: MapLayout,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
) -> Result<RegisterFileInstance> {
    let create_name = if prefix_with_block {
        format!("{}_{}", block.name, register_file.name)
    } else {
        register_file.name.clone()
    };
    let var_name = unique_ident(&create_name, used_names);
    let class_name = register_file_class_name(component, block, register_file);
    let dims = if is_array_dim(&register_file.dim)? {
        parse_dims("registerFile dim", &register_file.dims)?
    } else {
        Vec::new()
    };
    let reg_file_offset = block_base
        + map_offset_units(
            block,
            "registerFile addressOffset",
            &register_file.address_offset,
            layout,
        )?;
    let reg_file_stride = register_file_stride(block, register_file, layout)?;
    let hdl_path_expr = register_file
        .hdl_path
        .as_ref()
        .map(|path| hdl_path_expr(block.hdl_path.as_deref(), path))
        .or_else(|| {
            block
                .hdl_path
                .as_ref()
                .map(|path| hdl_path_expr(None, path))
        })
        .unwrap_or_else(|| sv_string(""));

    Ok(RegisterFileInstance {
        declaration_suffix: array_declaration_suffix(&dims),
        build_code: register_file_build_code(
            &var_name,
            &class_name,
            &create_name,
            &dims,
            &hdl_path_expr,
            map_var_name,
            reg_file_offset,
            reg_file_stride,
        ),
        var_name,
        class_name,
    })
}

fn register_file_build_code(
    var_name: &str,
    class_name: &str,
    create_name: &str,
    dims: &[u64],
    hdl_path_expr: &str,
    map_var_name: &str,
    base_offset: u64,
    stride: u64,
) -> String {
    if dims.is_empty() {
        return [
            format!(
                "      {var_name} = {class_name}::type_id::create({});",
                sv_string(create_name)
            ),
            format!("      {var_name}.configure(this, null, {hdl_path_expr});"),
            format!("      {var_name}.build();"),
            format!(
                "      {var_name}.map({map_var_name}, {});",
                addr_literal(base_offset)
            ),
        ]
        .join("\n");
    }

    let indices = loop_indices(dims.len());
    let element_ref = array_element_ref(var_name, &indices);
    let create_format = create_name_format(create_name, indices.len());
    let create_args = indices.join(", ");
    let linear_index = linear_index_expr(dims, &indices);
    let mut lines = Vec::new();
    for (level, (index, dim)) in indices.iter().zip(dims).enumerate() {
        lines.push(format!(
            "{}for (int unsigned {index} = 0; {index} < {dim}; {index}++) begin",
            indent(level)
        ));
    }
    let body_indent = indent(indices.len());
    lines.push(format!(
        "{body_indent}{element_ref} = {class_name}::type_id::create($sformatf({create_format}, {create_args}));"
    ));
    lines.push(format!(
        "{body_indent}{element_ref}.configure(this, null, {hdl_path_expr});"
    ));
    lines.push(format!("{body_indent}{element_ref}.build();"));
    lines.push(format!(
        "{body_indent}{element_ref}.map({map_var_name}, {} + {linear_index} * {});",
        addr_literal(base_offset),
        addr_literal(stride)
    ));
    for level in (0..indices.len()).rev() {
        lines.push(format!("{}end", indent(level)));
    }
    lines.join("\n")
}

fn register_file_member_build_lines(
    var_name: &str,
    class_name: &str,
    create_name: &str,
    dims: &[u64],
    hdl_slices: &[HdlSlice],
) -> Vec<String> {
    let mut lines = Vec::new();
    let indices = loop_indices(dims.len());
    if dims.is_empty() {
        lines.push(format!(
            "      {var_name} = {class_name}::type_id::create({});",
            sv_string(create_name)
        ));
        lines.push(format!("      {var_name}.configure(get_block(), this);"));
        lines.push(format!("      {var_name}.build();"));
        for slice in hdl_slices {
            lines.push(format!(
                "      {var_name}.add_hdl_path_slice({}, {}, {}, {});",
                slice.path_expr, slice.offset, slice.size, slice.first
            ));
        }
        return lines;
    }

    let element_ref = array_element_ref(var_name, &indices);
    let create_format = create_name_format(create_name, indices.len());
    let create_args = indices.join(", ");
    for (level, (index, dim)) in indices.iter().zip(dims).enumerate() {
        lines.push(format!(
            "{}for (int unsigned {index} = 0; {index} < {dim}; {index}++) begin",
            indent(level)
        ));
    }
    let body_indent = indent(indices.len());
    lines.push(format!(
        "{body_indent}{element_ref} = {class_name}::type_id::create($sformatf({create_format}, {create_args}));"
    ));
    lines.push(format!(
        "{body_indent}{element_ref}.configure(get_block(), this);"
    ));
    lines.push(format!("{body_indent}{element_ref}.build();"));
    for slice in hdl_slices {
        lines.push(format!(
            "{body_indent}{element_ref}.add_hdl_path_slice({}, {}, {}, {});",
            slice.path_expr, slice.offset, slice.size, slice.first
        ));
    }
    for level in (0..indices.len()).rev() {
        lines.push(format!("{}end", indent(level)));
    }
    lines
}

fn register_file_member_map_lines(
    var_name: &str,
    dims: &[u64],
    base_offset: u64,
    offset_groups: &[ArrayOffsetGroup],
    rights: &str,
) -> Vec<String> {
    if dims.is_empty() {
        return vec![format!(
            "      mp.add_reg({var_name}, offset + {}, {});",
            addr_literal(base_offset),
            sv_string(rights)
        )];
    }

    let indices = loop_indices(dims.len());
    let element_ref = array_element_ref(var_name, &indices);
    let offset_expr = array_offset_expr(dims, &indices, offset_groups);
    let mut lines = Vec::new();
    for (level, (index, dim)) in indices.iter().zip(dims).enumerate() {
        lines.push(format!(
            "{}for (int unsigned {index} = 0; {index} < {dim}; {index}++) begin",
            indent(level)
        ));
    }
    let body_indent = indent(indices.len());
    lines.push(format!(
        "{body_indent}mp.add_reg({element_ref}, offset + {} + {offset_expr}, {});",
        addr_literal(base_offset),
        sv_string(rights)
    ));
    for level in (0..indices.len()).rev() {
        lines.push(format!("{}end", indent(level)));
    }
    lines
}

fn register_instance(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    offset: u64,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
) -> Result<RegisterInstance> {
    let instance_name = if prefix_with_block {
        format!("{}_{}", block.name, register.name)
    } else {
        register.name.clone()
    };
    Ok(RegisterInstance {
        var_name: unique_ident(&instance_name, used_names),
        class_name: register_class_name(component, block, register, None),
        create_name: sv_string(&instance_name),
        configure_args: "this".into(),
        map_var_name: map_var_name.into(),
        offset_literal: addr_literal(offset),
        rights: sv_string(&register_rights(block, register)),
        hdl_slices: hdl_slices(
            register,
            register.hdl_path.as_ref().or(block.hdl_path.as_ref()),
        )?,
    })
}

fn alternate_register_instance(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: &AlternateRegister,
    offset: u64,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
) -> Result<RegisterInstance> {
    let instance_name = if prefix_with_block {
        format!("{}_{}_{}", block.name, register.name, alternate.name)
    } else {
        alternate.name.clone()
    };
    Ok(RegisterInstance {
        var_name: unique_ident(&instance_name, used_names),
        class_name: alternate_register_class_name(component, block, register, alternate),
        create_name: sv_string(&instance_name),
        configure_args: "this".into(),
        map_var_name: map_var_name.into(),
        offset_literal: addr_literal(offset),
        rights: sv_string(&register_rights_from_fields(
            block,
            alternate.access.as_deref(),
            &alternate.fields,
        )),
        hdl_slices: hdl_slices_from_fields(
            &alternate.fields,
            alternate
                .hdl_path
                .as_ref()
                .or(register.hdl_path.as_ref())
                .or(block.hdl_path.as_ref()),
        )?,
    })
}

fn register_array(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    base_offset: u64,
    layout: MapLayout,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
) -> Result<RegisterArray> {
    let create_name = if prefix_with_block {
        format!("{}_{}", block.name, register.name)
    } else {
        register.name.clone()
    };
    let var_name = unique_ident(&create_name, used_names);
    let class_name = register_class_name(component, block, register, None);
    let dims = parse_dims("register dim", &register.dims)?;
    let hdl_slices = hdl_slices(
        register,
        register.hdl_path.as_ref().or(block.hdl_path.as_ref()),
    )?;
    let offset_groups = vec![ArrayOffsetGroup {
        first_dimension: 0,
        dimension_count: dims.len(),
        stride: register_stride(block, register, layout)?,
    }];
    Ok(RegisterArray {
        declaration_suffix: array_declaration_suffix(&dims),
        build_code: array_build_code(ArrayBuildSpec {
            var_name: &var_name,
            class_name: &class_name,
            create_name: &create_name,
            dims: &dims,
            base_offset,
            offset_groups: &offset_groups,
            regfile_parent: None,
            map_var_name,
            rights: &register_rights(block, register),
            hdl_slices: &hdl_slices,
        }),
        var_name,
        class_name,
    })
}

fn alternate_register_array(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: &AlternateRegister,
    base_offset: u64,
    layout: MapLayout,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
) -> Result<RegisterArray> {
    let create_name = if prefix_with_block {
        format!("{}_{}_{}", block.name, register.name, alternate.name)
    } else {
        alternate.name.clone()
    };
    let var_name = unique_ident(&create_name, used_names);
    let class_name = alternate_register_class_name(component, block, register, alternate);
    let dims = parse_dims("register dim", &register.dims)?;
    let hdl_slices = hdl_slices_from_fields(
        &alternate.fields,
        alternate
            .hdl_path
            .as_ref()
            .or(register.hdl_path.as_ref())
            .or(block.hdl_path.as_ref()),
    )?;
    let rights = register_rights_from_fields(block, alternate.access.as_deref(), &alternate.fields);
    let offset_groups = vec![ArrayOffsetGroup {
        first_dimension: 0,
        dimension_count: dims.len(),
        stride: register_stride(block, register, layout)?,
    }];
    Ok(RegisterArray {
        declaration_suffix: array_declaration_suffix(&dims),
        build_code: array_build_code(ArrayBuildSpec {
            var_name: &var_name,
            class_name: &class_name,
            create_name: &create_name,
            dims: &dims,
            base_offset,
            offset_groups: &offset_groups,
            regfile_parent: None,
            map_var_name,
            rights: &rights,
            hdl_slices: &hdl_slices,
        }),
        var_name,
        class_name,
    })
}

struct ArrayBuildSpec<'a> {
    var_name: &'a str,
    class_name: &'a str,
    create_name: &'a str,
    dims: &'a [u64],
    base_offset: u64,
    offset_groups: &'a [ArrayOffsetGroup],
    regfile_parent: Option<ArrayParentSpec<'a>>,
    map_var_name: &'a str,
    rights: &'a str,
    hdl_slices: &'a [HdlSlice],
}

struct ArrayOffsetGroup {
    first_dimension: usize,
    dimension_count: usize,
    stride: u64,
}

#[derive(Clone, Copy)]
struct ArrayParentSpec<'a> {
    var_name: &'a str,
    dimension_count: usize,
}

fn array_declaration_suffix(dims: &[u64]) -> String {
    dims.iter()
        .map(|dim| format!("[{dim}]"))
        .collect::<Vec<_>>()
        .join("")
}

fn array_build_code(spec: ArrayBuildSpec<'_>) -> String {
    let indices = loop_indices(spec.dims.len());
    let element_ref = array_element_ref(spec.var_name, &indices);
    let offset_expr = array_offset_expr(spec.dims, &indices, spec.offset_groups);
    let create_format = create_name_format(spec.create_name, indices.len());
    let create_args = indices.join(", ");
    let mut lines = Vec::new();

    for (level, (index, dim)) in indices.iter().zip(spec.dims).enumerate() {
        lines.push(format!(
            "{}for (int unsigned {index} = 0; {index} < {dim}; {index}++) begin",
            indent(level)
        ));
    }

    let body_indent = indent(indices.len());
    lines.push(format!(
        "{body_indent}{element_ref} = {}::type_id::create($sformatf({create_format}, {create_args}));",
        spec.class_name
    ));
    lines.push(format!(
        "{body_indent}{element_ref}.configure({});",
        array_configure_args(spec.regfile_parent, &indices)
    ));
    lines.push(format!("{body_indent}{element_ref}.build();"));
    for slice in spec.hdl_slices {
        lines.push(format!(
            "{body_indent}{element_ref}.add_hdl_path_slice({}, {}, {}, {});",
            slice.path_expr, slice.offset, slice.size, slice.first
        ));
    }
    lines.push(format!(
        "{body_indent}{}.add_reg({element_ref}, {} + {offset_expr}, {});",
        spec.map_var_name,
        addr_literal(spec.base_offset),
        sv_string(spec.rights)
    ));

    for level in (0..indices.len()).rev() {
        lines.push(format!("{}end", indent(level)));
    }

    lines.join("\n")
}

fn loop_indices(count: usize) -> Vec<String> {
    if count == 1 {
        vec!["i".into()]
    } else {
        (0..count).map(|index| format!("i{index}")).collect()
    }
}

fn array_element_ref(var_name: &str, indices: &[String]) -> String {
    let suffix = indices
        .iter()
        .map(|index| format!("[{index}]"))
        .collect::<Vec<_>>()
        .join("");
    format!("{var_name}{suffix}")
}

fn create_name_format(create_name: &str, index_count: usize) -> String {
    let suffix = vec!["%0d"; index_count].join("_");
    sv_string(&format!("{create_name}_{suffix}"))
}

fn array_configure_args(regfile_parent: Option<ArrayParentSpec<'_>>, indices: &[String]) -> String {
    match regfile_parent {
        Some(parent) => {
            let parent_ref = array_parent_ref(parent, indices);
            format!("this, {parent_ref}")
        }
        None => "this".into(),
    }
}

fn array_parent_ref(parent: ArrayParentSpec<'_>, indices: &[String]) -> String {
    if parent.dimension_count == 0 {
        parent.var_name.into()
    } else {
        array_element_ref(parent.var_name, &indices[..parent.dimension_count])
    }
}

fn array_offset_expr(
    dims: &[u64],
    indices: &[String],
    offset_groups: &[ArrayOffsetGroup],
) -> String {
    offset_groups
        .iter()
        .map(|group| {
            let start = group.first_dimension;
            let end = start + group.dimension_count;
            let linear_index = linear_index_expr(&dims[start..end], &indices[start..end]);
            let index_expr = if group.dimension_count == 1 {
                linear_index
            } else {
                format!("({linear_index})")
            };
            format!("{index_expr} * {}", addr_literal(group.stride))
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn linear_index_expr(dims: &[u64], indices: &[String]) -> String {
    let mut expression = indices[0].clone();
    for (index, dim) in indices.iter().skip(1).zip(dims.iter().skip(1)) {
        expression = format!("{expression} * {dim} + {index}");
    }
    expression
}

fn indent(level: usize) -> String {
    "      ".to_string() + &"  ".repeat(level)
}

fn register_stride(block: &AddressBlock, register: &Register, layout: MapLayout) -> Result<u64> {
    register
        .stride
        .as_deref()
        .map(|stride| map_offset_units(block, "register array stride", stride, layout))
        .unwrap_or_else(|| {
            let register_bytes = parse_u64("register size", &register.size)?
                .div_ceil(8)
                .max(1);
            if layout.byte_addressing {
                Ok(register_bytes)
            } else {
                Ok(register_bytes.div_ceil(layout.n_bytes).max(1))
            }
        })
}

fn register_file_stride(
    block: &AddressBlock,
    register_file: &RegisterFile,
    layout: MapLayout,
) -> Result<u64> {
    register_file
        .stride
        .as_deref()
        .map(|stride| map_offset_units(block, "registerFile array stride", stride, layout))
        .unwrap_or_else(|| {
            map_offset_units(block, "registerFile range", &register_file.range, layout)
        })
}

fn hdl_slices(register: &Register, register_hdl_path: Option<&String>) -> Result<Vec<HdlSlice>> {
    hdl_slices_from_fields(&register.fields, register_hdl_path)
}

fn hdl_slices_from_fields(
    fields: &[Field],
    register_hdl_path: Option<&String>,
) -> Result<Vec<HdlSlice>> {
    let mut slices = fields
        .iter()
        .filter_map(|field| {
            field.hdl_path.as_ref().map(|field_hdl_path| {
                Ok((
                    hdl_path_expr(register_hdl_path.map(String::as_str), field_hdl_path),
                    parse_u64("field bitOffset", &field.bit_offset)? as i64,
                    parse_u64("field bitWidth", &field.bit_width)? as i64,
                ))
            })
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .enumerate()
        .map(|(index, (path_expr, offset, size))| HdlSlice {
            path_expr,
            offset,
            size,
            first: sv_bool_literal(index == 0),
        })
        .collect::<Vec<_>>();

    if slices.is_empty()
        && let Some(register_hdl_path) = register_hdl_path
    {
        slices.push(HdlSlice {
            path_expr: hdl_path_expr(None, register_hdl_path),
            offset: -1,
            size: -1,
            first: sv_bool_literal(true),
        });
    }

    Ok(slices)
}

fn alternate_metadata_params(alternate: &AlternateRegister) -> Vec<ConstraintParamView> {
    let groups_or_modes = mode_refs_string(&alternate.groups_or_modes);
    if groups_or_modes.is_empty() {
        return Vec::new();
    }

    vec![ConstraintParamView {
        name: "ALTERNATE_GROUPS_OR_MODES".into(),
        type_expr: "string".into(),
        value_expr: sv_string(&groups_or_modes),
    }]
}

fn description_param(name: &str, description: &str) -> Option<ConstraintParamView> {
    let description = description.trim();
    if description.is_empty() {
        return None;
    }

    Some(ConstraintParamView {
        name: name.into(),
        type_expr: "string".into(),
        value_expr: sv_string(description),
    })
}

fn access_policy_metadata_params(
    prefix: &str,
    policies: &[AccessPolicy],
) -> Vec<ConstraintParamView> {
    let mut params = Vec::new();
    for (index, policy) in policies.iter().enumerate() {
        if let Some(access) = &policy.access {
            params.push(ConstraintParamView {
                name: format!("{prefix}_{index}_ACCESS"),
                type_expr: "string".into(),
                value_expr: sv_string(access),
            });
        }
        let mode_refs = mode_refs_string(&policy.mode_refs);
        if !mode_refs.is_empty() {
            params.push(ConstraintParamView {
                name: format!("{prefix}_{index}_MODES"),
                type_expr: "string".into(),
                value_expr: sv_string(&mode_refs),
            });
        }
    }
    params
}

fn mode_refs_string(mode_refs: &[ModeRef]) -> String {
    mode_refs
        .iter()
        .map(|mode_ref| match &mode_ref.priority {
            Some(priority) => format!("{}:{}", mode_ref.name, priority),
            None => mode_ref.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn hdl_path_expr(parent: Option<&str>, child: &str) -> String {
    match parent {
        Some(parent) if parent.trim().starts_with('`') => {
            format!(
                "{{{}, {}}}",
                parent.trim(),
                sv_string(&format!(".{}", child.trim()))
            )
        }
        Some(parent) if !parent.trim().is_empty() => {
            sv_string(&format!("{}.{}", parent.trim(), child.trim()))
        }
        _ if child.trim().starts_with('`') => child.trim().into(),
        _ => sv_string(child.trim()),
    }
}

fn width_bytes(block: &AddressBlock) -> Result<u64> {
    let width = parse_u64("addressBlock width", &block.width)?;
    Ok(width.div_ceil(8).max(1))
}

fn memory_size_words(block: &AddressBlock) -> Result<u64> {
    let range = range_bytes(block, "addressBlock range", &block.range)?;
    Ok(range.div_ceil(width_bytes(block)?).max(1))
}

fn is_memory_block(block: &AddressBlock) -> bool {
    block
        .usage
        .as_deref()
        .is_some_and(|usage| usage.eq_ignore_ascii_case("memory"))
}

fn memory_rights(block: &AddressBlock) -> String {
    match block.access.as_deref() {
        Some("read-only") => "RO",
        _ => "RW",
    }
    .into()
}

fn register_rights(block: &AddressBlock, register: &Register) -> String {
    register_rights_from_fields(block, register.access.as_deref(), &register.fields)
}

fn register_rights_from_fields(
    block: &AddressBlock,
    register_access: Option<&str>,
    fields: &[Field],
) -> String {
    let mut has_read = false;
    let mut has_write = false;
    for field in fields {
        let access = uvm_access(effective_access(block, register_access, field), field);
        has_read |= access.contains('R');
        has_write |= access.contains('W');
    }
    match (has_read, has_write) {
        (true, false) => "RO",
        (false, true) => "WO",
        _ => "RW",
    }
    .into()
}

fn effective_access<'a>(
    block: &'a AddressBlock,
    register_access: Option<&'a str>,
    field: &'a Field,
) -> &'a str {
    field
        .access
        .as_deref()
        .or(register_access)
        .or(block.access.as_deref())
        .unwrap_or("read-write")
}

fn uvm_access(access: &str, field: &Field) -> String {
    match (
        access,
        field.modified_write_value.as_deref(),
        field.read_action.as_deref(),
    ) {
        (_, Some("oneToClear"), Some("clear")) => "W1SRC",
        (_, Some("oneToClear"), Some("set")) => "W1CRS",
        (_, Some("zeroToClear"), Some("clear")) => "W0SRC",
        (_, Some("zeroToClear"), Some("set")) => "W0CRS",
        (_, Some("oneToClear"), _) => "W1C",
        (_, Some("oneToSet"), _) => "W1S",
        (_, Some("oneToToggle"), _) => "W1T",
        (_, Some("zeroToClear"), _) => "W0C",
        (_, Some("zeroToSet"), _) => "W0S",
        (_, Some("zeroToToggle"), _) => "W0T",
        (_, Some("clear"), _) => "WC",
        (_, Some("set"), _) => "WS",
        ("read-write", _, Some("clear")) => "RC",
        ("read-write", _, Some("set")) => "RS",
        ("read-only", _, Some("clear")) => "RC",
        ("read-only", _, Some("set")) => "RS",
        ("read-only", _, _) => "RO",
        ("write-only", _, _) => "WO",
        ("writeOnce", _, _) => "WO1",
        ("read-writeOnce", _, _) => "WO1",
        _ => "RW",
    }
    .into()
}

fn is_writable_access(access: &str) -> bool {
    access.contains('W')
}

fn inherited_volatile(
    block: &AddressBlock,
    register_volatile: Option<&str>,
    field: &Field,
) -> bool {
    field
        .volatile
        .as_deref()
        .or(register_volatile)
        .or(block.volatile.as_deref())
        .is_some_and(is_truthy_sv_bool)
}

fn is_truthy_sv_bool(value: &str) -> bool {
    matches!(value.trim(), "true" | "1")
}

fn sv_bool_expr(value: &str) -> String {
    if is_truthy_sv_bool(value) {
        "1'b1"
    } else {
        "1'b0"
    }
    .into()
}

fn sv_bool_literal(value: bool) -> &'static str {
    if value { "1'b1" } else { "1'b0" }
}

fn bit_literal(field: &'static str, width: u64, value: &str) -> Result<String> {
    Ok(format!("{width}'h{:x}", parse_u64(field, value)?))
}

fn parse_u64(field: &'static str, value: &str) -> Result<u64> {
    let trimmed = value.trim();
    let parsed = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .map(|hex| u64::from_str_radix(hex, 16))
        .or_else(|| {
            trimmed
                .strip_prefix("0b")
                .or_else(|| trimmed.strip_prefix("0B"))
                .map(|binary| u64::from_str_radix(binary, 2))
        })
        .unwrap_or_else(|| trimmed.parse::<u64>());
    parsed.map_err(|_| Error::InvalidNumber {
        field,
        value: value.into(),
    })
}

fn addr_literal(value: u64) -> String {
    format!("64'h{value:x}")
}

fn sv_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn unique_const_ident(value: &str, used: &mut Vec<String>) -> String {
    unique_ident(value, used).to_ascii_uppercase()
}

fn unique_ident(value: &str, used: &mut Vec<String>) -> String {
    let base = ident(value);
    let mut candidate = base.clone();
    let mut index = 1;
    while used.iter().any(|name| name == &candidate) {
        candidate = format!("{base}_{index}");
        index += 1;
    }
    used.push(candidate.clone());
    candidate
}

fn ident(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    out = out.trim_matches('_').to_string();
    if out.is_empty() {
        out.push_str("unnamed");
    }
    if out.as_bytes()[0].is_ascii_digit() || is_sv_keyword(&out) {
        out.insert(0, '_');
    }
    out
}

fn is_sv_keyword(value: &str) -> bool {
    matches!(
        value,
        "class"
            | "endclass"
            | "function"
            | "endfunction"
            | "package"
            | "endpackage"
            | "rand"
            | "int"
            | "bit"
            | "begin"
            | "end"
            | "default"
            | "for"
            | "if"
            | "else"
            | "this"
            | "super"
            | "null"
    )
}
