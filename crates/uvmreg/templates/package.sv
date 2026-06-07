`ifndef {{ guard }}
`define {{ guard }}

import uvm_pkg::*;
`include "uvm_macros.svh"

{% for reg in register_classes %}
{% include "register_class.sv" %}

{% endfor %}
{% for reg_file_class in register_file_classes %}
{% include "register_file_class.sv" %}

{% endfor %}
{% for block in block_classes %}
{% include "block_class.sv" %}

{% endfor %}
`endif
