class {{ block.class_name }} extends uvm_reg_block;
    `uvm_object_utils({{ block.class_name }})
{%- for map in block.maps %}
{%- if !map.is_default %}
    uvm_reg_map {{ map.var_name }};
{%- endif %}
{%- endfor %}
{%- for mem in block.memories %}
    {{ mem.class_name }} {{ mem.var_name }};
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
      {{ map.var_name }} = create_map(
        .name({{ map.create_name }}),
        .base_addr(0),
        .n_bytes({{ map.n_bytes }}),
        .endian(UVM_LITTLE_ENDIAN),
        .byte_addressing({{ map.byte_addressing }})
      );
{%- endfor %}
{%- for mem in block.memories %}
{%- if mem.coverage_enabled %}
      {{ mem.var_name }} = new({{ mem.create_name }});
{%- else %}
      {{ mem.var_name }} = new(
        .name({{ mem.create_name }}),
        .size({{ mem.size_words }}),
        .n_bits({{ mem.width_bits }}),
        .access({{ mem.rights }}),
        .has_coverage(UVM_NO_COVERAGE)
      );
{%- endif %}
      {{ mem.var_name }}.configure(this, {{ mem.hdl_path_expr }});
      {{ mem.map_var_name }}.add_mem({{ mem.var_name }}, {{ mem.offset_literal }}, {{ mem.rights }});
{%- endfor %}
{%- for reg_file in block.reg_files %}
{{ reg_file.build_code }}
{%- endfor %}
{%- for inst in block.instances %}
      {{ inst.var_name }} = {{ inst.class_name }}::type_id::create({{ inst.create_name }});
      {{ inst.var_name }}.configure(this);
      {{ inst.var_name }}.build();
{%- for slice in inst.hdl_slices %}
      {{ inst.var_name }}.add_hdl_path_slice(
        .name({{ slice.path_expr }}),
        .offset({{ slice.offset }}),
        .size({{ slice.size }}),
        .first({{ slice.first }})
      );
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
