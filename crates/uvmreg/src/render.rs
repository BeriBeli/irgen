use std::collections::BTreeMap;

use askama::Template;

use crate::model::{
    AddressBlock, AlternateRegister, Component, EnumeratedValue, Field, HdlPathSlice, Register,
    RegisterFile, SubspaceMap,
};
use crate::numeric::parse_u64_expr;
use crate::{Error, Result};

#[derive(Template)]
#[template(path = "package.sv", escape = "none")]
struct PackageTemplate<'a> {
    guard: &'a str,
    includes: &'a [String],
    register_classes: &'a [RegisterClass],
    memory_classes: &'a [MemoryClass],
    register_file_classes: &'a [RegisterFileClass],
    block_classes: &'a [BlockClass],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Default)]
struct ClassSet {
    register_classes: Vec<RegisterClass>,
    memory_classes: Vec<MemoryClass>,
    register_file_classes: Vec<RegisterFileClass>,
    block_classes: Vec<BlockClass>,
}

#[derive(Debug)]
struct RegisterClass {
    class_name: String,
    default_name: String,
    size_bits: u64,
    coverage_enabled: bool,
    coverage_model: &'static str,
    fields: Vec<FieldView>,
}

#[derive(Debug)]
struct FieldView {
    var_name: String,
    create_name: String,
    enum_type_name: String,
    enum_msb: u64,
    has_enum_values: bool,
    enum_values: Vec<EnumValueView>,
    width: u64,
    lsb: u64,
    msb: u64,
    access: String,
    volatile: &'static str,
    reset_literal: String,
    has_reset: &'static str,
    is_rand: &'static str,
    compare_check: &'static str,
    compare_needs_set: bool,
    extra_resets: Vec<ResetView>,
}

#[derive(Debug)]
struct ResetView {
    value_literal: String,
    kind: String,
}

#[derive(Debug)]
struct MemoryClass {
    class_name: String,
    default_name: String,
    size_words: u64,
    width_bits: u64,
    rights: String,
    coverage_model: &'static str,
}

#[derive(Debug)]
struct EnumValueView {
    name: String,
    literal: String,
}

#[derive(Debug)]
struct BlockClass {
    class_name: String,
    default_name: String,
    maps: Vec<MapInstance>,
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
    class_name: String,
    var_name: String,
    create_name: String,
    map_var_name: String,
    size_words: u64,
    width_bits: u64,
    offset_literal: String,
    rights: String,
    hdl_path_expr: String,
    coverage_enabled: bool,
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

#[derive(Debug)]
struct IndexedHdlSlices {
    indices: Vec<u64>,
    slices: Vec<HdlSlice>,
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
    serialize_uvm_reg_with_options(component, RenderOptions::default())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderOptions {
    pub coverage: bool,
}

pub fn serialize_uvm_reg_with_options(
    component: &Component,
    options: RenderOptions,
) -> Result<String> {
    let classes = class_set(component, options)?;
    validate_unique_class_names(&classes)?;
    render_class_set(&single_file_guard(component), &[], &classes)
}

pub fn serialize_uvm_reg_by_block_with_options(
    component: &Component,
    options: RenderOptions,
) -> Result<Vec<RenderedFile>> {
    validate_unique_class_names(&class_set(component, options)?)?;

    let mut files = Vec::new();
    let mut top_includes = Vec::new();

    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        let mut address_space_includes = Vec::new();
        for block in &scoped_component.blocks {
            let file = address_block_file(&scoped_component, block, true, options)?;
            address_space_includes.push(file.path.clone());
            files.push(file);
        }
        let file = address_space_file(&scoped_component, address_space_includes, options)?;
        top_includes.push(file.path.clone());
        files.push(file);
    }

    for block in &component.blocks {
        let file = address_block_file(component, block, false, options)?;
        top_includes.push(file.path.clone());
        files.push(file);
    }

