class {{ mem.class_name }} extends uvm_mem;
    `uvm_object_utils({{ mem.class_name }})
    local uvm_reg_addr_t m_offset;
    local bit m_is_read;

    covergroup cg_addr();
      option.per_instance = 1;
      option.name = get_name();
      offset: coverpoint m_offset;
      access: coverpoint m_is_read;
    endgroup

    function new(string name = "{{ mem.default_name }}");
      super.new(
        .name(name),
        .size({{ mem.size_words }}),
        .n_bits({{ mem.width_bits }}),
        .access({{ mem.rights }}),
        .has_coverage({{ mem.coverage_model }})
      );
      add_coverage(build_coverage(UVM_CVR_ADDR_MAP));
      if (has_coverage(UVM_CVR_ADDR_MAP)) begin
        cg_addr = new();
      end
    endfunction

`ifdef UVM_MEM_PROTECTED_SAMPLE
    protected virtual function void sample(uvm_reg_addr_t offset,
                                           bit is_read,
                                           uvm_reg_map map);
`else
    virtual function void sample(uvm_reg_addr_t offset,
                                 bit is_read,
                                 uvm_reg_map map);
`endif
      if (get_coverage(UVM_CVR_ADDR_MAP)) begin
        m_offset = offset;
        m_is_read = is_read;
        cg_addr.sample();
      end
    endfunction
endclass
