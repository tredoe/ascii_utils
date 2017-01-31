// Copyright 2016  Jonas mg
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Defines the ASCII control characters.

/// Null char
pub const NUL: u8 = 0x0;
pub const NUL_AS_CHAR: char = '\u{0}';

/// Start of Heading
pub const SOH: u8 = 0x1;
pub const SOH_AS_CHAR: char = '\u{1}';

/// Start of Text
pub const STX: u8 = 0x2;
pub const STX_AS_CHAR: char = '\u{2}';

/// End of Text
pub const ETX: u8 = 0x3;
pub const ETX_AS_CHAR: char = '\u{3}';

/// End of Transmission
pub const EOT: u8 = 0x4;
pub const EOT_AS_CHAR: char = '\u{4}';

/// Enquiry
pub const ENQ: u8 = 0x5;
pub const ENQ_AS_CHAR: char = '\u{5}';

/// Acknowledgment
pub const ACK: u8 = 0x6;
pub const ACK_AS_CHAR: char = '\u{6}';

/// Bell
pub const BEL: u8 = 0x7;
pub const BEL_AS_CHAR: char = '\u{7}';

/// Back Space
pub const BS: u8 = 0x8;
pub const BS_AS_CHAR: char = '\u{8}';

/// Horizontal Tab
pub const HT: u8 = 0x9;
pub const HT_AS_CHAR: char = '\u{9}';

/// Line Feed
pub const LF: u8 = 0xA;
pub const LF_AS_CHAR: char = '\u{A}';

/// Vertical Tab
pub const VT: u8 = 0xB;
pub const VT_AS_CHAR: char = '\u{B}';

/// Form Feed
pub const FF: u8 = 0xC;
pub const FF_AS_CHAR: char = '\u{C}';

/// Carriage Return
pub const CR: u8 = 0xD;
pub const CR_AS_CHAR: char = '\u{D}';

/// Shift Out / X-On
pub const SO: u8 = 0xE;
pub const SO_AS_CHAR: char = '\u{E}';

/// Shift In / X-Off
pub const SI: u8 = 0xF;
pub const SI_AS_CHAR: char = '\u{F}';

/// Data Line Escape
pub const DLE: u8 = 0x10;
pub const DLE_AS_CHAR: char = '\u{10}';

/// Device Control 1 (oft. XON)
pub const DC1: u8 = 0x11;
pub const DC1_AS_CHAR: char = '\u{11}';

/// Device Control 2
pub const DC2: u8 = 0x12;
pub const DC2_AS_CHAR: char = '\u{12}';

/// Device Control 3 (oft. XOFF)
pub const DC3: u8 = 0x13;
pub const DC3_AS_CHAR: char = '\u{13}';

/// Device Control 4
pub const DC4: u8 = 0x14;
pub const DC4_AS_CHAR: char = '\u{14}';

/// Negative Acknowledgement
pub const NAK: u8 = 0x15;
pub const NAK_AS_CHAR: char = '\u{15}';

/// Synchronous Idle
pub const SYN: u8 = 0x16;
pub const SYN_AS_CHAR: char = '\u{16}';

/// End of Transmit Block
pub const ETB: u8 = 0x17;
pub const ETB_AS_CHAR: char = '\u{17}';

/// Cancel
pub const CAN: u8 = 0x18;
pub const CAN_AS_CHAR: char = '\u{18}';

/// End of Medium
pub const EM: u8 = 0x19;
pub const EM_AS_CHAR: char = '\u{19}';

/// Substitute
pub const SUB: u8 = 0x1A;
pub const SUB_AS_CHAR: char = '\u{1A}';

/// Escape
pub const ESC: u8 = 0x1B;
pub const ESC_AS_CHAR: char = '\u{1B}';

/// File Separator
pub const FS: u8 = 0x1C;
pub const FS_AS_CHAR: char = '\u{1C}';

/// Group Separator
pub const GS: u8 = 0x1D;
pub const GS_AS_CHAR: char = '\u{1D}';

/// Record Separator
pub const RS: u8 = 0x1E;
pub const RS_AS_CHAR: char = '\u{1E}';

/// Unit Separator
pub const US: u8 = 0x1F;
pub const US_AS_CHAR: char = '\u{1F}';

/// Delete
pub const DELETE: u8 = 0x7F;
pub const DELETE_AS_CHAR: char = '\u{7F}';

// * * *

/// Space
pub const SPACE: u8 = 0x20;
pub const SPACE_AS_CHAR: char = '\u{20}';
