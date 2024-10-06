#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ByteOrderMark {
    Le,
    Be,
    NotPresent,
}

impl ByteOrderMark {
    pub(crate) fn is_present(&self) -> bool {
        *self != ByteOrderMark::NotPresent
    }

    #[allow(dead_code)]
    pub(crate) fn is_le(&self, default: bool) -> bool {
        match self {
            ByteOrderMark::Le => true,
            ByteOrderMark::Be => false,
            ByteOrderMark::NotPresent => default,
        }
    }
}

#[allow(dead_code)]
pub(crate) trait ByteOrderMarkExt {
    fn get_utf8_bom(&self) -> ByteOrderMark;
    fn get_utf16_bom(&self) -> ByteOrderMark;
    fn get_utf32_bom(&self) -> ByteOrderMark;
}

pub(crate) const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
pub(crate) const UTF16_LE_BOM: [u8; 2] = [0xFF, 0xFE];
pub(crate) const UTF16_BE_BOM: [u8; 2] = [0xFE, 0xFF];
pub(crate) const UTF32_LE_BOM: [u8; 4] = [0xFF, 0xFE, 0, 0];
pub(crate) const UTF32_BE_BOM: [u8; 4] = [0, 0, 0xFE, 0xFF];

impl ByteOrderMarkExt for [u8] {
    fn get_utf8_bom(&self) -> ByteOrderMark {
        if self.get(0..3) == Some(&UTF8_BOM) {
            ByteOrderMark::Le
        } else {
            ByteOrderMark::NotPresent
        }
    }

    fn get_utf16_bom(&self) -> ByteOrderMark {
        if self.get(0..2) == Some(&UTF16_LE_BOM) {
            ByteOrderMark::Le
        } else if self.get(0..2) == Some(&UTF16_BE_BOM) {
            ByteOrderMark::Be
        } else {
            ByteOrderMark::NotPresent
        }
    }

    fn get_utf32_bom(&self) -> ByteOrderMark {
        if self.get(0..4) == Some(&UTF32_LE_BOM) {
            ByteOrderMark::Le
        } else if self.get(0..4) == Some(&UTF32_BE_BOM) {
            ByteOrderMark::Be
        } else {
            ByteOrderMark::NotPresent
        }
    }
}
