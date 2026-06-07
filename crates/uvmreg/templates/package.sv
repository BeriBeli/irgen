`ifndef {{ guard }}
`define {{ guard }}

import uvm_pkg::*;
`include "uvm_macros.svh"

{%- for reg in register_classes %}
class {{ reg.class_name }} extends uvm_reg;
    `uvm_object_utils({{ reg.class_name }})
{%- for param in reg.metadata_params %}
    localparam {{ param.type_expr }} {{ param.name }} = {{ param.value_expr }};
{%- endfor %}
{%- for field in reg.fields %}
{%- if field.has_enum_values %}
    typedef enum bit [{{ field.enum_msb }}:0] {
{%- for value in field.enum_values %}
      {{ value.name }} = {{ value.literal }}{% if value.has_usage %} /* {{ value.usage }} */{% endif %}{% if !loop.last %},{% endif %}
{%- endfor %}
    } {{ field.enum_type_name }};
{%- endif %}
{%- if field.has_constraint_params %}
{%- for param in field.constraint_params %}
    localparam {{ param.type_expr }} {{ param.name }} = {{ param.value_expr }};
{%- endfor %}
{%- endif %}
{%- if field.has_policy_params %}
{%- for param in field.policy_params %}
    localparam {{ param.type_expr }} {{ param.name }} = {{ param.value_expr }};
{%- endfor %}
{%- endif %}
{%- endfor %}
{%- for field in reg.fields %}
    rand uvm_reg_field {{ field.var_name }};
{%- endfor %}

    function new(string name = "{{ reg.default_name }}");
      super.new(name, {{ reg.size_bits }}, UVM_NO_COVERAGE);
    endfunction

    virtual function void build();
{%- for field in reg.fields %}
      {{ field.var_name }} = uvm_reg_field::type_id::create({{ field.create_name }});
      {{ field.var_name }}.configure(this, {{ field.width }}, {{ field.lsb }}, {{ field.access }}, {{ field.volatile }}, {{ field.reset_literal }}, {{ field.has_reset }}, {{ field.is_rand }}, 1);
{%- for reset in field.extra_resets %}
      {{ field.var_name }}.set_reset({{ reset.value_literal }}, {{ reset.kind }});
{%- endfor %}
{%- endfor %}
    endfunction
endclass

{%- endfor %}
{%- for reg_file_class in register_file_classes %}
class {{ reg_file_class.class_name }} extends uvm_reg_file;
    `uvm_object_utils({{ reg_file_class.class_name }})
{%- for declaration in reg_file_class.declarations %}
{{ declaration }}
{%- endfor %}

    function new(string name = "{{ reg_file_class.default_name }}");
      super.new(name);
    endfunction

    virtual function void build();
{{ reg_file_class.build_code }}
    endfunction

    virtual function void map(uvm_reg_map mp, uvm_reg_addr_t offset);
{{ reg_file_class.map_code }}
    endfunction
endclass

{%- endfor %}
{%- for block in block_classes %}
class {{ block.class_name }} extends uvm_reg_block;
    `uvm_object_utils({{ block.class_name }})
{%- for param in block.metadata_params %}
    localparam {{ param.type_expr }} {{ param.name }} = {{ param.value_expr }};
{%- endfor %}
{%- for map in block.maps %}
{%- if !map.is_default %}
    uvm_reg_map {{ map.var_name }};
{%- endif %}
{%- endfor %}
{%- for mem in block.memories %}
    uvm_mem {{ mem.var_name }};
{%- endfor %}
{%- for reg_file in block.reg_files %}
    {{ reg_file.class_name }} {{ reg_file.var_name }}{{ reg_file.declaration_suffix }};
{%- endfor %}
{%- for inst in block.instances %}
    rand {{ inst.class_name }} {{ inst.var_name }};
{%- endfor %}
{%- for array in block.arrays %}
    rand {{ array.class_name }} {{ array.var_name }}{{ array.declaration_suffix }};
{%- endfor %}
{%- for child in block.child_blocks %}
    rand {{ child.class_name }} {{ child.var_name }};
{%- endfor %}
{%- for submap in block.submaps %}
    rand {{ submap.class_name }} {{ submap.var_name }};
{%- endfor %}

    function new(string name = "{{ block.default_name }}");
      super.new(name, UVM_NO_COVERAGE);
    endfunction

    virtual function void build();
{%- for map in block.maps %}
      {{ map.var_name }} = create_map({{ map.create_name }}, 0, {{ map.n_bytes }}, UVM_LITTLE_ENDIAN, {{ map.byte_addressing }});
{%- endfor %}
{%- for mem in block.memories %}
      {{ mem.var_name }} = new({{ mem.create_name }}, {{ mem.size_words }}, {{ mem.width_bits }}, {{ mem.rights }}, UVM_NO_COVERAGE);
      {{ mem.var_name }}.configure(this, {{ mem.hdl_path_expr }});
      {{ mem.map_var_name }}.add_mem({{ mem.var_name }}, {{ mem.offset_literal }}, {{ mem.rights }});
{%- endfor %}
{%- for reg_file in block.reg_files %}
{{ reg_file.build_code }}
{%- endfor %}
{%- for inst in block.instances %}
      {{ inst.var_name }} = {{ inst.class_name }}::type_id::create({{ inst.create_name }});
      {{ inst.var_name }}.configure({{ inst.configure_args }});
      {{ inst.var_name }}.build();
{%- for slice in inst.hdl_slices %}
      {{ inst.var_name }}.add_hdl_path_slice({{ slice.path_expr }}, {{ slice.offset }}, {{ slice.size }}, {{ slice.first }});
{%- endfor %}
      {{ inst.map_var_name }}.add_reg({{ inst.var_name }}, {{ inst.offset_literal }}, {{ inst.rights }});
{%- endfor %}
{%- for array in block.arrays %}
{{ array.build_code }}
{%- endfor %}
{%- for child in block.child_blocks %}
      {{ child.var_name }} = {{ child.class_name }}::type_id::create({{ child.create_name }});
      {{ child.var_name }}.configure(this, {{ child.hdl_path_expr }});
      {{ child.var_name }}.build();
      {{ child.map_var_name }}.add_submap({{ child.var_name }}.default_map, {{ child.offset_literal }});
{%- endfor %}
{%- for submap in block.submaps %}
      {{ submap.var_name }} = {{ submap.class_name }}::type_id::create({{ submap.create_name }});
      {{ submap.var_name }}.configure(this);
      {{ submap.var_name }}.build();
      {{ submap.map_var_name }}.add_submap({{ submap.var_name }}.default_map, {{ submap.offset_literal }});
{%- endfor %}
      lock_model();
    endfunction
endclass
{%- endfor %}

{%- for alias in alias_classes %}
class {{ alias.class_name }} extends {{ alias.base_class_name }};
    `uvm_object_utils({{ alias.class_name }})

    function new(string name = "{{ alias.default_name }}");
      super.new(name);
    endfunction
endclass
{%- endfor %}

`endif
