#[doc = "Register `sd_cmd` reader"]
pub struct R(crate::R<SD_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_cmd` writer"]
pub struct W(crate::W<SD_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `resp_type` reader - "]
pub type RESP_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `resp_type` writer - "]
pub type RESP_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SD_CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2` reader - "]
pub type RESERVED_2_R = crate::BitReader<bool>;
#[doc = "Field `cmd_crc_chk_en` reader - "]
pub type CMD_CRC_CHK_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_crc_chk_en` writer - "]
pub type CMD_CRC_CHK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CMD_SPEC, bool, O>;
#[doc = "Field `cmd_index_chk_en` reader - "]
pub type CMD_INDEX_CHK_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_index_chk_en` writer - "]
pub type CMD_INDEX_CHK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CMD_SPEC, bool, O>;
#[doc = "Field `data_present` reader - "]
pub type DATA_PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `data_present` writer - "]
pub type DATA_PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CMD_SPEC, bool, O>;
#[doc = "Field `cmd_type` reader - "]
pub type CMD_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cmd_type` writer - "]
pub type CMD_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SD_CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `cmd_index` reader - "]
pub type CMD_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cmd_index` writer - "]
pub type CMD_INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SD_CMD_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_15_14` reader - "]
pub type RESERVED_15_14_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn resp_type(&self) -> RESP_TYPE_R {
        RESP_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reserved_2(&self) -> RESERVED_2_R {
        RESERVED_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_crc_chk_en(&self) -> CMD_CRC_CHK_EN_R {
        CMD_CRC_CHK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cmd_index_chk_en(&self) -> CMD_INDEX_CHK_EN_R {
        CMD_INDEX_CHK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn data_present(&self) -> DATA_PRESENT_R {
        DATA_PRESENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_15_14(&self) -> RESERVED_15_14_R {
        RESERVED_15_14_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type(&mut self) -> RESP_TYPE_W<0> {
        RESP_TYPE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_chk_en(&mut self) -> CMD_CRC_CHK_EN_W<3> {
        CMD_CRC_CHK_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index_chk_en(&mut self) -> CMD_INDEX_CHK_EN_W<4> {
        CMD_INDEX_CHK_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn data_present(&mut self) -> DATA_PRESENT_W<5> {
        DATA_PRESENT_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W<6> {
        CMD_TYPE_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W<8> {
        CMD_INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_cmd](index.html) module"]
pub struct SD_CMD_SPEC;
impl crate::RegisterSpec for SD_CMD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_cmd::R](R) reader structure"]
impl crate::Readable for SD_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_cmd::W](W) writer structure"]
impl crate::Writable for SD_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_cmd to value 0"]
impl crate::Resettable for SD_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
