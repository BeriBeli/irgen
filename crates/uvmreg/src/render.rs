use std::collections::BTreeMap;

use askama::Template;

use crate::numeric::parse_u64_expr;
use crate::{Error, Result};
use irgen_ipxact_model::{
    AddressBlock, AddressSpace, AlternateRegister, Component, EnumeratedValue, Field, HdlPathSlice,
    Register, RegisterFile, Reset, Segment, SubspaceMap,
};

#[derive(Template)]
#[template(path = "package.sv", escape = "none")]
struct PackageTemplate<'a> {
    guard: &'a str,
    package_name: &'a str,
    is_package: bool,
    include_uvm: bool,
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
pub enum FileType {
    #[default]
    Package,
    Header,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderOptions {
    pub coverage: bool,
    pub file_type: FileType,
}

pub fn serialize_uvm_reg_with_options(
    component: &Component,
    options: RenderOptions,
) -> Result<String> {
    let classes = class_set(component, options)?;
    validate_unique_class_names(&classes)?;
    render_root_class_set(
        component,
        &single_file_guard(component, options.file_type),
        &[],
        &classes,
        options,
    )
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
    if options.file_type == FileType::Package {
        let includes = files.iter().map(|file| file.path.clone()).collect();
        files.push(package_file(component, includes)?);
    }
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

fn render_root_class_set(
    component: &Component,
    guard: &str,
    includes: &[String],
    classes: &ClassSet,
    options: RenderOptions,
) -> Result<String> {
    render_class_set(
        guard,
        &package_name(component),
        options.file_type == FileType::Package,
        true,
        includes,
        classes,
    )
}

fn render_include_class_set(
    guard: &str,
    includes: &[String],
    classes: &ClassSet,
) -> Result<String> {
    render_class_set(guard, "", false, false, includes, classes)
}

fn render_class_set(
    guard: &str,
    package_name: &str,
    is_package: bool,
    include_uvm: bool,
    includes: &[String],
    classes: &ClassSet,
) -> Result<String> {
    let rendered = PackageTemplate {
        guard,
        package_name,
        is_package,
        include_uvm,
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
    if !sv.ends_with('\n') {
        sv.push('\n');
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
    render_include_file(class_file_name(&class_name), Vec::new(), classes)
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
        .push(block_class(component, true, false, options)?);
    let includes = if options.file_type == FileType::Package {
        Vec::new()
    } else {
        includes
    };
    render_include_file(class_file_name(&class_name), includes, classes)
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
    classes
        .block_classes
        .push(block_class(component, false, true, options)?);
    if options.file_type == FileType::Package {
        render_include_file(top_file_name(component), Vec::new(), classes)
    } else {
        render_root_file(
            component,
            top_file_name(component),
            includes,
            classes,
            options,
        )
    }
}

fn package_file(component: &Component, includes: Vec<String>) -> Result<RenderedFile> {
    render_root_file(
        component,
        package_file_name(component),
        includes,
        ClassSet::default(),
        RenderOptions {
            file_type: FileType::Package,
            ..RenderOptions::default()
        },
    )
}

fn render_root_file(
    component: &Component,
    path: String,
    includes: Vec<String>,
    classes: ClassSet,
    options: RenderOptions,
) -> Result<RenderedFile> {
    let guard = file_guard(&path);
    let content = render_root_class_set(component, &guard, &includes, &classes, options)?;
    Ok(RenderedFile { path, content })
}

fn render_include_file(
    path: String,
    includes: Vec<String>,
    classes: ClassSet,
) -> Result<RenderedFile> {
    let guard = file_guard(&path);
    let content = render_include_class_set(&guard, &includes, &classes)?;
    Ok(RenderedFile { path, content })
}

fn single_file_guard(component: &Component, file_type: FileType) -> String {
    match file_type {
        FileType::Package => file_guard(&package_file_name(component)),
        FileType::Header => file_guard(&top_file_name(component)),
    }
}

fn package_name(component: &Component) -> String {
    format!("ral_{}_pkg", ident(&component.name))
}

fn file_guard(path: &str) -> String {
    ident(path.trim_end_matches(".sv")).to_ascii_uppercase() + "_SV"
}

fn top_file_name(component: &Component) -> String {
    format!("ral_{}.sv", ident(&component.name))
}

fn package_file_name(component: &Component) -> String {
    format!("ral_{}_pkg.sv", ident(&component.name))
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
        let width_bits = parse_address_block_width_bits(&block.width)?;
        classes.push(MemoryClass {
            class_name: memory_class_name(component, block, include_component),
            default_name: block.name.clone(),
            size_words: memory_size_words(block)?,
            width_bits,
            rights: sv_string(&memory_rights(block)?),
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
    let mut used_names = reserved_register_file_member_names();
    let layout = map_layout_for_block(block)?;
    validate_register_file_member_layout(block, register_file, layout)?;

    for register in &register_file.registers {
        let register_dims = if is_array_dim(&register.dim)? {
            parse_array_dims("register dim", &register.dims)?
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
    let size_bits = parse_register_size_bits(size)?;
    validate_field_layout(default_name, size_bits, fields)?;
    let class_name = format!(
        "ral_reg_{}",
        path_parts
            .iter()
            .map(|part| ident(part))
            .collect::<Vec<_>>()
            .join("_")
    );
    let mut used_field_names = reserved_register_member_names(options.coverage);
    let mut used_enum_value_names = reserved_register_member_names(options.coverage);
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

fn validate_field_layout(register: &str, size_bits: u64, fields: &[Field]) -> Result<()> {
    let mut ranges = Vec::<(&str, u64, u64)>::new();
    for field in fields {
        let lsb = parse_u64("field bitOffset", &field.bit_offset)?;
        let width = parse_field_bit_width(&field.bit_width)?;
        let msb = lsb.checked_add(width.saturating_sub(1)).ok_or_else(|| {
            Error::FieldRangeExceedsRegisterSize {
                register: register.into(),
                field: field.name.clone(),
                lsb,
                msb: u64::MAX,
                size: size_bits,
            }
        })?;
        if msb >= size_bits {
            return Err(Error::FieldRangeExceedsRegisterSize {
                register: register.into(),
                field: field.name.clone(),
                lsb,
                msb,
                size: size_bits,
            });
        }
        for (other, other_lsb, other_msb) in &ranges {
            let overlap_lsb = lsb.max(*other_lsb);
            let overlap_msb = msb.min(*other_msb);
            if overlap_lsb <= overlap_msb {
                return Err(Error::FieldRangeOverlap {
                    register: register.into(),
                    field: field.name.clone(),
                    other: (*other).into(),
                    lsb: overlap_lsb,
                    msb: overlap_msb,
                });
            }
        }
        ranges.push((&field.name, lsb, msb));
    }
    Ok(())
}

fn field_view(
    block: &AddressBlock,
    register_volatile: Option<&str>,
    register_access: Option<&str>,
    field: &Field,
    used_names: &mut Vec<String>,
    used_enum_value_names: &mut Vec<String>,
) -> Result<FieldView> {
    let width = parse_field_bit_width(&field.bit_width)?;
    let default_reset_index = default_reset_index(field);
    let reset_value = default_reset_index
        .map(|index| effective_reset_value(&field.resets[index]))
        .transpose()?
        .unwrap_or(0);
    let extra_resets = extra_reset_views(field, width, default_reset_index)?;
    let access = uvm_access(effective_access(block, register_access, field), field)?;
    let var_name = unique_ident(&field.name, used_names);
    let enum_type_name = if field.enumerated_values.is_empty() {
        String::new()
    } else {
        unique_ident(&format!("{var_name}_e"), used_names)
    };
    let enum_values = enum_value_views(
        &field.name,
        &field.enumerated_values,
        width,
        used_enum_value_names,
    )?;
    Ok(FieldView {
        enum_type_name,
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
        has_reset: sv_bool_literal(
            default_reset_index
                .map(|index| reset_has_defined_bits(&field.resets[index]))
                .transpose()?
                .unwrap_or(false),
        ),
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
            if !reset_has_defined_bits(reset)? {
                return Ok(None);
            }
            Ok(Some(ResetView {
                value_literal: format!("{width}'h{:x}", effective_reset_value(reset)?),
                kind: sv_string(reset.reset_type.as_deref().unwrap_or("HARD")),
            }))
        })
        .filter_map(|reset| reset.transpose())
        .collect()
}

fn effective_reset_value(reset: &Reset) -> Result<u64> {
    let value = parse_u64("field reset", &reset.value)?;
    reset
        .mask
        .as_deref()
        .map(|mask| parse_u64("field reset mask", mask).map(|mask| value & mask))
        .unwrap_or(Ok(value))
}

fn reset_has_defined_bits(reset: &Reset) -> Result<bool> {
    reset
        .mask
        .as_deref()
        .map(|mask| parse_u64("field reset mask", mask).map(|mask| mask != 0))
        .unwrap_or(Ok(true))
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
        classes.push(block_class(&scoped_component, true, false, options)?);
    }
    for block in &component.blocks {
        classes.push(address_block_class(component, block, false, options)?);
    }
    classes.push(block_class(component, false, true, options)?);
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
    include_component: bool,
    include_submaps: bool,
    options: RenderOptions,
) -> Result<BlockClass> {
    let class_name = format!("ral_sys_{}", ident(&component.name));
    let mut memories = Vec::new();
    let mut reg_files = Vec::new();
    let mut instances = Vec::new();
    let mut arrays = Vec::new();
    let maps = map_instances(component)?;
    let mut used_names = reserved_block_member_names();
    used_names.extend(maps.iter().map(|map| map.var_name.clone()));
    let layouts = map_layouts(component, &maps);
    validate_map_member_layout(component, &layouts)?;
    let child_blocks =
        child_block_instances(component, &layouts, &mut used_names, include_component)?;

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
    let submaps = if include_submaps {
        submap_instances(component, &layouts, &mut used_names)?
    } else {
        Vec::new()
    };

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
    let layout = map_layout_for_block(block)?;
    let maps = vec![MapInstance {
        var_name: "default_map".into(),
        create_name: sv_string("default_map"),
        n_bytes: layout.n_bytes,
        byte_addressing: sv_bool_literal(layout.byte_addressing),
        is_default: true,
    }];
    let mut used_names = reserved_block_member_names();
    used_names.extend(maps.iter().map(|map| map.var_name.clone()));

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
    used_names: &mut Vec<String>,
    include_component: bool,
) -> Result<Vec<ChildBlockInstance>> {
    let mut children = Vec::new();
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
            var_name: unique_ident(&block.name, used_names),
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

fn submap_instances(
    component: &Component,
    layouts: &BTreeMap<String, MapContext>,
    used_names: &mut Vec<String>,
) -> Result<Vec<SubmapInstance>> {
    let mut submaps = Vec::new();

    for subspace in component.subspace_maps.iter().chain(
        component
            .memory_remaps
            .iter()
            .flat_map(|remap| remap.subspace_maps.iter()),
    ) {
        let address_space = local_address_space_for_subspace(component, subspace)?;
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
        let var_name = unique_ident(&subspace.name, used_names);
        submaps.push(SubmapInstance {
            class_name: format!(
                "ral_sys_{}",
                ident(&format!("{}_{}", component.name, address_space.name))
            ),
            create_name: sv_string(&subspace.name),
            map_var_name: map.var_name.clone(),
            offset_literal: addr_literal(offset),
            var_name,
        });
    }

    Ok(submaps)
}

fn validate_map_member_layout(
    component: &Component,
    layouts: &BTreeMap<String, MapContext>,
) -> Result<()> {
    let mut ranges_by_map = BTreeMap::<String, Vec<(String, u64, u64)>>::new();
    append_map_block_ranges(component.blocks.iter(), layouts, &mut ranges_by_map)?;
    append_map_subspace_ranges(
        component,
        component.subspace_maps.iter(),
        layouts,
        &mut ranges_by_map,
    )?;

    for remap in &component.memory_remaps {
        let mut ranges_by_map = BTreeMap::<String, Vec<(String, u64, u64)>>::new();
        append_map_block_ranges(remap.blocks.iter(), layouts, &mut ranges_by_map)?;
        append_map_subspace_ranges(
            component,
            remap.subspace_maps.iter(),
            layouts,
            &mut ranges_by_map,
        )?;
    }

    Ok(())
}

fn append_map_block_ranges<'a>(
    blocks: impl Iterator<Item = &'a AddressBlock>,
    layouts: &BTreeMap<String, MapContext>,
    ranges_by_map: &mut BTreeMap<String, Vec<(String, u64, u64)>>,
) -> Result<()> {
    for block in blocks {
        let map = layouts
            .get(&block.map_name)
            .ok_or_else(|| Error::MissingElement("memoryMap"))?;
        let start = map_offset_units(
            block,
            "addressBlock baseAddress",
            &block.base_address,
            map.layout,
        )?;
        let length = map_offset_units(block, "addressBlock range", &block.range, map.layout)?;
        append_map_address_range(
            &block.map_name,
            ranges_by_map.entry(block.map_name.clone()).or_default(),
            &block.name,
            start,
            range_end_for_length("addressBlock range", &block.range, start, length)?,
        )?;
    }
    Ok(())
}

fn append_map_subspace_ranges<'a>(
    component: &Component,
    subspaces: impl Iterator<Item = &'a SubspaceMap>,
    layouts: &BTreeMap<String, MapContext>,
    ranges_by_map: &mut BTreeMap<String, Vec<(String, u64, u64)>>,
) -> Result<()> {
    for subspace in subspaces {
        let address_space = local_address_space_for_subspace(component, subspace)?;
        let map = layouts
            .get(&subspace.map_name)
            .ok_or_else(|| Error::MissingElement("memoryMap"))?;
        let start = subspace_parent_offset(component, subspace, map.layout)?;
        let length = subspace_range_units(subspace, address_space, map.layout)?;
        if length == 0 {
            continue;
        }
        append_map_address_range(
            &subspace.map_name,
            ranges_by_map.entry(subspace.map_name.clone()).or_default(),
            &subspace.name,
            start,
            range_end_for_length("subspaceMap range", &subspace.name, start, length)?,
        )?;
    }
    Ok(())
}

fn subspace_parent_offset(
    component: &Component,
    subspace: &SubspaceMap,
    parent_layout: MapLayout,
) -> Result<u64> {
    let offset = map_offset_units_for_address_unit_bits(
        "subspaceMap baseAddress",
        &subspace.base_address,
        parent_layout,
        parse_u64("memoryMap addressUnitBits", &subspace.address_unit_bits)?,
    )?;
    let segment_offset = subspace_segment_offset(component, subspace, parent_layout)?;
    offset
        .checked_sub(segment_offset)
        .ok_or_else(|| Error::InvalidNumber {
            field: "subspaceMap segmentRef addressOffset",
            value: subspace
                .segment_ref
                .as_deref()
                .unwrap_or_default()
                .to_string(),
        })
}

fn local_address_space_for_subspace<'a>(
    component: &'a Component,
    subspace: &SubspaceMap,
) -> Result<&'a AddressSpace> {
    let Some(address_space_ref) = subspace.address_space_ref.as_deref() else {
        return Err(Error::SubspaceMapAddressSpaceNotFound {
            subspace: subspace.name.clone(),
            initiator: subspace.initiator_ref.clone(),
        });
    };
    component
        .address_spaces
        .iter()
        .find(|address_space| address_space.name == address_space_ref)
        .ok_or_else(|| Error::SubspaceMapAddressSpaceNotFound {
            subspace: subspace.name.clone(),
            initiator: subspace.initiator_ref.clone(),
        })
}

fn segment_for_subspace<'a>(
    subspace: &SubspaceMap,
    address_space: &'a AddressSpace,
    segment_ref: &str,
) -> Result<&'a Segment> {
    address_space
        .segments
        .iter()
        .find(|segment| segment.name == segment_ref)
        .ok_or_else(|| Error::SubspaceMapSegmentNotFound {
            subspace: subspace.name.clone(),
            segment: segment_ref.into(),
            address_space: address_space.name.clone(),
        })
}

fn subspace_range_units(
    subspace: &SubspaceMap,
    address_space: &AddressSpace,
    parent_layout: MapLayout,
) -> Result<u64> {
    if let Some(segment_ref) = subspace.segment_ref.as_deref() {
        let segment = segment_for_subspace(subspace, address_space, segment_ref)?;
        validate_segment_ref_range(subspace, address_space, segment, parent_layout)?;
        return segment_range_units(address_space, segment, parent_layout);
    }

    address_space_span_units(address_space, parent_layout)
}

fn segment_range_units(
    address_space: &AddressSpace,
    segment: &Segment,
    parent_layout: MapLayout,
) -> Result<u64> {
    map_offset_units_for_address_unit_bits(
        "addressSpace segment range",
        &segment.range,
        parent_layout,
        parse_u64(
            "addressSpace addressUnitBits",
            &address_space.address_unit_bits,
        )?,
    )
}

fn address_space_span_units(address_space: &AddressSpace, parent_layout: MapLayout) -> Result<u64> {
    let address_unit_bits = parse_u64(
        "addressSpace addressUnitBits",
        &address_space.address_unit_bits,
    )?;
    let mut limit = 0;

    for block in &address_space.blocks {
        let start = map_offset_units_for_address_unit_bits(
            "addressBlock baseAddress",
            &block.base_address,
            parent_layout,
            address_unit_bits,
        )?;
        let length = map_offset_units_for_address_unit_bits(
            "addressBlock range",
            &block.range,
            parent_layout,
            address_unit_bits,
        )?;
        if length == 0 {
            continue;
        }
        let end = range_end_for_length("addressBlock range", &block.range, start, length)?;
        limit = limit.max(end.checked_add(1).ok_or_else(|| Error::InvalidNumber {
            field: "addressSpace localMemoryMap range",
            value: block.range.clone(),
        })?);
    }

    Ok(limit)
}

fn subspace_segment_offset(
    component: &Component,
    subspace: &SubspaceMap,
    parent_layout: MapLayout,
) -> Result<u64> {
    let Some(segment_ref) = subspace.segment_ref.as_deref() else {
        return Ok(0);
    };
    let address_space = local_address_space_for_subspace(component, subspace)?;
    let segment = segment_for_subspace(subspace, address_space, segment_ref)?;
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
    address_space: &AddressSpace,
    segment: &Segment,
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
    validate_block_member_layout(block, block_base, layout)?;
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

fn validate_block_member_layout(
    block: &AddressBlock,
    block_base: u64,
    layout: MapLayout,
) -> Result<()> {
    let mut ranges = Vec::<(String, u64, u64)>::new();

    for register in &block.registers {
        let start = block_base
            + map_offset_units(
                block,
                "register addressOffset",
                &register.address_offset,
                layout,
            )?;
        let element_size = register_size_units(register, layout)?;
        let count = total_element_count("register dim", &register.dims)?;
        let stride = if count > 1 {
            register_stride(block, register, layout)?
        } else {
            element_size
        };
        validate_array_stride(block, &register.name, start, count, stride, element_size)?;
        append_address_range(
            block,
            &mut ranges,
            &register.name,
            start,
            range_end(start, count, stride, element_size)?,
        )?;
    }

    for register_file in &block.register_files {
        let start = block_base
            + map_offset_units(
                block,
                "registerFile addressOffset",
                &register_file.address_offset,
                layout,
            )?;
        let element_size =
            positive_map_offset_units(block, "registerFile range", &register_file.range, layout)?;
        let count = total_element_count("registerFile dim", &register_file.dims)?;
        let stride = if count > 1 {
            register_file_stride(block, register_file, layout)?
        } else {
            element_size
        };
        validate_array_stride(
            block,
            &register_file.name,
            start,
            count,
            stride,
            element_size,
        )?;
        append_address_range(
            block,
            &mut ranges,
            &register_file.name,
            start,
            range_end(start, count, stride, element_size)?,
        )?;
    }

    Ok(())
}

fn validate_register_file_member_layout(
    block: &AddressBlock,
    register_file: &RegisterFile,
    layout: MapLayout,
) -> Result<()> {
    let register_file_range =
        positive_map_offset_units(block, "registerFile range", &register_file.range, layout)?;
    let mut ranges = Vec::<(String, u64, u64)>::new();

    for register in &register_file.registers {
        let start = map_offset_units(
            block,
            "register addressOffset",
            &register.address_offset,
            layout,
        )?;
        let element_size = register_size_units(register, layout)?;
        let count = total_element_count("register dim", &register.dims)?;
        let stride = if count > 1 {
            register_stride(block, register, layout)?
        } else {
            element_size
        };
        validate_register_file_array_stride(
            register_file,
            &register.name,
            start,
            count,
            stride,
            element_size,
        )?;
        let end = range_end(start, count, stride, element_size)?;
        if end >= register_file_range {
            return Err(Error::RegisterFileRangeExceeded {
                register_file: register_file.name.clone(),
                name: register.name.clone(),
                end,
                range: register_file_range,
            });
        }
        append_register_file_address_range(register_file, &mut ranges, &register.name, start, end)?;
    }

    Ok(())
}

fn validate_array_stride(
    block: &AddressBlock,
    name: &str,
    start: u64,
    count: u64,
    stride: u64,
    element_size: u64,
) -> Result<()> {
    if count > 1 && stride < element_size {
        return Err(Error::AddressRangeOverlap {
            block: block.name.clone(),
            name: name.into(),
            other: name.into(),
            start: start + stride,
            end: start + element_size - 1,
        });
    }
    Ok(())
}

fn validate_register_file_array_stride(
    register_file: &RegisterFile,
    name: &str,
    start: u64,
    count: u64,
    stride: u64,
    element_size: u64,
) -> Result<()> {
    if count > 1 && stride < element_size {
        return Err(Error::RegisterFileAddressRangeOverlap {
            register_file: register_file.name.clone(),
            name: name.into(),
            other: name.into(),
            start: start + stride,
            end: start + element_size - 1,
        });
    }
    Ok(())
}

fn append_address_range(
    block: &AddressBlock,
    ranges: &mut Vec<(String, u64, u64)>,
    name: &str,
    start: u64,
    end: u64,
) -> Result<()> {
    for (other, other_start, other_end) in ranges.iter() {
        let overlap_start = start.max(*other_start);
        let overlap_end = end.min(*other_end);
        if overlap_start <= overlap_end {
            return Err(Error::AddressRangeOverlap {
                block: block.name.clone(),
                name: name.into(),
                other: other.clone(),
                start: overlap_start,
                end: overlap_end,
            });
        }
    }
    ranges.push((name.into(), start, end));
    Ok(())
}

fn append_register_file_address_range(
    register_file: &RegisterFile,
    ranges: &mut Vec<(String, u64, u64)>,
    name: &str,
    start: u64,
    end: u64,
) -> Result<()> {
    for (other, other_start, other_end) in ranges.iter() {
        let overlap_start = start.max(*other_start);
        let overlap_end = end.min(*other_end);
        if overlap_start <= overlap_end {
            return Err(Error::RegisterFileAddressRangeOverlap {
                register_file: register_file.name.clone(),
                name: name.into(),
                other: other.clone(),
                start: overlap_start,
                end: overlap_end,
            });
        }
    }
    ranges.push((name.into(), start, end));
    Ok(())
}

fn append_map_address_range(
    map: &str,
    ranges: &mut Vec<(String, u64, u64)>,
    name: &str,
    start: u64,
    end: u64,
) -> Result<()> {
    for (other, other_start, other_end) in ranges.iter() {
        let overlap_start = start.max(*other_start);
        let overlap_end = end.min(*other_end);
        if overlap_start <= overlap_end {
            return Err(Error::MapAddressRangeOverlap {
                map: map.into(),
                name: name.into(),
                other: other.clone(),
                start: overlap_start,
                end: overlap_end,
            });
        }
    }
    ranges.push((name.into(), start, end));
    Ok(())
}

fn range_end_for_length(field: &'static str, value: &str, start: u64, length: u64) -> Result<u64> {
    if length == 0 {
        return Err(Error::InvalidNumber {
            field,
            value: value.into(),
        });
    }
    start
        .checked_add(length - 1)
        .ok_or_else(|| Error::InvalidNumber {
            field: "address range",
            value: format!("{start} + {length}"),
        })
}

fn range_end(start: u64, count: u64, stride: u64, element_size: u64) -> Result<u64> {
    let last_index = count.saturating_sub(1);
    start
        .checked_add(
            last_index
                .checked_mul(stride)
                .ok_or_else(|| Error::InvalidNumber {
                    field: "address range",
                    value: format!("{count} * {stride}"),
                })?,
        )
        .and_then(|last_start| last_start.checked_add(element_size.saturating_sub(1)))
        .ok_or_else(|| Error::InvalidNumber {
            field: "address range",
            value: format!("{start} + {last_index} * {stride} + {element_size}"),
        })
}

fn register_size_units(register: &Register, layout: MapLayout) -> Result<u64> {
    let register_bytes = register_size_bytes(&register.size)?;
    if layout.byte_addressing {
        Ok(register_bytes)
    } else {
        Ok(register_bytes.div_ceil(layout.n_bytes).max(1))
    }
}

fn register_size_bytes(size: &str) -> Result<u64> {
    Ok(parse_register_size_bits(size)?.div_ceil(8))
}

fn parse_field_bit_width(width: &str) -> Result<u64> {
    let width_bits = parse_u64("field bitWidth", width)?;
    if width_bits == 0 {
        return Err(Error::InvalidNumber {
            field: "field bitWidth",
            value: width.into(),
        });
    }
    Ok(width_bits)
}

fn parse_register_size_bits(size: &str) -> Result<u64> {
    let size_bits = parse_u64("register size", size)?;
    if size_bits == 0 {
        return Err(Error::InvalidNumber {
            field: "register size",
            value: size.into(),
        });
    }
    Ok(size_bits)
}

fn total_element_count(field: &'static str, dims: &[String]) -> Result<u64> {
    parse_array_dims(field, dims)?
        .into_iter()
        .try_fold(1u64, |total, dim| {
            total.checked_mul(dim).ok_or_else(|| Error::InvalidNumber {
                field,
                value: dims.join("*"),
            })
        })
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
    let dim = parse_u64("array dim", dim)?;
    if dim == 0 {
        return Err(Error::InvalidNumber {
            field: "array dim",
            value: "0".into(),
        });
    }
    Ok(dim > 1)
}

fn parse_dims(field: &'static str, dims: &[String]) -> Result<Vec<u64>> {
    dims.iter().map(|dim| parse_u64(field, dim)).collect()
}

fn parse_array_dims(field: &'static str, dims: &[String]) -> Result<Vec<u64>> {
    dims.iter()
        .map(|dim| {
            let parsed = parse_u64(field, dim)?;
            if parsed == 0 {
                return Err(Error::InvalidNumber {
                    field,
                    value: dim.clone(),
                });
            }
            Ok(parsed)
        })
        .collect()
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

    let mut used_map_vars = vec!["default_map".to_string()];
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
                        byte_addressing: map.byte_addressing == "1",
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
    if bits != 0 && bits.is_multiple_of(8) {
        Ok(bits / 8)
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

fn positive_map_offset_units(
    block: &AddressBlock,
    field: &'static str,
    value: &str,
    layout: MapLayout,
) -> Result<u64> {
    let units = map_offset_units(block, field, value, layout)?;
    if units == 0 {
        return Err(Error::InvalidNumber {
            field,
            value: value.into(),
        });
    }
    Ok(units)
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
    let range = parse_u64(field, value)?;
    if range == 0 {
        return Err(Error::InvalidNumber {
            field,
            value: value.into(),
        });
    }
    Ok(range
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
        rights: sv_string(&memory_rights(block)?),
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
        parse_array_dims("registerFile dim", &register_file.dims)?
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
            sv_named_call(
                "      ",
                &format!("{var_name}.configure"),
                &[
                    ("blk_parent", "this".into()),
                    ("regfile_parent", "null".into()),
                    ("hdl_path", hdl_path_expr.into()),
                ],
            ),
            format!("      {var_name}.build();"),
            sv_named_call(
                "      ",
                &format!("{var_name}.map"),
                &[
                    ("mp", map_var_name.into()),
                    ("offset", addr_literal(base_offset)),
                ],
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
    lines.push(sv_named_call(
        &body_indent,
        &format!("{element_ref}.configure"),
        &[
            ("blk_parent", "this".into()),
            ("regfile_parent", "null".into()),
            ("hdl_path", hdl_path_expr.into()),
        ],
    ));
    lines.push(format!("{body_indent}{element_ref}.build();"));
    lines.push(sv_named_call(
        &body_indent,
        &format!("{element_ref}.map"),
        &[
            ("mp", map_var_name.into()),
            (
                "offset",
                format!(
                    "{} + {linear_index} * {}",
                    addr_literal(base_offset),
                    addr_literal(stride)
                ),
            ),
        ],
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
        lines.push(sv_named_call(
            "      ",
            &format!("{var_name}.configure"),
            &[
                ("blk_parent", "get_block()".into()),
                ("regfile_parent", "this".into()),
            ],
        ));
        lines.push(format!("      {var_name}.build();"));
        for slice in hdl_slices {
            lines.push(sv_named_call(
                "      ",
                &format!("{var_name}.add_hdl_path_slice"),
                &[
                    ("name", slice.path_expr.clone()),
                    ("offset", slice.offset.to_string()),
                    ("size", slice.size.to_string()),
                    ("first", slice.first.into()),
                ],
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
    lines.push(sv_named_call(
        &body_indent,
        &format!("{element_ref}.configure"),
        &[
            ("blk_parent", "get_block()".into()),
            ("regfile_parent", "this".into()),
        ],
    ));
    lines.push(format!("{body_indent}{element_ref}.build();"));
    for slice in hdl_slices {
        lines.push(sv_named_call(
            &body_indent,
            &format!("{element_ref}.add_hdl_path_slice"),
            &[
                ("name", slice.path_expr.clone()),
                ("offset", slice.offset.to_string()),
                ("size", slice.size.to_string()),
                ("first", slice.first.into()),
            ],
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
        return vec![sv_named_call(
            "      ",
            "mp.add_reg",
            &[
                ("rg", var_name.into()),
                ("offset", format!("offset + {}", addr_literal(base_offset))),
                ("rights", sv_string(rights)),
            ],
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
    lines.push(sv_named_call(
        &body_indent,
        "mp.add_reg",
        &[
            ("rg", element_ref),
            (
                "offset",
                format!("offset + {} + {offset_expr}", addr_literal(base_offset)),
            ),
            ("rights", sv_string(rights)),
        ],
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
    let dims = parse_array_dims("register dim", &register.dims)?;
    let hdl_slices = hdl_slices(
        register,
        register.hdl_path.as_ref().or(block.hdl_path.as_ref()),
    )?;
    let indexed_hdl_slices = indexed_hdl_slices(register, &dims)?;
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
    let dims = parse_array_dims("register dim", &register.dims)?;
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
    lines.push(sv_named_call(
        &body_indent,
        &format!("{element_ref}.configure"),
        &array_configure_args(spec.regfile_parent, &indices),
    ));
    lines.push(format!("{body_indent}{element_ref}.build();"));
    for slice in spec.hdl_slices {
        lines.push(sv_named_call(
            &body_indent,
            &format!("{element_ref}.add_hdl_path_slice"),
            &[
                ("name", slice.path_expr.clone()),
                ("offset", slice.offset.to_string()),
                ("size", slice.size.to_string()),
                ("first", slice.first.into()),
            ],
        ));
    }
    for indexed in spec.indexed_hdl_slices {
        lines.push(format!(
            "{body_indent}if ({}) begin",
            index_condition(&indices, &indexed.indices)
        ));
        for slice in &indexed.slices {
            let branch_indent = indent(indices.len() + 1);
            lines.push(sv_named_call(
                &branch_indent,
                &format!("{element_ref}.add_hdl_path_slice"),
                &[
                    ("name", slice.path_expr.clone()),
                    ("offset", slice.offset.to_string()),
                    ("size", slice.size.to_string()),
                    ("first", slice.first.into()),
                ],
            ));
        }
        lines.push(format!("{body_indent}end"));
    }
    lines.push(sv_named_call(
        &body_indent,
        &format!("{}.add_reg", spec.map_var_name),
        &[
            ("rg", element_ref),
            (
                "offset",
                format!("{} + {offset_expr}", addr_literal(spec.base_offset)),
            ),
            ("rights", sv_string(spec.rights)),
        ],
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

fn array_configure_args(
    regfile_parent: Option<ArrayParentSpec<'_>>,
    indices: &[String],
) -> Vec<(&'static str, String)> {
    match regfile_parent {
        Some(parent) => {
            let parent_ref = array_parent_ref(parent, indices);
            vec![
                ("blk_parent", "get_block()".into()),
                ("regfile_parent", parent_ref),
            ]
        }
        None => vec![("blk_parent", "this".into())],
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
            let register_bytes = register_size_bytes(&register.size)?;
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
            positive_map_offset_units(block, "registerFile range", &register_file.range, layout)
        })
}

fn hdl_slices(register: &Register, register_hdl_path: Option<&String>) -> Result<Vec<HdlSlice>> {
    hdl_slices_from_fields(&register.fields, register_hdl_path)
}

fn indexed_hdl_slices(register: &Register, dims: &[u64]) -> Result<Vec<IndexedHdlSlices>> {
    validate_unique_indexed_hdl_paths(&register.name, dims, &register.indexed_hdl_paths)?;
    for field in &register.fields {
        validate_unique_indexed_hdl_paths(
            &format!("{}.{}", register.name, field.name),
            dims,
            &field.indexed_hdl_paths,
        )?;
    }

    let mut indexed_slices = register
        .indexed_hdl_paths
        .iter()
        .map(|indexed| {
            Ok(IndexedHdlSlices {
                indices: indexed_access_handle_indices(&register.name, dims, indexed)?,
                slices: hdl_slices_from_fields(&register.fields, Some(&indexed.path))?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    for field in &register.fields {
        let field_owner = format!("{}.{}", register.name, field.name);
        for indexed in &field.indexed_hdl_paths {
            let indices = indexed_access_handle_indices(&field_owner, dims, indexed)?;
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
    dims: &[u64],
    indexed_paths: &[irgen_ipxact_model::IndexedHdlPath],
) -> Result<()> {
    let mut used = Vec::new();
    for indexed in indexed_paths {
        let indices = indexed_access_handle_indices(owner, dims, indexed)?;
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
    owner: &str,
    dims: &[u64],
    indexed: &irgen_ipxact_model::IndexedHdlPath,
) -> Result<Vec<u64>> {
    let indices = parse_dims("accessHandle index", &indexed.indices)?;
    if indices.len() != dims.len() {
        return Err(Error::AccessHandleIndexDimensionMismatch {
            register: owner.into(),
            expected: dims.len(),
            actual: indices.len(),
        });
    }
    for (dimension, (index, size)) in indices.iter().zip(dims).enumerate() {
        if index >= size {
            return Err(Error::AccessHandleIndexOutOfRange {
                owner: owner.into(),
                dimension: dimension + 1,
                index: *index,
                size: *size,
            });
        }
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
                size: parse_field_bit_width(&field.bit_width)? as i64,
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
    let field_width = parse_field_bit_width(&field.bit_width)?;
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
        (None, None) if slice_count == 1 => parse_field_bit_width(&field.bit_width),
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
    Ok(parse_address_block_width_bits(&block.width)?.div_ceil(8))
}

fn parse_address_block_width_bits(width: &str) -> Result<u64> {
    let width_bits = parse_u64("addressBlock width", width)?;
    if width_bits == 0 {
        return Err(Error::InvalidNumber {
            field: "addressBlock width",
            value: width.into(),
        });
    }
    Ok(width_bits)
}

fn memory_size_words(block: &AddressBlock) -> Result<u64> {
    let range = range_bytes(block, "addressBlock range", &block.range)?;
    Ok(range.div_ceil(width_bytes(block)?))
}

fn is_memory_block(block: &AddressBlock) -> bool {
    block
        .usage
        .as_deref()
        .is_some_and(|usage| usage.eq_ignore_ascii_case("memory"))
}

fn memory_rights(block: &AddressBlock) -> Result<String> {
    match block.access.as_deref().unwrap_or("read-write") {
        "read-only" => Ok("RO".into()),
        "read-write" => Ok("RW".into()),
        access => Err(Error::UnsupportedMemoryAccess {
            block: block.name.clone(),
            access: access.into(),
        }),
    }
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
        let access = effective_access(block, register_access, field);
        has_read |= access_allows_read(access);
        has_write |= access_allows_write(access);
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

fn uvm_access(access: &str, field: &Field) -> Result<String> {
    let uvm_access = match (
        access,
        field.modified_write_value.as_deref(),
        field.read_action.as_deref(),
    ) {
        (_, Some("oneToSet"), Some("clear")) => "W1SRC",
        (_, Some("oneToClear"), Some("set")) => "W1CRS",
        (_, Some("zeroToSet"), Some("clear")) => "W0SRC",
        (_, Some("zeroToClear"), Some("set")) => "W0CRS",
        (_, Some("set"), Some("clear")) => "WSRC",
        (_, Some("clear"), Some("set")) => "WCRS",
        ("write-only", Some("clear"), _) => "WOC",
        ("write-only", Some("set"), _) => "WOS",
        (_, Some("oneToClear"), _) => "W1C",
        (_, Some("oneToSet"), _) => "W1S",
        (_, Some("oneToToggle"), _) => "W1T",
        (_, Some("zeroToClear"), _) => "W0C",
        (_, Some("zeroToSet"), _) => "W0S",
        (_, Some("zeroToToggle"), _) => "W0T",
        (_, Some("clear"), _) => "WC",
        (_, Some("set"), _) => "WS",
        ("read-write", _, Some("clear")) => "WRC",
        ("read-write", _, Some("set")) => "WRS",
        ("read-only", None, Some("clear")) => "RC",
        ("read-only", None, Some("set")) => "RS",
        ("read-only", None, None) => "RO",
        ("write-only", None, None) => "WO",
        ("writeOnce", None, None) => "WO1",
        ("read-writeOnce", None, None) => "W1",
        ("no-access", None, None) => "NOACCESS",
        ("read-write", None, None) => "RW",
        _ => {
            return Err(Error::UnsupportedAccessPolicy {
                field: field.name.clone(),
                access: access.into(),
                modified_write_value: field.modified_write_value.clone(),
                read_action: field.read_action.clone(),
            });
        }
    };
    Ok(uvm_access.into())
}

fn access_allows_read(access: &str) -> bool {
    !matches!(access, "write-only" | "writeOnce" | "no-access")
}

fn access_allows_write(access: &str) -> bool {
    !matches!(access, "read-only" | "no-access")
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
    if value { "1" } else { "0" }
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

fn sv_named_call(indent: &str, callee: &str, args: &[(&str, String)]) -> String {
    let mut out = String::new();
    out.push_str(indent);
    out.push_str(callee);
    out.push('(');
    if args.len() <= 3 {
        for (index, (_, value)) in args.iter().enumerate() {
            if index > 0 {
                out.push_str(", ");
            }
            out.push_str(value);
        }
        out.push_str(");");
        return out;
    }

    for (index, (name, value)) in args.iter().enumerate() {
        out.push('\n');
        out.push_str(indent);
        out.push_str("  .");
        out.push_str(name);
        out.push('(');
        out.push_str(value);
        out.push(')');
        if index + 1 != args.len() {
            out.push(',');
        }
    }
    out.push('\n');
    out.push_str(indent);
    out.push_str(");");
    out
}

fn unique_const_ident(value: &str, used: &mut Vec<String>) -> String {
    unique_ident(value, used).to_ascii_uppercase()
}

fn reserved_block_member_names() -> Vec<String> {
    reserved_generated_names(&[
        "build",
        "configure",
        "create_map",
        "default_map",
        "get_full_name",
        "get_name",
        "lock_model",
        "name",
        "new",
        "parent",
    ])
}

fn reserved_register_member_names(coverage: bool) -> Vec<String> {
    let mut names = reserved_generated_names(&[
        "add_coverage",
        "build",
        "build_coverage",
        "configure",
        "get_coverage",
        "map",
        "name",
        "new",
        "parent",
        "sample",
    ]);
    if coverage {
        names.extend(reserved_generated_names(&[
            "byte_en",
            "cg_bits",
            "data",
            "is_read",
            "m_be",
            "m_data",
            "m_is_read",
        ]));
    }
    names
}

fn reserved_register_file_member_names() -> Vec<String> {
    reserved_generated_names(&[
        "build",
        "configure",
        "get_block",
        "map",
        "mp",
        "name",
        "new",
        "offset",
        "parent",
    ])
}

fn reserved_generated_names(values: &[&str]) -> Vec<String> {
    let mut names = values.iter().map(|value| ident(value)).collect::<Vec<_>>();
    names.push("i".into());
    for index in 0..8 {
        names.push(format!("i{index}"));
    }
    names.sort();
    names.dedup();
    names
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
