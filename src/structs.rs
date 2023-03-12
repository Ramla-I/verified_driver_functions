use prusti_contracts::*;

pub struct Volatile<T: Copy>{
    inner: T
}

impl<T: Copy> Volatile<T> {
    pub fn write(&mut self, val: T) {
        self.inner = val;
    }

    pub fn read(&self) -> T {
        self.inner
    }
}


/// Rx Status: Descriptor Done
pub const RX_STATUS_DD:                    u8 = 1 << 0;
/// Rx Status: End of Packet
pub const RX_STATUS_EOP:                   u8 = 1 << 1;

pub struct AdvancedRxDescriptor {
    /// Starting physcal address of the receive buffer for the packet.
    pub packet_buffer_address:  Volatile<u64>,
    /// Starting physcal address of the receive buffer for the header.
    /// This field will only be used if header splitting is enabled. 
    pub header_buffer_address:  Volatile<u64>,
}

impl AdvancedRxDescriptor {
    pub(crate) fn init (&mut self, packet_buffer_address: PhysicalAddress) {
        self.packet_buffer_address.write(packet_buffer_address.value() as u64);
        // set the header address to 0 because packet splitting is not supposed to be enabled in the 82599
        self.header_buffer_address.write(0);
    }

    #[inline(always)]
    pub(crate) fn set_packet_address(&mut self, packet_buffer_address: PhysicalAddress) {
        self.packet_buffer_address.write(packet_buffer_address.value() as u64);
    }

    #[inline(always)]
    pub(crate) fn reset_status(&mut self) {
        self.header_buffer_address.write(0);
    }

    #[inline(always)]
    pub fn descriptor_done(&self) -> bool{
        // (self.get_ext_status() & RX_STATUS_DD as u64) == RX_STATUS_DD as u64
        true
    }

    pub fn end_of_packet(&self) -> bool {
        // (self.get_ext_status() & RX_STATUS_EOP as u64) == RX_STATUS_EOP as u64    
        true    
    }

    pub fn length(&self) -> u64 {
        self.get_pkt_len() as u64
    }

    /// Write Back mode function for the Advanced Receive Descriptor.
    /// Status information indicates whether a descriptor has been used 
    /// and whether the buffer is the last one for a packet
    pub fn get_ext_status(&self) -> u64{
        // self.header_buffer_address.read() & 0xFFFFF //.get_bits(0..19) ?
        self.header_buffer_address.read()
    }
    
    
    /// Write Back mode function for the Advanced Receive Descriptor.
    /// Returns the number of bytes posted to the packet buffer
    pub fn get_pkt_len(&self) -> u64{
        // (self.header_buffer_address.read() & 0xFFFF) >>  32//.get_bits(32..47) 
        self.header_buffer_address.read()
    }
 
}

pub struct PacketBufferS {
    pub(crate) mp: MappedPages,
    pub(crate) phys_addr: PhysicalAddress,
    pub(crate) length: u16,
}

impl std::cmp::PartialEq for PacketBufferS {
    #[pure]
    fn eq(&self, other: &Self) -> bool {
        self.phys_addr.0 == other.phys_addr.0
    }
}

#[derive(Clone, Copy)]
pub struct PhysicalAddress(usize);
impl PhysicalAddress {
    #[pure]
    pub fn value(&self) -> usize {
        self.0
    }
}

impl std::cmp::PartialEq for PhysicalAddress {
    #[pure]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct Frame(usize);

pub struct AllocatedPages(RangeInclusive<usize>);

pub struct EntryFlags(u64);

pub struct MappedPages {
    page_table_p4: Frame,
    pages: AllocatedPages,
}


pub struct RxQueueRegisters {
    /// the ID of the rx queue that these registers control
    id: usize,
    /// We prevent the drop handler from dropping the `regs` because the backing memory is not in the heap,
    /// but in the stored mapped pages. The memory will be deallocated when the `backing_pages` are dropped.
    pub(crate) regs: Fragment<RegistersRx>
}

pub struct Fragment<T> {
    pub(crate) ptr: Box<T>,
}
pub struct RegistersRx {
    /// Receive Descriptor Base Address Low
    pub rdbal:                          Volatile<u32>,        // 0x1000

    /// Recive Descriptor Base Address High
    pub rdbah:                          Volatile<u32>,        // 0x1004

    /// Recive Descriptor Length
    pub rdlen:                          Volatile<u32>,        // 0x1008

    /// Rx DCA Control Register
    dca_rxctrl:                         Volatile<u32>,          // 0x100C

    /// Recive Descriptor Head
    pub rdh:                            Volatile<u32>,          // 0x1010

