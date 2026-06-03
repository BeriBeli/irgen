use crate::ast::*;
use crate::util::{quote_attr_value, sanitize_doc};
use crate::writer::Writer;

pub fn serialize_document(document: &Document) -> String {
    let mut writer = Writer::default();
    for item in &document.items {
        write_top_level_item(&mut writer, item);
        writer.blank_line();
    }
    writer.finish()
}

fn write_top_level_item(writer: &mut Writer, item: &TopLevelItem) {
    match item {
        TopLevelItem::Source(path) => writer.line(format!("source {path}")),
        TopLevelItem::Field(field) => write_field_def(writer, field, None),
        TopLevelItem::Register(register) => write_register_def(writer, register, None),
        TopLevelItem::RegFile(regfile) => write_regfile_def(writer, regfile, None),
        TopLevelItem::Memory(memory) => write_memory_def(writer, memory, None),
        TopLevelItem::VirtualRegister(vreg) => write_virtual_register_def(writer, vreg, None),
        TopLevelItem::Block(block) => write_block_def(writer, block, None),
        TopLevelItem::System(system) => write_system_def(writer, system, None),
        TopLevelItem::RegisterCallback(callback) => {
            write_callback_class(writer, "register_cb", callback)
        }
        TopLevelItem::FieldCallback(callback) => {
            write_callback_class(writer, "field_cb_class", callback)
        }
        TopLevelItem::Raw(raw) => writer.raw_block(raw),
    }
}