    files.push(top_file(component, top_includes, options)?);
    Ok(files)
}

pub fn serialize_uvm_reg_by_block(component: &Component) -> Result<Vec<RenderedFile>> {
    serialize_uvm_reg_by_block_with_options(component, RenderOptions::default())
}

fn class_set(component: &Component, options: RenderOptions) -> Result<ClassSet> {
    Ok(ClassSet {
        register_classes: register_classes_with_options(component, options)?,
        memory_classes: memory_classes_with_options(component, options)?,
        register_file_classes: register_file_classes(component)?,
        block_classes: block_classes(component, options)?,
    })
}

fn render_class_set(guard: &str, includes: &[String], classes: &ClassSet) -> Result<String> {
    let rendered = PackageTemplate {
        guard,
        includes,
        register_classes: &classes.register_classes,
        memory_classes: &classes.memory_classes,
        register_file_classes: &classes.register_file_classes,
        block_classes: &classes.block_classes,
    }
    .render()?;
    Ok(normalize_spacing(rendered))
}

fn validate_unique_class_names(classes: &ClassSet) -> Result<()> {
    let mut used = Vec::new();
    for name in classes
        .register_classes
        .iter()
        .map(|class| &class.class_name)
        .chain(classes.memory_classes.iter().map(|class| &class.class_name))
        .chain(
            classes
                .register_file_classes
                .iter()
                .map(|class| &class.class_name),
        )
        .chain(classes.block_classes.iter().map(|class| &class.class_name))
    {
        if used.iter().any(|used_name| used_name == name) {
            return Err(Error::DuplicateGeneratedClassName { name: name.clone() });
        }
        used.push(name.clone());
    }
    Ok(())
}

fn normalize_spacing(mut sv: String) -> String {
    sv = sv.replace("\r\n", "\n").replace('\r', "\n");
    while sv.contains("\n\n\n") {
        sv = sv.replace("\n\n\n", "\n\n");
    }
    while sv.contains("`include \"uvm_macros.svh\"\n\n`include ") {
        sv = sv.replace(
            "`include \"uvm_macros.svh\"\n\n`include ",
            "`include \"uvm_macros.svh\"\n`include ",
        );
    }
    while sv.contains(".sv\"\n\n`include ") {
        sv = sv.replace(".sv\"\n\n`include ", ".sv\"\n`include ");
    }
    sv
}

fn address_block_file(
    component: &Component,
    block: &AddressBlock,
    include_component: bool,
    options: RenderOptions,
) -> Result<RenderedFile> {
    let class_name = address_block_class_name(component, block, include_component);
    let mut classes = ClassSet::default();
    register_classes_for_block(
        component,
        block,
        &mut classes.register_classes,
        include_component,
        options,
    )?;
    if options.coverage {
        memory_classes_for_block(
            component,
            block,
            &mut classes.memory_classes,
            include_component,
        )?;
    }
    register_file_classes_for_block(
        component,
        block,
        &mut classes.register_file_classes,
        include_component,
    )?;
    classes.block_classes.push(address_block_class(
        component,
        block,
        include_component,
        options,
    )?);
    render_file(class_file_name(&class_name), Vec::new(), classes)
}

fn address_space_file(
    component: &Component,
    includes: Vec<String>,
    options: RenderOptions,
) -> Result<RenderedFile> {
    let class_name = format!("ral_sys_{}", ident(&component.name));
    let mut classes = ClassSet::default();
    classes
        .block_classes
        .push(block_class(component, Vec::new(), true, options)?);
    render_file(class_file_name(&class_name), includes, classes)
}

fn top_file(
    component: &Component,
    includes: Vec<String>,
    options: RenderOptions,
) -> Result<RenderedFile> {
    let mut classes = ClassSet::default();
    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            register_classes_for_block(
                component,
                block,
                &mut classes.register_classes,
                false,
                options,
            )?;
            if options.coverage {
                memory_classes_for_block(component, block, &mut classes.memory_classes, false)?;
            }
            register_file_classes_for_block(
                component,
                block,
                &mut classes.register_file_classes,
                false,
            )?;
        }
    }
    classes.block_classes.push(block_class(
        component,
        submap_instances(component)?,
        false,
        options,
    )?);
    render_file(top_file_name(component), includes, classes)
}

fn render_file(path: String, includes: Vec<String>, classes: ClassSet) -> Result<RenderedFile> {
    let guard = file_guard(&path);
    let content = render_class_set(&guard, &includes, &classes)?;
    Ok(RenderedFile { path, content })
}

fn single_file_guard(component: &Component) -> String {
    format!("RAL_{}_SV", ident(&component.name).to_ascii_uppercase())
}

fn file_guard(path: &str) -> String {
    ident(path.trim_end_matches(".sv")).to_ascii_uppercase() + "_SV"
}

fn top_file_name(component: &Component) -> String {
    format!("ral_{}.sv", ident(&component.name))
}

fn class_file_name(class_name: &str) -> String {
    format!("{class_name}.sv")
}

