# Pipelight-init nixos module

A set of systemd-unit that execute only if a "INIT" disk is detected.
The **pipelines** present in this disk are executed on machine boot.

Only pipelines(hooks) with the following special names are executed:

- init_net_pre (starts before network initialization)
- init_net_post (starts after network initialization)
