use core::ptr::NonNull;

use log::{info, warn, debug};
use virtio_drivers::{
    device::blk::VirtIOBlk,
    transport::{
        self,
        mmio::{MmioTransport, VirtIOHeader},
        Transport,
    },
};

pub mod blk;

pub fn virtio_blk_init() {
    let config_space = init_config();
    let state = init_state();
    let transport = init_transport(config_space, state);
    logger::init(LevelFilter::Debug).unwrap();
    info!("virtio-drivers example started.");
    debug!(
        "x0={:#018x}, x1={:#018x}, x2={:#018x}, x3={:#018x}",
        x0, x1, x2, x3
    );

    // Safe because `HEAP` is only used here and `entry` is only called once.
    unsafe {
        // Give the allocator some memory to allocate.
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_mut_ptr() as usize, HEAP.len());
    }

    info!("Loading FDT from {:#018x}", x0);
    // Safe because the pointer is a valid pointer to unaliased memory.
    let fdt = unsafe { Fdt::from_ptr(x0 as *const u8).unwrap() };

    for node in fdt.all_nodes() {
        // Dump information about the node for debugging.
        trace!(
            "{}: {:?}",
            node.name,
            node.compatible().map(Compatible::first),
        );
        if let Some(reg) = node.reg() {
            for range in reg {
                trace!(
                    "  {:#018x?}, length {:?}",
                    range.starting_address,
                    range.size
                );
            }
        }

    // Check whether it is a VirtIO MMIO device.
    if let (Some(compatible), Some(region)) =
        (node.compatible(), node.reg().and_then(|mut reg| reg.next()))
    {
        if compatible.all().any(|s| s == "virtio,mmio")
            && region.size.unwrap_or(0) > size_of::<VirtIOHeader>()
        {
            debug!("Found VirtIO MMIO device at {:?}", region);

            let header = NonNull::new(region.starting_address as *mut VirtIOHeader).unwrap();
            match unsafe { MmioTransport::new(header) } {
                Err(e) => warn!("Error creating VirtIO MMIO transport: {}", e),
                Ok(transport) => {
                    info!(
                "Detected virtio MMIO device with vendor id {:#X}, device type {:?}, version {:?}",
                transport.vendor_id(),
                transport.device_type(),
                transport.version(),
            );
                    virtio_device(transport);
                }
            }
        }
    }
    VirtIOBlk::new(transport)
}

fn virtio_device(transport: MmioTransport) -> _ {
    todo!()
}

fn init_transport(config_space: _, state: !) -> impl Transport {
    todo!()
}

fn init_state() -> ! {
    todo!()
}

fn init_config() -> virtio_drivers::device::blk::BlkConfig {
    todo!()
}
