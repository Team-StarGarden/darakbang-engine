use crate::packet_kind_child;

mod goto;

packet_kind_child!(CommonPacketKind, "common", { Goto("goto") });
