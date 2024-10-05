#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ByteOrderMark {
    Le,
    Be,
    NotPresent,
}

impl ByteOrderMark {
    #[allow(dead_code)]
    pub(crate) fn is_present(&self) -> bool {
        *self != ByteOrderMark::NotPresent
    }
}

#[allow(dead_code)]
pub(crate) trait ByteOrderMarkExt {
    fn get_utf8_bom(&self) -> ByteOrderMark;
    fn get_utf16_bom(&self) -> ByteOrderMark;
    fn get_utf32_bom(&self) -> ByteOrderMark;
}

impl ByteOrderMarkExt for [u8] {
    fn get_utf8_bom(&self) -> ByteOrderMark {
        if self.get(0..3) == Some(&[0xEF, 0xBB, 0xBF]) {
            ByteOrderMark::Le
        } else {
            ByteOrderMark::NotPresent
        }
    }

    fn get_utf16_bom(&self) -> ByteOrderMark {
        if self.get(0..2) == Some(&[0xFF, 0xFE]) {
            ByteOrderMark::Le
        } else if self.get(0..2) == Some(&[0xFE, 0xFF]) {
            ByteOrderMark::Be
        } else {
            ByteOrderMark::NotPresent
        }
    }

    fn get_utf32_bom(&self) -> ByteOrderMark {
        if self.get(0..4) == Some(&[0xFF, 0xFE, 0, 0]) {
            ByteOrderMark::Le
        } else if self.get(0..4) == Some(&[0, 0, 0xFE, 0xFF]) {
            ByteOrderMark::Be
        } else {
            ByteOrderMark::NotPresent
        }
    }
}
