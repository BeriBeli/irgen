use crate::ast::*;
use crate::writer::Writer;

pub fn serialize_document(document: &Document) -> String {
    let mut writer = Writer::default();
    if let Some(package) = &document.package {
        writer.line(format!("package {package};"));
        writer.blank_line();
    }
    for import in &document.imports {
        let suffix = if import.wildcard { "::*" } else { "" };
        writer.line(format!("import {}{};", import.path, suffix));
    }
    if !document.imports.is_empty() {
        writer.blank_line();
    }
    for declaration in &document.declarations {
        write_declaration(&mut writer, declaration);
        writer.blank_line();
    }
    writer.finish()
}

fn write_declaration(writer: &mut Writer, declaration: &Declaration) {
    match declaration {
        Declaration::Enum(enum_decl) => write_enum_decl(writer, enum_decl),
        Declaration::Struct(struct_decl) => write_struct_decl(writer, struct_decl),
        Declaration::Property(property) => write_property_decl(writer, property),
        Declaration::Component(component) => write_component_def(writer, component, None),
        Declaration::Raw(raw) => writer.raw_block(raw),
    }
}

fn write_enum_decl(writer: &mut Writer, enum_decl: &EnumDecl) {
    writer.line(format!("enum {} {{", enum_decl.name));
    writer.indent();
    for variant in &enum_decl.variants {
        match &variant.value {
            Some(value) => writer.line(format!("{} = {},", variant.name, expr(value))),
            None => writer.line(format!("{},", variant.name)),
        }
    }
    writer.dedent();
    writer.line("};");
}

fn write_struct_decl(writer: &mut Writer, struct_decl: &StructDecl) {
    writer.line(format!("struct {} {{", struct_decl.name));
    writer.indent();
    for field in &struct_decl.fields {
        writer.line(format!("{} {};", field.ty, field.name));
    }
    writer.dedent();
    writer.line("};");
}

fn write_property_decl(writer: &mut Writer, property: &PropertyDecl) {
    let components = property
        .component_kinds
        .iter()
        .map(ComponentKind::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let default = property
        .default
        .as_ref()
        .map(|value| format!(" = {}", expr(value)))
        .unwrap_or_default();
    writer.line(format!(
        "property {} {{ type = {}; component = {{{components}}};{default}; }};",
        property.name,
        property.ty.as_str()
    ));
}

fn write_component_def(writer: &mut Writer, component: &Component, header: Option<String>) {
    let header = header.unwrap_or_else(|| {
        format!(
            "{} {}{} {{",
            component.kind.as_str(),
            component.name,
            parameters(&component.parameters)
        )
    });
    write_component_body(writer, component, &header, "};");
}

fn write_component_body(writer: &mut Writer, component: &Component, header: &str, footer: &str) {
    writer.line(header);
    writer.indent();
    for property in &component.properties {
        write_property_assignment(writer, property);
    }
    for child in &component.children {
        write_component_child(writer, child);
    }
    for instance in &component.instances {
        write_instance(writer, instance);
    }
    writer.dedent();
    writer.line(footer);
}

fn write_component_child(writer: &mut Writer, child: &ComponentChild) {
    match child {
        ComponentChild::Component(component) => write_component_def(writer, component, None),
        ComponentChild::Instance(instance) => write_instance(writer, instance),
        ComponentChild::Property(property) => write_property_assignment(writer, property),
        ComponentChild::Constraint(constraint) => write_constraint(writer, constraint),
        ComponentChild::Raw(raw) => writer.raw_indented_block(raw),
    }
}

fn write_instance(writer: &mut Writer, instance: &Instance) {
    let header = format!(
        "{} {} {{",
        instance.component.kind.as_str(),
        instance.component.name
    );

    let mut footer = String::from("} ");
    footer.push_str(&instance.name);
    if let Some(array) = &instance.array {
        footer.push_str(&array_expr(array));
    }
    if let Some(range) = &instance.range {
        footer.push_str(&bit_range(range));
    }
    if let Some(reset) = &instance.reset {
        footer.push_str(" = ");
        footer.push_str(&expr(reset));
    }
    if let Some(address) = &instance.address {
        footer.push_str(" @ ");
        footer.push_str(&expr(address));
    }
    if let Some(stride) = &instance.stride {
        footer.push_str(" += ");
        footer.push_str(&expr(stride));
    }
    footer.push(';');
    write_component_body(writer, &instance.component, &header, &footer);
    for property in &instance.instance_properties {
        writer.line(format!(
            "{}->{};",
            instance.name,
            property_assignment(property)
        ));
    }
}

fn write_property_assignment(writer: &mut Writer, property: &PropertyAssignment) {
    writer.line(format!("{};", property_assignment(property)));
}

fn property_assignment(property: &PropertyAssignment) -> String {
    match &property.value {
        Some(value) => format!("{} = {}", property.name, expr(value)),
        None => property.name.clone(),
    }
}

fn write_constraint(writer: &mut Writer, constraint: &Constraint) {
    match &constraint.name {
        Some(name) => writer.line(format!("constraint {name} {{")),
        None => writer.line("constraint {"),
    }
    writer.indent();
    writer.raw_indented_block(&constraint.body);
    writer.dedent();
    writer.line("};");
}

fn parameters(parameters: &[Parameter]) -> String {
    if parameters.is_empty() {
        return String::new();
    }
    let values = parameters
        .iter()
        .map(|parameter| match &parameter.default {
            Some(default) => format!("{} = {}", parameter.name, expr(default)),
            None => parameter.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("#({values})")
}

fn array_expr(array: &Array) -> String {
    array
        .dimensions
        .iter()
        .map(|dim| match dim {
            ArrayDimension::Count(count) => format!("[{}]", expr(count)),
            ArrayDimension::Range { left, right } => format!("[{}:{}]", expr(left), expr(right)),
        })
        .collect::<String>()
}

fn bit_range(range: &BitRange) -> String {
    match &range.lsb {
        Some(lsb) => format!("[{}:{}]", expr(&range.msb), expr(lsb)),
        None => format!("[{}]", expr(&range.msb)),
    }
}

fn expr(value: &Expression) -> String {
    match value {
        Expression::Identifier(value) | Expression::Number(value) | Expression::EnumRef(value) => {
            value.clone()
        }
        Expression::String(value) => format!("\"{}\"", value.replace('"', "\\\"")),
        Expression::Boolean(true) => "true".into(),
        Expression::Boolean(false) => "false".into(),
        Expression::Array(values) => {
            let values = values.iter().map(expr).collect::<Vec<_>>().join(", ");
            format!("'{{{values}}}")
        }
        Expression::Struct(fields) => {
            let fields = fields
                .iter()
                .map(|(name, value)| format!("{name}: {}", expr(value)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("'{{{fields}}}")
        }
        Expression::Raw(value) => value.clone(),
    }
}