fn write_field_def(writer: &mut Writer, field: &Field, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("field {} {{", field.name)));
    writer.indent();
    write_attributes(writer, &field.attributes);
    if let Some(bits) = &field.bits {
        writer.line(format!("bits {bits};"));
    }
    if let Some(access) = &field.access {
        writer.line(format!("access {};", access.as_str()));
    }
    if let Some(reset) = &field.hard_reset {
        writer.line(format!("hard_reset {reset};"));
    }
    if let Some(reset) = &field.soft_reset {
        writer.line(format!("soft_reset {reset};"));
    }
    if let Some(volatile) = field.volatile {
        writer.line(format!("volatile {};", u8::from(volatile)));
    }
    for constraint in &field.constraints {
        write_constraint(writer, constraint);
    }
    if !field.enum_values.is_empty() {
        writer.line(format!(
            "enum {{ {} }}",
            field
                .enum_values
                .iter()
                .map(|value| match &value.value {
                    Some(v) => format!("{}={v}", value.name),
                    None => value.name.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    for cover in &field.cover {
        writer.line(format!("cover {};", cover.as_str()));
    }
    for coverpoint in &field.coverpoints {
        writer.line("coverpoint {");
        writer.indent();
        writer.raw_indented_block(&coverpoint.body);
        writer.dedent();
        writer.line("}");
    }
    write_doc(writer, field.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_register_def(writer: &mut Writer, register: &Register, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("register {} {{", register.name)));
    writer.indent();
    write_attributes(writer, &register.attributes);
    if let Some(bytes) = &register.bytes {
        writer.line(format!("bytes {bytes};"));
    }
    if register.left_to_right {
        writer.line("left_to_right;");
    }
    for field in &register.fields {
        write_field_instance(writer, field);
    }
    for constraint in &register.constraints {
        write_constraint(writer, constraint);
    }
    if let Some(noise) = &register.noise {
        writer.line(format!("noise {};", noise.as_str()));
    }
    write_shared(writer, register.shared.as_ref());
    for cover in &register.cover {
        writer.line(format!("cover {};", cover.as_str()));
    }
    for cross in &register.crosses {
        write_cross(writer, cross);
    }
    for user_code in &register.user_codes {
        write_user_code(writer, user_code);
    }
    for callback in &register.add_reg_callbacks {
        write_add_reg_callback(writer, callback);
    }
    write_doc(writer, register.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_regfile_def(writer: &mut Writer, regfile: &RegFile, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("regfile {} {{", regfile.name)));
    writer.indent();
    write_attributes(writer, &regfile.attributes);
    for register in &regfile.registers {
        write_register_instance(writer, register);
    }
    for constraint in &regfile.constraints {
        write_constraint(writer, constraint);
    }
    write_shared(writer, regfile.shared.as_ref());
    for cover in &regfile.cover {
        writer.line(format!("cover {};", cover.as_str()));
    }
    for user_code in &regfile.user_codes {
        write_user_code(writer, user_code);
    }
    for callback in &regfile.add_reg_callbacks {
        write_add_reg_callback(writer, callback);
    }
    write_doc(writer, regfile.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_memory_def(writer: &mut Writer, memory: &Memory, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("memory {} {{", memory.name)));
    writer.indent();
    write_attributes(writer, &memory.attributes);
    if let Some(size) = &memory.size {
        writer.line(format!("size {size};"));
    }
    if let Some(bits) = &memory.bits {
        writer.line(format!("bits {bits};"));
    }
    if let Some(access) = &memory.access {
        writer.line(format!("access {};", access.as_str()));
    }
    if let Some(initial) = &memory.initial {
        writer.line(format!("initial {};", initial.as_str()));
    }
    write_shared(writer, memory.shared.as_ref());
    for cover in &memory.cover {
        writer.line(format!("cover {};", cover.as_str()));
    }
    for user_code in &memory.user_codes {
        write_user_code(writer, user_code);
    }
    write_doc(writer, memory.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_virtual_register_def(writer: &mut Writer, vreg: &VirtualRegister, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("virtual register {} {{", vreg.name)));
    writer.indent();
    write_attributes(writer, &vreg.attributes);
    if let Some(bytes) = &vreg.bytes {
        writer.line(format!("bytes {bytes};"));
    }
    if vreg.left_to_right {
        writer.line("left_to_right;");
    }
    for field in &vreg.fields {
        write_virtual_field_instance(writer, field);
    }
    for user_code in &vreg.user_codes {
        write_user_code(writer, user_code);
    }
    write_doc(writer, vreg.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_block_def(writer: &mut Writer, block: &Block, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("block {} {{", block.name)));
    writer.indent();
    write_attributes(writer, &block.attributes);
    if block.domains.is_empty() {
        write_addressable_body(writer, &block.body);
    } else {
        for domain in &block.domains {
            write_domain(writer, domain, write_addressable_body);
        }
    }
    if let Some(default_map_name) = &block.default_map_name {
        writer.line(format!("default_map_name {default_map_name};"));
    }
    for user_code in &block.user_codes {
        write_user_code(writer, user_code);
    }
    for callback in &block.add_reg_callbacks {
        write_add_reg_callback(writer, callback);
    }
    write_doc(writer, block.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_system_def(writer: &mut Writer, system: &System, header: Option<String>) {
    writer.line(header.unwrap_or_else(|| format!("system {} {{", system.name)));
    writer.indent();
    write_attributes(writer, &system.attributes);
    if system.domains.is_empty() {
        write_hierarchy_body(writer, &system.body);
    } else {
        for domain in &system.domains {
            write_domain(writer, domain, write_hierarchy_body);
        }
    }
    for user_code in &system.user_codes {
        write_user_code(writer, user_code);
    }
    write_doc(writer, system.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_domain<T>(writer: &mut Writer, domain: &Domain<T>, write_body: fn(&mut Writer, &T)) {
    writer.line(format!("domain {} {{", domain.name));
    writer.indent();
    write_attributes(writer, &domain.attributes);
    write_body(writer, &domain.body);
    write_doc(writer, domain.doc.as_deref());
    writer.dedent();
    writer.line("}");
}

fn write_addressable_body(writer: &mut Writer, body: &AddressableBody) {
    if let Some(bytes) = &body.bytes {
        writer.line(format!("bytes {bytes};"));
    }
    if let Some(endian) = &body.endian {
        writer.line(format!("endian {};", endian.as_str()));
    }
    for register in &body.registers {
        write_register_instance(writer, register);
    }
    for regfile in &body.regfiles {
        write_regfile_instance(writer, regfile);
    }
    for memory in &body.memories {
        write_memory_instance(writer, memory);
    }
    for vreg in &body.virtual_registers {
        write_virtual_register_instance(writer, vreg);
    }
    for block in &body.blocks {
        write_block_instance(writer, block);
    }
    for constraint in &body.constraints {
        write_constraint(writer, constraint);
    }
    for cover in &body.cover {
        writer.line(format!("cover {};", cover.as_str()));
    }
}

fn write_hierarchy_body(writer: &mut Writer, body: &HierarchyBody) {
    if let Some(bytes) = &body.bytes {
        writer.line(format!("bytes {bytes};"));
    }
    if let Some(endian) = &body.endian {
        writer.line(format!("endian {};", endian.as_str()));
    }
    for block in &body.blocks {
        write_block_instance(writer, block);
    }
    for system in &body.systems {
        write_system_instance(writer, system);
    }
    for constraint in &body.constraints {
        write_constraint(writer, constraint);
    }
    for cover in &body.cover {
        writer.line(format!("cover {};", cover.as_str()));
    }
}

fn write_field_instance(writer: &mut Writer, field: &FieldInstance) {
    let header = format!("{} {{", field_instance_prefix(field));
    if let Some(definition) = &field.definition {
        write_field_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", field_instance_prefix(field)));
    }
}

fn write_register_instance(writer: &mut Writer, register: &RegisterInstance) {
    let header = format!("{} {{", register_instance_prefix(register));
    if let Some(definition) = &register.definition {
        write_register_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", register_instance_prefix(register)));
    }
}

fn write_regfile_instance(writer: &mut Writer, regfile: &RegFileInstance) {
    let header = format!("{} {{", regfile_instance_prefix(regfile));
    if let Some(definition) = &regfile.definition {
        write_regfile_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", regfile_instance_prefix(regfile)));
    }
}

fn write_memory_instance(writer: &mut Writer, memory: &MemoryInstance) {
    let header = format!("{} {{", memory_instance_prefix(memory));
    if let Some(definition) = &memory.definition {
        write_memory_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", memory_instance_prefix(memory)));
    }
}

fn write_virtual_register_instance(writer: &mut Writer, vreg: &VirtualRegisterInstance) {
    let header = format!("{} {{", virtual_register_instance_prefix(vreg));
    if let Some(definition) = &vreg.definition {
        write_virtual_register_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", virtual_register_instance_prefix(vreg)));
    }
}

fn write_block_instance(writer: &mut Writer, block: &BlockInstance) {
    let header = format!("{} {{", block_instance_prefix(block));
    if let Some(definition) = &block.definition {
        write_block_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", block_instance_prefix(block)));
    }
}

fn write_system_instance(writer: &mut Writer, system: &SystemInstance) {
    let header = format!("{} {{", system_instance_prefix(system));
    if let Some(definition) = &system.definition {
        write_system_def(writer, definition, Some(header));
    } else {
        writer.line(format!("{};", system_instance_prefix(system)));
    }
}

fn write_virtual_field_instance(writer: &mut Writer, field: &VirtualFieldInstance) {
    let mut prefix = format!("field {}", field.name);
    if let Some(rename) = &field.rename {
        prefix.push('=');
        prefix.push_str(rename);
    }
    if let Some(offset) = &field.offset {
        prefix.push_str(" @");
        prefix.push_str(offset);
    }
    if let Some(definition) = &field.definition {
        write_field_def(writer, definition, Some(format!("{prefix} {{")));
    } else if let Some(bits) = &field.bits {
        writer.line(format!("{prefix} {{"));
        writer.indent();
        writer.line(format!("bits {bits};"));
        write_doc(writer, field.doc.as_deref());
        writer.dedent();
        writer.line("}");
    } else {
        writer.line(format!("{prefix};"));
    }
}

fn field_instance_prefix(field: &FieldInstance) -> String {
    let mut prefix = format!(
        "field {}",
        format_named_instance(&field.name, field.rename.as_deref(), field.array.as_ref())
    );
    append_optional_paren(&mut prefix, field.hdl_path.as_deref());
    append_optional_offset(
        &mut prefix,
        field.offset.as_deref(),
        field.increment.as_deref(),
    );
    prefix
}

fn register_instance_prefix(register: &RegisterInstance) -> String {
    let mut prefix = format!(
        "register {}",
        format_named_instance(
            &register.name,
            register.rename.as_deref(),
            register.array.as_ref()
        )
    );
    append_optional_paren(&mut prefix, register.hdl_path.as_deref());
    append_optional_offset(
        &mut prefix,
        register.offset.as_deref(),
        register.increment.as_deref(),
    );
    append_instance_access(&mut prefix, register.access.as_ref());
    prefix
}

fn regfile_instance_prefix(regfile: &RegFileInstance) -> String {
    let mut prefix = format!(
        "regfile {}",
        format_named_instance(
            &regfile.name,
            regfile.rename.as_deref(),
            regfile.array.as_ref()
        )
    );
    append_optional_paren(&mut prefix, regfile.hdl_path.as_deref());
    append_optional_offset(
        &mut prefix,
        regfile.offset.as_deref(),
        regfile.increment.as_deref(),
    );
    append_instance_access(&mut prefix, regfile.access.as_ref());
    prefix
}

fn memory_instance_prefix(memory: &MemoryInstance) -> String {
    let mut prefix = format_named_instance(&memory.name, memory.rename.as_deref(), None);
    append_optional_paren(&mut prefix, memory.hdl_path.as_deref());
    append_optional_offset(&mut prefix, memory.offset.as_deref(), None);
    append_instance_access(&mut prefix, memory.access.as_ref());
    prefix
}

fn virtual_register_instance_prefix(vreg: &VirtualRegisterInstance) -> String {
    let mut prefix = format!(
        "virtual register {}",
        format_named_instance(&vreg.name, vreg.rename.as_deref(), vreg.array.as_ref())
    );
    if let Some(memory) = &vreg.memory {
        prefix.push(' ');
        prefix.push_str(memory);
        if let Some(offset) = &vreg.memory_offset {
            prefix.push('@');
            prefix.push_str(offset);
        }
    }
    if let Some(increment) = &vreg.increment {
        prefix.push_str(" +");
        prefix.push_str(increment);
    }
    prefix
}

fn block_instance_prefix(block: &BlockInstance) -> String {
    let mut name = block.name.clone();
    if let Some(domain) = &block.domain {
        name.push('.');
        name.push_str(domain);
    }
    let mut prefix = format!(
        "block {}",
        format_named_instance(&name, block.rename.as_deref(), block.array.as_ref())
    );
    append_optional_paren(&mut prefix, block.hdl_path.as_deref());
    append_required_offset(&mut prefix, &block.offset, block.increment.as_deref());
    prefix
}

fn system_instance_prefix(system: &SystemInstance) -> String {
    let mut name = system.name.clone();
    if let Some(domain) = &system.domain {
        name.push('.');
        name.push_str(domain);
    }
    let mut prefix = format!(
        "system {}",
        format_named_instance(&name, system.rename.as_deref(), system.array.as_ref())
    );
    append_optional_paren(&mut prefix, system.hdl_path.as_deref());
    append_required_offset(&mut prefix, &system.offset, system.increment.as_deref());
    prefix
}

fn format_named_instance(name: &str, rename: Option<&str>, array: Option<&Array>) -> String {
    let mut output = name.to_owned();
    if let Some(rename) = rename {
        output.push('=');
        output.push_str(rename);
    }
    if let Some(array) = array {
        output.push_str(&array.as_str());
    }
    output
}

fn append_optional_paren(output: &mut String, value: Option<&str>) {
    if let Some(value) = value {
        output.push_str(" (");
        output.push_str(value);
        output.push(')');
    }
}

fn append_optional_offset(output: &mut String, offset: Option<&str>, increment: Option<&str>) {
    if let Some(offset) = offset {
        output.push_str(" @");
        output.push_str(offset);
    }
    if let Some(increment) = increment {
        output.push_str(" +");
        output.push_str(increment);
    }
}

fn append_required_offset(output: &mut String, offset: &str, increment: Option<&str>) {
    output.push_str(" @");
    output.push_str(offset);
    if let Some(increment) = increment {
        output.push_str(" +");
        output.push_str(increment);
    }
}

fn append_instance_access(output: &mut String, access: Option<&InstanceAccess>) {
    if let Some(access) = access {
        output.push(' ');
        output.push_str(access.as_str());
    }
}

fn write_constraint(writer: &mut Writer, constraint: &Constraint) {
    match &constraint.body {
        Some(body) => {
            writer.line(format!("constraint {} {{", constraint.name));
            writer.indent();
            writer.raw_indented_block(body);
            writer.dedent();
            writer.line("}");
        }
        None => writer.line(format!("constraint {};", constraint.name)),
    }
}

fn write_cross(writer: &mut Writer, cross: &Cross) {
    let items = cross.items.join(" ");
    if let Some(label) = &cross.label {
        writer.line(format!("cross {items} {{"));
        writer.indent();
        writer.line(format!("label {label};"));
        writer.dedent();
        writer.line("}");
    } else {
        writer.line(format!("cross {items};"));
    }
}

fn write_user_code(writer: &mut Writer, user_code: &UserCode) {
    let mut header = String::from("user_code");
    if let Some(lang) = &user_code.lang {
        header.push_str(" lang=");
        header.push_str(lang);
    }
    if let Some(scope) = &user_code.scope {
        header.push_str(" (");
        header.push_str(scope);
        header.push(')');
    }
    writer.line(format!("{header} {{"));
    writer.indent();
    writer.raw_indented_block(&user_code.body);
    writer.dedent();
    writer.line("}");
}

fn write_callback_class(writer: &mut Writer, keyword: &str, callback: &CallbackClass) {
    writer.line(format!("{keyword} {} {{", callback.name));
    writer.indent();
    if let Some(body) = &callback.var_declarations {
        writer.line("var_declarations {");
        writer.indent();
        writer.raw_indented_block(body);
        writer.dedent();
        writer.line("}");
    }
    if let Some(method) = &callback.new_method {
        let args = method.args.as_deref().unwrap_or("");
        writer.line(format!("new_method ({args}) {{"));
        writer.indent();
        if let Some(body) = &method.body {
            writer.raw_indented_block(body);
        }
        writer.dedent();
        writer.line("}");
    }
    write_callback_method(
        writer,
        "pre_write_method",
        callback.pre_write_method.as_deref(),
    );
    write_callback_method(
        writer,
        "post_write_method",
        callback.post_write_method.as_deref(),
    );
    write_callback_method(
        writer,
        "pre_read_method",
        callback.pre_read_method.as_deref(),
    );
    write_callback_method(
        writer,
        "post_read_method",
        callback.post_read_method.as_deref(),
    );
    writer.dedent();
    writer.line("}");
}

fn write_callback_method(writer: &mut Writer, name: &str, body: Option<&str>) {
    if let Some(body) = body {
        writer.line(format!("{name} {{"));
        writer.indent();
        writer.raw_indented_block(body);
        writer.dedent();
        writer.line("}");
    }
}

fn write_add_reg_callback(writer: &mut Writer, callback: &AddRegCallback) {
    let mut line = String::from("add_reg_cb");
    if let Some(target) = &callback.target {
        line.push(' ');
        line.push_str(target);
    }
    line.push(' ');
    line.push_str(&callback.callback_class);
    if let Some(args) = &callback.args {
        line.push_str(" (");
        line.push_str(args);
        line.push(')');
    }
    if callback.external_cb_class {
        line.push_str(" external_cb_class");
    }
    line.push(';');
    writer.line(line);
}

fn write_shared(writer: &mut Writer, shared: Option<&Option<String>>) {
    match shared {
        Some(Some(path)) => writer.line(format!("shared ({path});")),
        Some(None) => writer.line("shared;"),
        None => {}
    }
}

fn write_attributes(writer: &mut Writer, attributes: &Attributes) {
    if attributes.entries.is_empty() {
        return;
    }

    writer.line("attributes {");
    writer.indent();
    for (index, attr) in attributes.entries.iter().enumerate() {
        let suffix = if index + 1 == attributes.entries.len() {
            ""
        } else {
            ","
        };
        writer.line(format!(
            "{} {}{}",
            attr.name,
            quote_attr_value(&attr.value),
            suffix
        ));
    }
    writer.dedent();
    writer.line("}");
}

fn write_doc(writer: &mut Writer, doc: Option<&str>) {
    if let Some(doc) = doc {
        writer.line(format!("doc {{ {} }}", sanitize_doc(doc)));
    }
}
