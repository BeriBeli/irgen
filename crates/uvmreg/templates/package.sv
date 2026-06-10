`ifndef {{ guard }}
`define {{ guard }}

{% if is_package %}
package {{ package_name }};

{% endif %}
{% if include_uvm %}
import uvm_pkg::*;
`include "uvm_macros.svh"

{% endif %}
{% for include in includes %}
`include "{{ include }}"

{% endfor %}
{% for reg in register_classes %}
{% include "register_class.sv" %}

{% endfor %}
{% for mem in memory_classes %}
{% include "memory_class.sv" %}

{% endfor %}
{% for reg_file_class in register_file_classes %}
{% include "register_file_class.sv" %}

{% endfor %}
{% for block in block_classes %}
{% include "block_class.sv" %}

{% endfor %}
{% if is_package %}
endpackage

{% endif %}
`endif
