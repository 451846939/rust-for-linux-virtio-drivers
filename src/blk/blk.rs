use crate::drivers::block::VirtIOBlock;


pub struct VirtIOBlock {
    virtio_blk: UPIntrFreeCell<VirtIOBlk<'static, VirtioHal>>,
    condvars: BTreeMap<u16, Condvar>,
}

impl BlockDevice for VirtIOBlock{

}