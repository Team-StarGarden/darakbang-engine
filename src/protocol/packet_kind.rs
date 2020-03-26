use crate::protocol::structure::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct InvalidPacketKindError;

impl fmt::Display for InvalidPacketKindError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InvalidPacketKindError")
    }
}

macro_rules! packet_kind_root {
    ($($outer_name:ident($inner_name:ty)),+) => {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
        #[serde(try_from = "String")]
        #[serde(into = "String")]
        pub enum PacketKind {
            $($outer_name($inner_name)),+
        }

        impl std::convert::TryFrom<String> for PacketKind {
            type Error = crate::protocol::packet_kind::InvalidPacketKindError;
            fn try_from(data: String) -> Result<Self, Self::Error> {
                let split: Vec<_> = data.splitn(2, ':').collect();
                match split.get(1) {
                    None => Self::try_from(format!("common:{}", split[0])),
                    Some(id) => {
                        match split[0] {
                            $(<$inner_name>::NAMESPACE => <$inner_name>::try_from(id.to_string()).map(PacketKind::$outer_name),)+
                            _ => Err(InvalidPacketKindError),
                        }
                    },
                }
            }
        }

        impl std::convert::Into<String> for PacketKind {
            fn into(self) -> String {
                match self {
                    $(PacketKind::$outer_name(inner) => inner.into(),)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! packet_kind_child {
    ($enum_name:ident, $namespace:literal, { $($outer_name:ident($id:literal)),+ }) => {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
        #[serde(try_from = "String")]
        #[serde(into = "String")]
        pub enum $enum_name {
            $($outer_name),+
        }

        impl $enum_name {
            pub const NAMESPACE: &'static str = $namespace;
        }

        impl std::convert::TryFrom<String> for $enum_name {
            type Error = crate::protocol::packet_kind::InvalidPacketKindError;
            fn try_from(data: String) -> Result<Self, Self::Error> {
                match data.as_ref() {
                    $($id => Ok(<$enum_name>::$outer_name),)+
                    _ => Err(crate::protocol::packet_kind::InvalidPacketKindError)
                }
            }
        }

        impl std::convert::Into<String> for $enum_name {
            fn into(self) -> String {
                match self {
                    $(<$enum_name>::$outer_name => format!("{}:{}", <$enum_name>::NAMESPACE, $id),)+
                }
            }
        }
    };
}

packet_kind_root! {
    Common(CommonPacketKind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn serialization() {
        assert_eq!(
            serde_json::to_string(&PacketKind::Common(CommonPacketKind::Goto)).unwrap(),
            "\"common:goto\"".to_string()
        );
    }

    #[test]
    fn deserialization() {
        assert_eq!(
            serde_json::from_str::<PacketKind>("\"common:goto\"").unwrap(),
            PacketKind::Common(CommonPacketKind::Goto)
        );
    }

    #[test]
    #[should_panic]
    fn deserialization_not_exists() {
        serde_json::from_str::<PacketKind>("\"it_should_not_exists\"").unwrap();
    }
}