    /// Split Receive Control Registers
    srrctl:                             Volatile<u32>,          // 0x1014 //specify descriptor type

    /// Receive Descriptor Tail
    pub rdt:                            Volatile<u32>,          // 0x1018
    _padding1:                          [u8;12],                // 0x101C - 0x1027

    /// Receive Descriptor Control
    rxdctl:                             Volatile<u32>,          // 0x1028
    _padding2:                          [u8;20],                // 0x102C - 0x103F                                            
} // 64B

pub struct RangeInclusive<Idx: Clone + PartialOrd> {
    start: Idx,
    end: Idx
}

// impl<Idx: Clone + PartialOrd> RangeInclusive<Idx> {
//     #[ensures(result.start == start)]
//     #[ensures(result.end == end)]
//     pub(crate) const fn new(start: Idx, end: Idx) -> Self {
//         Self{start, end}
//     }

//     #[pure]
//     pub const fn start(&self) -> &Idx {
//         &self.start
//     }

//     #[pure]
//     pub const fn end(&self) -> &Idx {
//         &self.end
//     }

//     pub fn is_empty(&self) -> bool {
//         !(self.start <= self.end)
//     }

// }

pub struct AdvancedTxDescriptor {
    /// Starting physical address of the receive buffer for the packet.
    pub packet_buffer_address:  Volatile<u64>,
    /// Length of data buffer
    pub data_len: Volatile<u16>,
    /// A multi-part field:
    /// * `dtyp`: Descriptor Type, occupies bits `[7:4]`,
    /// * `mac`: options to apply LinkSec and time stamp, occupies bits `[3:2]`.
    pub dtyp_mac_rsv : Volatile<u8>,
    /// Command bits
    pub dcmd:  Volatile<u8>,
    /// A multi-part field:
    /// * `paylen`: the size in bytes of the data buffer in host memory.
    ///   not including the fields that the hardware adds), occupies bits `[31:14]`.
    /// * `popts`: options to offload checksum calculation, occupies bits `[13:8]`.
    /// * `sta`: status of the descriptor (whether it's in use or not), occupies bits `[3:0]`.
    pub paylen_popts_cc_idx_sta: Volatile<u32>,
}

impl AdvancedTxDescriptor {
    pub(crate) fn send(&mut self, transmit_buffer_addr: PhysicalAddress, transmit_buffer_length: u16) {
        // self.packet_buffer_address.write(transmit_buffer_addr.value() as u64);
        // self.data_len.write(transmit_buffer_length);
        // self.dtyp_mac_rsv.write(TX_DTYP_ADV);
        // self.paylen_popts_cc_idx_sta.write((transmit_buffer_length as u32) << TX_PAYLEN_SHIFT);
        // self.dcmd.write(TX_CMD_DEXT | TX_CMD_RS | TX_CMD_IFCS | TX_CMD_EOP);
    }

    pub fn desc_done(&self) -> bool {
        // (self.paylen_popts_cc_idx_sta.read() as u8 & TX_STATUS_DD) == TX_STATUS_DD
        true
    }

}

pub(crate) struct TxQueueRegisters {
    /// the ID of the tx queue that these registers control
    id: usize,
    /// We prevent the drop handler from dropping the `regs` because the backing memory is not in the heap,
    /// but in the stored mapped pages. The memory will be deallocated when the `backing_pages` are dropped.
    pub(crate)regs: Fragment<RegistersTx>
}

pub(crate) struct RegistersTx {
    /// Transmit Descriptor Base Address Low
    pub tdbal:                          Volatile<u32>,        // 0x6000

    /// Transmit Descriptor Base Address High
    pub tdbah:                          Volatile<u32>,        // 0x6004
    
    /// Transmit Descriptor Length    
    pub tdlen:                          Volatile<u32>,        // 0x6008

    /// Tx DCA Control Register
    dca_txctrl:                         Volatile<u32>,          // 0x600C

    /// Transmit Descriptor Head
    pub tdh:                            Volatile<u32>,          // 0x6010
    _padding0:                          [u8; 4],                // 0x6014 - 0x6017

    /// Transmit Descriptor Tail
    pub tdt:                            Volatile<u32>,          // 0x6018
    _padding1:                          [u8; 12],               // 0x601C - 0x6027

    /// Transmit Descriptor Control
    pub txdctl:                             Volatile<u32>,          // 0x6028
    _padding2:                          [u8; 12],               // 0x602C - 0x6037

    /// Transmit Descriptor Completion Write Back Address Low
    tdwbal:                             Volatile<u32>,          // 0x6038

    /// Transmit Descriptor Completion Write Back Address High
    tdwbah:                             Volatile<u32>,          // 0x603C
} // 64B