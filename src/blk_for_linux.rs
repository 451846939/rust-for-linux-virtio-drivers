#![allow(unused)]
#![feature(type_alias_impl_trait)]
use core::convert::AsRef;
use core::iter::{IntoIterator, Iterator};
use core::result::Result::Ok;
use core::slice::from_raw_parts_mut;
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use kernel::block::mq::{GenDisk, Operations, Request, TagSet, self};
use kernel::{define_virtio_id_table, PointerWrapper};
use kernel::sync::{Mutex, Arc};
use kernel::virtio::VirtioDevice;
use kernel::workqueue::Work;
use kernel::{prelude::*, virtio};
use kernel::Error;
use kernel::init::PinInit;
use kernel::types::ForeignOwnable;
// use kernel::sync::p;
// use kernel::sync::spinlock;
use kernel::{bindings, c_str, driver};

module! {
    type: RustVirtioBLK,
    name: "rust_virtio_BLK",
    author: "hua shao",
    description: "Rust virtio blk device driver",
    license: "GPL",
}

struct RustVirtioBLK {}

impl virtio::Driver for RustVirtioBLK {

    fn probe(vdev: Arc<VirtioDevice>) -> Result<()> {

        // GenDisk::try_new(tagset, queue_data)

        // todo impl request_queue
        // let disk=GenDisk::try_new(Arc::try_new(TagSet::try_new(nr_hw_queues, tagset_data, num_tags, num_maps)), queue_data);
        


        let vblk = Box::try_new(VirtioBlk::<VirtioMqOps> {
            vdev: None,
            disk: None,
            config_work: unsafe { Work::new() },
            index: 0,
            num_vqs: 0,
            io_queues: Vec::try_with_capacity( 3)?,
            vqs: None,
        })
        .unwrap();
        //todo init  request_queue
        //todo ida_simple_get

        if vdev.as_ref().config_get() == None {
            return Err(EINVAL);
        }

        // let lock = unsafe { Mutex::new(vblk) };
        
        vblk.vdev = Some(vdev);

        // init_vq
        // 先默认3个
        let num_vqs = 3;
        let num_poll_vqs = num_vqs - 1;
        vblk.io_queues[0] = num_vqs - num_poll_vqs;
        vblk.io_queues[1] = 0;
        vblk.io_queues[2] = num_poll_vqs;
        let mut vqs = Vec::try_with_capacity(num_vqs as usize).unwrap();
        // todo vqs init
        vblk.vqs = Some(vqs);
        vblk.num_vqs = num_vqs;

        // vblk.disk=Some(disk);

        Ok(())
    }

    fn remove(_data: &mut VirtioDevice) {
        todo!()
    }

    define_virtio_id_table! {u32, [
        (virtio::DeviceId::new(VIRTIO_ID_BLOCK, VIRTIO_DEV_ANY_ID),None),
    ]}
}

pub const VIRTIO_ID_BLOCK: u32 = 2;
pub const VIRTIO_DEV_ANY_ID: u32 = 0xffffffff;
impl kernel::Module for RustVirtioBLK {
    /*
    1. 初始化过程：（驱动程序执行）

    1.1 virtio设备驱动在对设备进行初始化时，会申请virtqueue（包括描述符表、可用环、已用环）的内存空间；

    1.2 并把virtqueue中的描述符、可用环、已用环三部分的物理地址分别写入到virtio设备中对应的控制寄存器（即设备绑定的特定内存地址）中。至此，设备驱动和设备就共享了整个virtqueue的内存空间。

    2. I/O请求过程：（驱动程序执行）

    2.1 设备驱动在发出I/O请求时，首先把I/O请求的命令/数据等放到一个或多个buffer中；

    2.2 然后在描述符表中分配新的描述符（或描述符链）来指向这些buffer；

    2.3 再把描述符（或描述符链的首描述符）的索引值写入到可用环中，更新可用环的idx指针；

    2.4 驱动程序通过 kick 机制（即写virtio设备中特定的通知控制寄存器）来通知设备有新请求；

    3. I/O完成过程：（设备执行）

    3.1 virtio设备通过 kick 机制（知道有新的I/O请求，通过访问可用环的idx指针，解析出I/O请求；

    3.2 根据I/O请求内容完成I/O请求，并把I/O操作的结果放到I/O请求中相应的buffer中；

    3.3 再把描述符（或描述符链的首描述符）的索引值写入到已用环中，更新已用环的idx指针；

    3.4 设备通过再通过中断机制来通知设备驱动程序有I/O操作完成；

    4. I/O后处理过程：（驱动程序执行）

    4.1 设备驱动程序读取已用环的idx信息，读取已用环中的描述符索引，获得I/O操作完成信息。
     */
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust_virtio_BLK device driver (init)\n");

