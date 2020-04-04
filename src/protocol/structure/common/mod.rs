use packet_macro::packet;

mod goto;

#[packet(namespace = "common")]
pub enum CommonPacket {
    #[packet(id = "goto")]
    Goto {
        request: goto::GotoPacketRequest,
        response: goto::GotoPacketResponse,
    },
}
