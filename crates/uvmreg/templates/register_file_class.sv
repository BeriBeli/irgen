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
