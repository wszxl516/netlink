use crate::{
    rtnl::traits::{Emitable, Parseable},
    DecodeError,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LinkStats {
    /// total packets received
    pub rx_packets: u32,
    /// total packets transmitted
    pub tx_packets: u32,
    /// total bytes received
    pub rx_bytes: u32,
    /// total bytes transmitted
    pub tx_bytes: u32,
    /// bad packets received
    pub rx_errors: u32,
    /// packet transmit problems
    pub tx_errors: u32,
    /// no space in linux buffers
    pub rx_dropped: u32,
    /// no space available in linux
    pub tx_dropped: u32,
    /// multicast packets received
    pub multicast: u32,
    pub collisions: u32,

    // detailed rx_errors
    pub rx_length_errors: u32,
    /// receiver ring buff overflow
    pub rx_over_errors: u32,
    /// received packets with crc error
    pub rx_crc_errors: u32,
    /// received frame alignment errors
    pub rx_frame_errors: u32,
    /// recv'r fifo overrun
    pub rx_fifo_errors: u32,
    /// receiver missed packet
    pub rx_missed_errors: u32,

    // detailed tx_errors
    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_fifo_errors: u32,
    pub tx_heartbeat_errors: u32,
    pub tx_window_errors: u32,

    // for cslip etc
    pub rx_compressed: u32,
    pub tx_compressed: u32,

    /// dropped, no handler found
    pub rx_nohandler: u32,
}

pub const LINK_STATS_LEN: usize = 96;

buffer!(LinkStatsBuffer, LINK_STATS_LEN);
fields!(LinkStatsBuffer {
    rx_packets: (u32, 0..4),
    tx_packets: (u32, 4..8),
    rx_bytes: (u32, 8..12),
    tx_bytes: (u32, 12..16),
    rx_errors: (u32, 16..20),
    tx_errors: (u32, 20..24),
    rx_dropped: (u32, 24..28),
    tx_dropped: (u32, 28..32),
    multicast: (u32, 32..36),
    collisions: (u32, 36..40),
    rx_length_errors: (u32, 40..44),
    rx_over_errors: (u32, 44..48),
    rx_crc_errors: (u32, 48..52),
    rx_frame_errors: (u32, 52..56),
    rx_fifo_errors: (u32, 56..60),
    rx_missed_errors: (u32, 60..64),
    tx_aborted_errors: (u32, 64..68),
    tx_carrier_errors: (u32, 68..72),
    tx_fifo_errors: (u32, 72..76),
    tx_heartbeat_errors: (u32, 76..80),
    tx_window_errors: (u32, 80..84),
    rx_compressed: (u32, 84..88),
    tx_compressed: (u32, 88..92),
    rx_nohandler: (u32, 92..96),
});

impl<T: AsRef<[u8]>> Parseable<LinkStats> for LinkStatsBuffer<T> {
    fn parse(&self) -> Result<LinkStats, DecodeError> {
        Ok(LinkStats {
            rx_packets: self.rx_packets(),
            tx_packets: self.tx_packets(),
            rx_bytes: self.rx_bytes(),
            tx_bytes: self.tx_bytes(),
            rx_errors: self.rx_errors(),
            tx_errors: self.tx_errors(),
            rx_dropped: self.rx_dropped(),
            tx_dropped: self.tx_dropped(),
            multicast: self.multicast(),
            collisions: self.collisions(),
            rx_length_errors: self.rx_length_errors(),
            rx_over_errors: self.rx_over_errors(),
            rx_crc_errors: self.rx_crc_errors(),
            rx_frame_errors: self.rx_frame_errors(),
            rx_fifo_errors: self.rx_fifo_errors(),
            rx_missed_errors: self.rx_missed_errors(),
            tx_aborted_errors: self.tx_aborted_errors(),
            tx_carrier_errors: self.tx_carrier_errors(),
            tx_fifo_errors: self.tx_fifo_errors(),
            tx_heartbeat_errors: self.tx_heartbeat_errors(),
            tx_window_errors: self.tx_window_errors(),
            rx_compressed: self.rx_compressed(),
            tx_compressed: self.tx_compressed(),
            rx_nohandler: self.rx_nohandler(),
        })
    }
}

impl Emitable for LinkStats {
    fn buffer_len(&self) -> usize {
        LINK_STATS_LEN
    }

    fn emit(&self, buffer: &mut [u8]) {
        let mut buffer = LinkStatsBuffer::new(buffer);
        buffer.set_rx_packets(self.rx_packets);
        buffer.set_tx_packets(self.tx_packets);
        buffer.set_rx_bytes(self.rx_bytes);
        buffer.set_tx_bytes(self.tx_bytes);
        buffer.set_rx_errors(self.rx_errors);
        buffer.set_tx_errors(self.tx_errors);
        buffer.set_rx_dropped(self.rx_dropped);
        buffer.set_tx_dropped(self.tx_dropped);
        buffer.set_multicast(self.multicast);
        buffer.set_collisions(self.collisions);
        buffer.set_rx_length_errors(self.rx_length_errors);
        buffer.set_rx_over_errors(self.rx_over_errors);
        buffer.set_rx_crc_errors(self.rx_crc_errors);
        buffer.set_rx_frame_errors(self.rx_frame_errors);
        buffer.set_rx_fifo_errors(self.rx_fifo_errors);
        buffer.set_rx_missed_errors(self.rx_missed_errors);
        buffer.set_tx_aborted_errors(self.tx_aborted_errors);
        buffer.set_tx_carrier_errors(self.tx_carrier_errors);
        buffer.set_tx_fifo_errors(self.tx_fifo_errors);
        buffer.set_tx_heartbeat_errors(self.tx_heartbeat_errors);
        buffer.set_tx_window_errors(self.tx_window_errors);
        buffer.set_rx_compressed(self.rx_compressed);
        buffer.set_tx_compressed(self.tx_compressed);
        buffer.set_rx_nohandler(self.rx_nohandler);
    }
}
