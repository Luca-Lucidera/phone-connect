//! Logica condivisa di Phone Connect: protocollo, pairing, crittografia e
//! configurazione. La usano sia il daemon (`phc`) sia la UI desktop.

/// Versione del protocollo scambiato tra PC e telefono. Va incrementata a
/// ogni modifica incompatibile dei messaggi.
pub const PROTOCOL_VERSION: u32 = 1;