        let dev = driver::Registration::<virtio::Adapter<RustVirtioBLK>>::new_pinned(name, module)?;
        Ok(RustVirtioBLK {})
    }
}

pub struct VirtioBlk<T: Operations> {
    pub vdev: Option<Arc<VirtioDevice>>,
    pub disk: Option<GenDisk<T>>,
    pub config_work: Work,
    pub index: u32,
    pub num_vqs: u32,
    pub io_queues: Vec<bindings::hctx_type>,
    pub vqs: Option<Vec<VirtblkVQ>>,
}

pub struct VirtblkVQ {
    // todo
}

pub struct VirtioMqOps;

pub struct VirtioRequest{

}


pub struct VirtioNamespace{

}

pub struct VirtioQueue{

}

impl VirtioQueue {
    
}
pub struct VirtioData{

}

// unsafe impl<T, E> PinInit<VirtioBlk<T>,E> for VirtioBlk<T> {
//     unsafe fn __pinned_init(self, slot: *mut VirtioBlk<T>) -> Result<()> {
//         unsafe { self.__init(slot) }
//     }
// }
type RequestDataInit=impl PinInit<VirtioBlk<VirtioMqOps>>;

type DeviceData = kernel::device::Data<(), VirtioRequest, VirtioData>;


#[kernel::macros::vtable]
impl Operations for VirtioMqOps {

    #[doc = " Data associated with a request. This data is located next to the request"]
    #[doc = " structure."]
    type RequestData=VirtioBlk<Self>;


    type RequestDataInit=impl PinInit<RequestData>;

    #[doc = " Data associated with the `struct request_queue` that is allocated for"]
    #[doc = " the `GenDisk` associated with this `Operations` implementation."]
    type QueueData=Box<VirtioNamespace>;

    #[doc = " Data associated with a dispatch queue. This is stored as a pointer in"]
    #[doc = " `struct blk_mq_hw_ctx`."]
    type HwData=Arc<VirtioQueue>;

    #[doc = " Data associated with a tag set. This is stored as a pointer in `struct"]
    #[doc = " blk_mq_tag_set`."]
    type TagSetData=Arc<DeviceData>;

    #[doc = " Called by the kernel to get an initializer for a `Pin<&mut RequestData>`."]
    fn new_request_data(
        tagset_data: <Self::TagSetData as ForeignOwnable>::Borrowed<'_>,
    ) -> Self::RequestDataInit {
        todo!()
    }

    #[doc = " Called by the kernel to queue a request with the driver. If `is_last` is"]
    #[doc = " `false`, the driver is allowed to defer commiting the request."]
    fn queue_rq(
        hw_data: <Self::HwData as ForeignOwnable>::Borrowed<'_>,
        queue_data: <Self::QueueData as ForeignOwnable>::Borrowed<'_>,
        rq: Request<Self>,
        is_last: bool,
    ) -> Result {
        todo!()
    }

    #[doc = " Called by the kernel to indicate that queued requests should be submitted"]
    fn commit_rqs(
        hw_data: <Self::HwData as ForeignOwnable>::Borrowed<'_>,
        queue_data: <Self::QueueData as ForeignOwnable>::Borrowed<'_>,
    ) {
        todo!()
    }

    #[doc = " Called by the kernel when the request is completed"]
    fn complete(_rq: Request<Self>) {
        todo!()
    }

    #[doc = " Called by the kernel to allocate and initialize a driver specific hardware context data"]
    fn init_hctx(
        tagset_data: <Self::TagSetData as ForeignOwnable>::Borrowed<'_>,
        hctx_idx: u32,
    ) -> Result<Self::HwData> {
        todo!()
    }
}