fn register_classes_with_options(
    component: &Component,
    options: RenderOptions,
) -> Result<Vec<RegisterClass>> {
    let mut classes = Vec::new();
    for block in &component.blocks {
        register_classes_for_block(component, block, &mut classes, false, options)?;
    }
    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            register_classes_for_block(component, block, &mut classes, false, options)?;
        }
    }
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &address_space.blocks {
            register_classes_for_block(&scoped_component, block, &mut classes, true, options)?;
        }
    }
    Ok(classes)
}

fn memory_classes_with_options(
    component: &Component,
    options: RenderOptions,
) -> Result<Vec<MemoryClass>> {
    let mut classes = Vec::new();
    if !options.coverage {
        return Ok(classes);
    }
    for block in &component.blocks {
        memory_classes_for_block(component, block, &mut classes, false)?;
    }
    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            memory_classes_for_block(component, block, &mut classes, false)?;
        }
    }
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &address_space.blocks {
            memory_classes_for_block(&scoped_component, block, &mut classes, true)?;
        }
    }
    Ok(classes)
}

fn memory_classes_for_block(
    component: &Component,
    block: &AddressBlock,
    classes: &mut Vec<MemoryClass>,
    include_component: bool,
) -> Result<()> {
    if is_memory_block(block) {
        let width_bits = parse_u64("addressBlock width", &block.width)?;
        classes.push(MemoryClass {
            class_name: memory_class_name(component, block, include_component),
            default_name: block.name.clone(),
            size_words: memory_size_words(block)?,
            width_bits,
            rights: sv_string(&memory_rights(block)),
            coverage_model: "build_coverage(UVM_CVR_ADDR_MAP)",
        });
    }
    Ok(())
}

fn register_file_classes(component: &Component) -> Result<Vec<RegisterFileClass>> {
    let mut classes = Vec::new();
    for block in &component.blocks {
        register_file_classes_for_block(component, block, &mut classes, false)?;
    }
    for remap in &component.memory_remaps {
        for block in &remap.blocks {
            register_file_classes_for_block(component, block, &mut classes, false)?;
        }
    }
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &address_space.blocks {
            register_file_classes_for_block(&scoped_component, block, &mut classes, true)?;
        }
    }
    Ok(classes)
}

fn register_file_classes_for_block(
    component: &Component,
    block: &AddressBlock,
    classes: &mut Vec<RegisterFileClass>,
    include_component: bool,
) -> Result<()> {
    for register_file in &block.register_files {
        classes.push(register_file_class(
            component,
            block,
            register_file,
            include_component,
        )?);
    }
    Ok(())
}

