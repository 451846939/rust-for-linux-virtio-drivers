# SPDX-License-Identifier: GPL-2.0

# KDIR ?= ../linux

# default:
# 	$(MAKE) -C $(KDIR) M=$$PWD


obj-$(CONFIG_RUST_NVME) += rust_blk.o