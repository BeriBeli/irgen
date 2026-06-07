class {{ reg.class_name }} extends uvm_reg;
    `uvm_object_utils({{ reg.class_name }})
{%- for field in reg.fields %}
{%- if field.has_enum_values %}
    typedef enum bit [{{ field.enum_msb }}:0] {
{%- for value in field.enum_values %}
      {{ value.name }} = {{ value.literal }}{% if !loop.last %},{% endif %}
{%- endfor %}
    } {{ field.enum_type_name }};
{%- endif %}
{%- endfor %}
{%- for field in reg.fields %}
    rand uvm_reg_field {{ field.var_name }};
{%- endfor %}
{%- if reg.coverage_enabled %}
    local uvm_reg_data_t m_data;
    local uvm_reg_data_t m_be;
    local bit m_is_read;

    covergroup cg_bits();
      option.per_instance = 1;
      option.name = get_name();
{%- for field in reg.fields %}
      {{ field.var_name }}_bits: coverpoint {m_data[{{ field.msb }}:{{ field.lsb }}], m_is_read} iff (m_be);
{%- endfor %}
    endgroup
{%- endif %}

    function new(string name = "{{ reg.default_name }}");
      super.new(name, {{ reg.size_bits }}, {{ reg.coverage_model }});
{%- if reg.coverage_enabled %}
      add_coverage(build_coverage(UVM_CVR_REG_BITS));
      if (has_coverage(UVM_CVR_REG_BITS)) begin
        cg_bits = new();
      end
{%- endif %}
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
{%- if reg.coverage_enabled %}

`ifdef UVM_REG_PROTECTED_SAMPLE
    protected virtual function void sample(uvm_reg_data_t data,
                                           uvm_reg_data_t byte_en,
                                           bit is_read,
                                           uvm_reg_map map);
`else
    virtual function void sample(uvm_reg_data_t data,
                                 uvm_reg_data_t byte_en,
                                 bit is_read,
                                 uvm_reg_map map);
`endif
      if (get_coverage(UVM_CVR_REG_BITS)) begin
        m_data = data;
        m_be = byte_en;
        m_is_read = is_read;
        cg_bits.sample();
      end
    endfunction
{%- endif %}
endclass
