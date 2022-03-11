/// The protocol frame `Header` holds crucial data that helps validate data that
/// may be untrusted and uses a generic Type-Length-Value framing technique.
///
/// NOTE: All data is considered untrusted unless explicitly noted.
#[derive(Debug, PartialEq)]
pub struct Header {
    /// Sequence of pre-defined bytes that indicates the start of a frame.
    pub magic: u32,
    /// Randomly generated bytes that uniquely identify this frame.
    pub id: u32,
    /// Version number and whether the frame is a `Request` or `Response`.
    pub version: u8,
    /// Code that maps to a known instruction set, or command.
    pub opcode: u8,
    /// Contains the length of the message body that follows.
    pub length: u16,
}