fn register_file_class(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    include_component: bool,
) -> Result<RegisterFileClass> {
    let class_name = register_file_class_name(component, block, register_file, include_component);
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
        let class_name = register_file_register_class_name(
            component,
            block,
            register_file,
            register,
            include_component,
        );
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
                include_component,
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
    include_component: bool,
    options: RenderOptions,
) -> Result<()> {
    for register in &block.registers {
        let path_parts =
            class_path_parts(component, include_component, &[&block.name, &register.name]);
        classes.push(register_class(
            block,
            &path_parts,
            &register.name,
            &register.size,
            register.volatile.as_deref(),
            register.access.as_deref(),
            &register.fields,
            options,
        )?);
        for alternate in &register.alternate_registers {
            let path_parts = class_path_parts(
                component,
                include_component,
                &[&block.name, &register.name, &alternate.name],
            );
            classes.push(register_class(
                block,
                &path_parts,
                &alternate.name,
                &register.size,
                alternate.volatile.as_deref(),
                alternate.access.as_deref(),
                &alternate.fields,
                options,
            )?);
        }
    }
    for register_file in &block.register_files {
        for register in &register_file.registers {
            let path_parts = class_path_parts(
                component,
                include_component,
                &[&block.name, &register_file.name, &register.name],
            );
            classes.push(register_class(
                block,
                &path_parts,
                &register.name,
                &register.size,
                register.volatile.as_deref(),
                register.access.as_deref(),
                &register.fields,
                options,
            )?);
            for alternate in &register.alternate_registers {
                let path_parts = class_path_parts(
                    component,
                    include_component,
                    &[
                        &block.name,
                        &register_file.name,
                        &register.name,
                        &alternate.name,
                    ],
                );
                classes.push(register_class(
                    block,
                    &path_parts,
                    &alternate.name,
                    &register.size,
                    alternate.volatile.as_deref(),
                    alternate.access.as_deref(),
                    &alternate.fields,
                    options,
                )?);
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn register_class(
    block: &AddressBlock,
    path_parts: &[String],
    default_name: &str,
    size: &str,
    register_volatile: Option<&str>,
    register_access: Option<&str>,
    fields: &[Field],
    options: RenderOptions,
) -> Result<RegisterClass> {
    let size_bits = parse_u64("register size", size)?;
    let class_name = format!(
        "ral_reg_{}",
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

    Ok(RegisterClass {
        class_name,
        default_name: default_name.into(),
        size_bits,
        coverage_enabled: options.coverage,
        coverage_model: if options.coverage {
            "build_coverage(UVM_CVR_REG_BITS)"
        } else {
            "UVM_NO_COVERAGE"
        },
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
    Ok(FieldView {
        enum_type_name: format!("{var_name}_e"),
        enum_msb: width.saturating_sub(1),
        has_enum_values: !enum_values.is_empty(),
        enum_values,
        var_name,
        create_name: sv_string(&field.name),
        width,
        lsb: parse_u64("field bitOffset", &field.bit_offset)?,
        msb: parse_u64("field bitOffset", &field.bit_offset)? + width.saturating_sub(1),
        access: sv_string(&access),
        volatile: sv_bool_literal(inherited_volatile(block, register_volatile, field)),
        reset_literal: format!("{width}'h{reset_value:x}"),
        has_reset: sv_bool_literal(default_reset_index.is_some()),
        is_rand: sv_bool_literal(is_writable_access(&access)),
        compare_check: field_compare_check(field),
        compare_needs_set: field_compare_check(field) != "UVM_CHECK",
        extra_resets,
    })
}

fn field_compare_check(field: &Field) -> &'static str {
    let testable = field
        .testable
        .as_deref()
        .map(|value| value.trim().eq_ignore_ascii_case("false"))
        .unwrap_or(false);
    let reserved = field
        .reserved
        .as_deref()
        .map(|value| {
            let trimmed = value.trim();
            trimmed.eq_ignore_ascii_case("true") || trimmed == "1"
        })
        .unwrap_or(false);
    if testable || reserved {
        "UVM_NO_CHECK"
    } else {
        "UVM_CHECK"
    }
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
            Ok(EnumValueView {
                name: unique_const_ident(&format!("{field_name}_{}", value.name), used_names),
                literal: format!("{width}'h{parsed:x}"),
            })
        })
        .collect()
}

fn block_classes(component: &Component, options: RenderOptions) -> Result<Vec<BlockClass>> {
    let mut classes = Vec::new();
    for address_space in &component.address_spaces {
        let scoped_component =
            scoped_component(component, &address_space.name, &address_space.blocks);
        for block in &scoped_component.blocks {
            classes.push(address_block_class(
                &scoped_component,
                block,
                true,
                options,
            )?);
        }
        classes.push(block_class(&scoped_component, Vec::new(), true, options)?);
    }
    for block in &component.blocks {
        classes.push(address_block_class(component, block, false, options)?);
    }
    classes.push(block_class(
        component,
        submap_instances(component)?,
        false,
        options,
    )?);
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

fn class_path_parts(component: &Component, include_component: bool, parts: &[&str]) -> Vec<String> {
    let mut path = Vec::new();
    if include_component {
        path.push(component.name.clone());
    }
    path.extend(parts.iter().map(|part| (*part).to_string()));
    path
}

fn block_class(
    component: &Component,
    submaps: Vec<SubmapInstance>,
    include_component: bool,
    options: RenderOptions,
) -> Result<BlockClass> {
    let class_name = format!("ral_sys_{}", ident(&component.name));
    let mut memories = Vec::new();
    let mut reg_files = Vec::new();
    let mut instances = Vec::new();
    let mut arrays = Vec::new();
    let mut used_names = Vec::new();
    let maps = map_instances(component)?;
    let layouts = map_layouts(component, &maps);
    let child_blocks = child_block_instances(component, &layouts, include_component)?;

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
                include_component,
                options,
            )?;
        }
    }

    Ok(BlockClass {
        class_name,
        default_name: component.name.clone(),
        maps,
        memories,
        reg_files,
        instances,
        arrays,
        child_blocks,
        submaps,
    })
}

fn address_block_class(
    component: &Component,
    block: &AddressBlock,
    include_component: bool,
    options: RenderOptions,
) -> Result<BlockClass> {
    let class_name = address_block_class_name(component, block, include_component);
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
        include_component,
        options,
    )?;

    Ok(BlockClass {
        class_name,
        default_name: block.name.clone(),
        maps,
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
    include_component: bool,
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
            class_name: address_block_class_name(component, block, include_component),
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
        let segment_offset = subspace_segment_offset(component, subspace, map.layout)?;
        let offset = offset
            .checked_sub(segment_offset)
            .ok_or_else(|| Error::InvalidNumber {
                field: "subspaceMap segmentRef addressOffset",
                value: subspace
                    .segment_ref
                    .as_deref()
                    .unwrap_or_default()
                    .to_string(),
            })?;
        let var_name = unique_ident(&subspace.name, &mut used_names);
        submaps.push(SubmapInstance {
            class_name: format!(
                "ral_sys_{}",
                ident(&format!("{}_{}", component.name, address_space_ref))
            ),
            create_name: sv_string(&subspace.name),
            map_var_name: map.var_name.clone(),
            offset_literal: addr_literal(offset),
            var_name,
        });
    }

    Ok(submaps)
}

fn subspace_segment_offset(
    component: &Component,
    subspace: &SubspaceMap,
    parent_layout: MapLayout,
) -> Result<u64> {
    let Some(segment_ref) = subspace.segment_ref.as_deref() else {
        return Ok(0);
    };
    let Some(address_space_ref) = subspace.address_space_ref.as_deref() else {
        return Ok(0);
    };
    let Some(address_space) = component
        .address_spaces
        .iter()
        .find(|address_space| address_space.name == address_space_ref)
    else {
        return Ok(0);
    };
    let Some(segment) = address_space
        .segments
        .iter()
        .find(|segment| segment.name == segment_ref)
    else {
        return Ok(0);
    };
    validate_segment_ref_range(subspace, address_space, segment, parent_layout)?;

    map_offset_units_for_address_unit_bits(
        "addressSpace segment addressOffset",
        &segment.address_offset,
        parent_layout,
        parse_u64(
            "addressSpace addressUnitBits",
            &address_space.address_unit_bits,
        )?,
    )
}

fn validate_segment_ref_range(
    subspace: &SubspaceMap,
    address_space: &crate::model::AddressSpace,
    segment: &crate::model::Segment,
    parent_layout: MapLayout,
) -> Result<()> {
    let address_unit_bits = parse_u64(
        "addressSpace addressUnitBits",
        &address_space.address_unit_bits,
    )?;
    let segment_base = map_offset_units_for_address_unit_bits(
        "addressSpace segment addressOffset",
        &segment.address_offset,
        parent_layout,
        address_unit_bits,
    )?;
    let segment_range = map_offset_units_for_address_unit_bits(
        "addressSpace segment range",
        &segment.range,
        parent_layout,
        address_unit_bits,
    )?;
    let segment_limit = segment_base.saturating_add(segment_range);

    for block in &address_space.blocks {
        let block_base = map_offset_units_for_address_unit_bits(
            "addressBlock baseAddress",
            &block.base_address,
            parent_layout,
            address_unit_bits,
        )?;
        let block_range = map_offset_units_for_address_unit_bits(
            "addressBlock range",
            &block.range,
            parent_layout,
            address_unit_bits,
        )?;
        let block_limit = block_base.saturating_add(block_range);
        if block_base < segment_base || block_limit > segment_limit {
            return Err(Error::SegmentRefRangeViolation {
                subspace: subspace.name.clone(),
                segment: segment.name.clone(),
                block: block.name.clone(),
            });
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
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
    include_component: bool,
    options: RenderOptions,
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
            component,
            block,
            block_base,
            map_var_name,
            used_names,
            include_component,
            options,
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
                include_component,
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
                    include_component,
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
                include_component,
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
                    include_component,
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
            include_component,
        )?);
    }
    Ok(())
}

fn register_class_name(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: Option<&AlternateRegister>,
    include_component: bool,
) -> String {
    match alternate {
        Some(alternate) => {
            alternate_register_class_name(component, block, register, alternate, include_component)
        }
        None => ral_class_name(
            "ral_reg",
            class_path_parts(component, include_component, &[&block.name, &register.name]),
        ),
    }
}

fn alternate_register_class_name(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: &AlternateRegister,
    include_component: bool,
) -> String {
    ral_class_name(
        "ral_reg",
        class_path_parts(
            component,
            include_component,
            &[&block.name, &register.name, &alternate.name],
        ),
    )
}

fn register_file_register_class_name(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    register: &Register,
    include_component: bool,
) -> String {
    ral_class_name(
        "ral_reg",
        class_path_parts(
            component,
            include_component,
            &[&block.name, &register_file.name, &register.name],
        ),
    )
}

fn register_file_class_name(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    include_component: bool,
) -> String {
    ral_class_name(
        "ral_regfile",
        class_path_parts(
            component,
            include_component,
            &[&block.name, &register_file.name],
        ),
    )
}

fn register_file_alternate_class_name(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    register: &Register,
    alternate: &AlternateRegister,
    include_component: bool,
) -> String {
    ral_class_name(
        "ral_reg",
        class_path_parts(
            component,
            include_component,
            &[
                &block.name,
                &register_file.name,
                &register.name,
                &alternate.name,
            ],
        ),
    )
}

fn address_block_class_name(
    component: &Component,
    block: &AddressBlock,
    include_component: bool,
) -> String {
    ral_class_name(
        "ral_block",
        class_path_parts(component, include_component, &[&block.name]),
    )
}

fn memory_class_name(
    component: &Component,
    block: &AddressBlock,
    include_component: bool,
) -> String {
    ral_class_name(
        "ral_mem",
        class_path_parts(component, include_component, &[&block.name]),
    )
}

fn ral_class_name(prefix: &str, path_parts: Vec<String>) -> String {
    format!(
        "{prefix}_{}",
        path_parts
            .iter()
            .map(|part| ident(part))
            .collect::<Vec<_>>()
            .join("_")
    )
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
    if bits.is_multiple_of(8) {
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
    component: &Component,
    block: &AddressBlock,
    offset: u64,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    include_component: bool,
    options: RenderOptions,
) -> Result<MemoryInstance> {
    let width_bits = parse_u64("addressBlock width", &block.width)?;
    Ok(MemoryInstance {
        class_name: if options.coverage {
            memory_class_name(component, block, include_component)
        } else {
            "uvm_mem".into()
        },
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
        coverage_enabled: options.coverage,
    })
}

#[allow(clippy::too_many_arguments)]
fn register_file_instance_view(
    component: &Component,
    block: &AddressBlock,
    register_file: &RegisterFile,
    block_base: u64,
    layout: MapLayout,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
    include_component: bool,
) -> Result<RegisterFileInstance> {
    let create_name = if prefix_with_block {
        format!("{}_{}", block.name, register_file.name)
    } else {
        register_file.name.clone()
    };
    let var_name = unique_ident(&create_name, used_names);
    let class_name = register_file_class_name(component, block, register_file, include_component);
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

#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::too_many_arguments)]
fn register_instance(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    offset: u64,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
    include_component: bool,
) -> Result<RegisterInstance> {
    let instance_name = if prefix_with_block {
        format!("{}_{}", block.name, register.name)
    } else {
        register.name.clone()
    };
    Ok(RegisterInstance {
        var_name: unique_ident(&instance_name, used_names),
        class_name: register_class_name(component, block, register, None, include_component),
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

#[allow(clippy::too_many_arguments)]
fn alternate_register_instance(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    alternate: &AlternateRegister,
    offset: u64,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
    include_component: bool,
) -> Result<RegisterInstance> {
    let instance_name = if prefix_with_block {
        format!("{}_{}_{}", block.name, register.name, alternate.name)
    } else {
        alternate.name.clone()
    };
    Ok(RegisterInstance {
        var_name: unique_ident(&instance_name, used_names),
        class_name: alternate_register_class_name(
            component,
            block,
            register,
            alternate,
            include_component,
        ),
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

#[allow(clippy::too_many_arguments)]
fn register_array(
    component: &Component,
    block: &AddressBlock,
    register: &Register,
    base_offset: u64,
    layout: MapLayout,
    map_var_name: &str,
    used_names: &mut Vec<String>,
    prefix_with_block: bool,
    include_component: bool,
) -> Result<RegisterArray> {
    let create_name = if prefix_with_block {
        format!("{}_{}", block.name, register.name)
    } else {
        register.name.clone()
    };
    let var_name = unique_ident(&create_name, used_names);
    let class_name = register_class_name(component, block, register, None, include_component);
    let dims = parse_dims("register dim", &register.dims)?;
    let hdl_slices = hdl_slices(
        register,
        register.hdl_path.as_ref().or(block.hdl_path.as_ref()),
    )?;
    let indexed_hdl_slices = indexed_hdl_slices(register, dims.len())?;
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
            indexed_hdl_slices: &indexed_hdl_slices,
        }),
        var_name,
        class_name,
    })
}

#[allow(clippy::too_many_arguments)]
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
    include_component: bool,
) -> Result<RegisterArray> {
    let create_name = if prefix_with_block {
        format!("{}_{}_{}", block.name, register.name, alternate.name)
    } else {
        alternate.name.clone()
    };
    let var_name = unique_ident(&create_name, used_names);
    let class_name =
        alternate_register_class_name(component, block, register, alternate, include_component);
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
            indexed_hdl_slices: &[],
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
    indexed_hdl_slices: &'a [IndexedHdlSlices],
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
    for indexed in spec.indexed_hdl_slices {
        lines.push(format!(
            "{body_indent}if ({}) begin",
            index_condition(&indices, &indexed.indices)
        ));
        for slice in &indexed.slices {
            lines.push(format!(
                "{}{element_ref}.add_hdl_path_slice({}, {}, {}, {});",
                indent(indices.len() + 1),
                slice.path_expr,
                slice.offset,
                slice.size,
                slice.first
            ));
        }
        lines.push(format!("{body_indent}end"));
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

fn index_condition(index_names: &[String], index_values: &[u64]) -> String {
    index_names
        .iter()
        .zip(index_values)
        .map(|(name, value)| format!("{name} == {value}"))
        .collect::<Vec<_>>()
        .join(" && ")
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

fn indexed_hdl_slices(register: &Register, dim_count: usize) -> Result<Vec<IndexedHdlSlices>> {
    validate_unique_indexed_hdl_paths(&register.name, dim_count, &register.indexed_hdl_paths)?;
    for field in &register.fields {
        validate_unique_indexed_hdl_paths(
            &format!("{}.{}", register.name, field.name),
            dim_count,
            &field.indexed_hdl_paths,
        )?;
    }

    let mut indexed_slices = register
        .indexed_hdl_paths
        .iter()
        .map(|indexed| {
            Ok(IndexedHdlSlices {
                indices: indexed_access_handle_indices(register, dim_count, indexed)?,
                slices: hdl_slices_from_fields(&register.fields, Some(&indexed.path))?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    for field in &register.fields {
        for indexed in &field.indexed_hdl_paths {
            let indices = indexed_access_handle_indices(register, dim_count, indexed)?;
            let slices = if indexed.slices.is_empty() {
                vec![HdlPathSlice {
                    path: indexed.path.clone(),
                    left: None,
                    right: None,
                }]
            } else {
                indexed.slices.clone()
            };
            for slice in field_hdl_slices(field, None, &slices)? {
                append_indexed_hdl_slice(&mut indexed_slices, indices.clone(), slice);
            }
        }
    }

    for indexed in &mut indexed_slices {
        for (index, slice) in indexed.slices.iter_mut().enumerate() {
            slice.first = sv_bool_literal(index == 0);
        }
    }

    Ok(indexed_slices)
}

fn validate_unique_indexed_hdl_paths(
    owner: &str,
    dim_count: usize,
    indexed_paths: &[crate::model::IndexedHdlPath],
) -> Result<()> {
    let mut used = Vec::new();
    for indexed in indexed_paths {
        let indices = parse_dims("accessHandle index", &indexed.indices)?;
        if indices.len() != dim_count {
            return Err(Error::AccessHandleIndexDimensionMismatch {
                register: owner.into(),
                expected: dim_count,
                actual: indices.len(),
            });
        }
        if used.iter().any(|used_indices| used_indices == &indices) {
            return Err(Error::DuplicateAccessHandleIndices {
                owner: owner.into(),
                indices: indices
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            });
        }
        used.push(indices);
    }
    Ok(())
}

fn indexed_access_handle_indices(
    register: &Register,
    dim_count: usize,
    indexed: &crate::model::IndexedHdlPath,
) -> Result<Vec<u64>> {
    let indices = parse_dims("accessHandle index", &indexed.indices)?;
    if indices.len() != dim_count {
        return Err(Error::AccessHandleIndexDimensionMismatch {
            register: register.name.clone(),
            expected: dim_count,
            actual: indices.len(),
        });
    }
    Ok(indices)
}

fn append_indexed_hdl_slice(
    indexed_slices: &mut Vec<IndexedHdlSlices>,
    indices: Vec<u64>,
    slice: HdlSlice,
) {
    if let Some(indexed) = indexed_slices
        .iter_mut()
        .find(|indexed| indexed.indices == indices)
    {
        indexed.slices.push(slice);
    } else {
        indexed_slices.push(IndexedHdlSlices {
            indices,
            slices: vec![slice],
        });
    }
}

fn hdl_slices_from_fields(
    fields: &[Field],
    register_hdl_path: Option<&String>,
) -> Result<Vec<HdlSlice>> {
    let mut slices = Vec::new();
    for field in fields {
        if !field.hdl_path_slices.is_empty() {
            slices.extend(field_hdl_slices(
                field,
                register_hdl_path.map(String::as_str),
                &field.hdl_path_slices,
            )?);
        } else if let Some(field_hdl_path) = &field.hdl_path {
            slices.push(HdlSlice {
                path_expr: hdl_path_expr(register_hdl_path.map(String::as_str), field_hdl_path),
                offset: parse_u64("field bitOffset", &field.bit_offset)? as i64,
                size: parse_u64("field bitWidth", &field.bit_width)? as i64,
                first: sv_bool_literal(true),
            });
        }
    }

    for (index, slice) in slices.iter_mut().enumerate() {
        slice.first = sv_bool_literal(index == 0);
    }

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

fn field_hdl_slices(
    field: &Field,
    register_hdl_path: Option<&str>,
    path_slices: &[HdlPathSlice],
) -> Result<Vec<HdlSlice>> {
    if path_slices.is_empty() {
        return Ok(Vec::new());
    }

    let field_offset = parse_u64("field bitOffset", &field.bit_offset)?;
    let field_width = parse_u64("field bitWidth", &field.bit_width)?;
    let widths = path_slices
        .iter()
        .map(|slice| hdl_path_slice_width(field, slice, path_slices.len()))
        .collect::<Result<Vec<_>>>()?;
    let total_width = widths.iter().sum::<u64>();
    if total_width != field_width {
        return Err(Error::AccessHandleSliceWidthMismatch {
            field: field.name.clone(),
            expected: field_width,
            actual: total_width,
        });
    }

    let mut remaining_width = field_width;
    path_slices
        .iter()
        .zip(widths)
        .map(|(slice, width)| {
            remaining_width -= width;
            Ok(HdlSlice {
                path_expr: hdl_path_expr(
                    register_hdl_path,
                    &hdl_path_slice_path("accessHandle slice range", slice)?,
                ),
                offset: (field_offset + remaining_width) as i64,
                size: width as i64,
                first: sv_bool_literal(true),
            })
        })
        .collect()
}

fn hdl_path_slice_width(field: &Field, slice: &HdlPathSlice, slice_count: usize) -> Result<u64> {
    match (slice.left.as_deref(), slice.right.as_deref()) {
        (Some(left), Some(right)) => {
            let left = parse_u64("accessHandle slice left", left)?;
            let right = parse_u64("accessHandle slice right", right)?;
            Ok(left.abs_diff(right) + 1)
        }
        (None, None) if slice_count == 1 => parse_u64("field bitWidth", &field.bit_width),
        _ => Err(Error::AccessHandleSliceRangeMissing {
            field: field.name.clone(),
        }),
    }
}

fn hdl_path_slice_path(field: &'static str, slice: &HdlPathSlice) -> Result<String> {
    match (slice.left.as_deref(), slice.right.as_deref()) {
        (Some(left), Some(right)) => Ok(format!(
            "{}[{}:{}]",
            slice.path.trim(),
            parse_u64(field, left)?,
            parse_u64(field, right)?
        )),
        _ => Ok(slice.path.clone()),
    }
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

fn sv_bool_literal(value: bool) -> &'static str {
    if value { "1'b1" } else { "1'b0" }
}

fn bit_literal(field: &'static str, width: u64, value: &str) -> Result<String> {
    Ok(format!("{width}'h{:x}", parse_u64(field, value)?))
}

fn parse_u64(field: &'static str, value: &str) -> Result<u64> {
    parse_u64_expr(field, value)
}

fn addr_literal(value: u64) -> String {
    format!("`UVM_REG_ADDR_WIDTH'h{value:x}")
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
