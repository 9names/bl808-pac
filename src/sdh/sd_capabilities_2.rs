#[doc = "Register `sd_capabilities_2` reader"]
pub struct R(crate::R<SD_CAPABILITIES_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CAPABILITIES_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CAPABILITIES_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CAPABILITIES_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_capabilities_2` writer"]
pub struct W(crate::W<SD_CAPABILITIES_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CAPABILITIES_2_SPEC>;
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
impl From<crate::W<SD_CAPABILITIES_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CAPABILITIES_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `max_blk_len` reader - "]
pub type MAX_BLK_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ex_data_width_support` reader - "]
pub type EX_DATA_WIDTH_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `adma2_support` reader - "]
pub type ADMA2_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `adma1_support` reader - "]
pub type ADMA1_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `hi_speed_support` reader - "]
pub type HI_SPEED_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `sdma_support` reader - "]
pub type SDMA_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `sus_res_support` reader - "]
pub type SUS_RES_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `vlg_33_support` reader - "]
pub type VLG_33_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `vlg_30_support` reader - "]
pub type VLG_30_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `vlg_18_support` reader - "]
pub type VLG_18_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `sys_bus_64_support` reader - "]
pub type SYS_BUS_64_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `async_int_support` reader - "]
pub type ASYNC_INT_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `cfg_slot_type` reader - "]
pub type CFG_SLOT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cfg_slot_type` writer - "]
pub type CFG_SLOT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_CAPABILITIES_2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn max_blk_len(&self) -> MAX_BLK_LEN_R {
        MAX_BLK_LEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ex_data_width_support(&self) -> EX_DATA_WIDTH_SUPPORT_R {
        EX_DATA_WIDTH_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adma2_support(&self) -> ADMA2_SUPPORT_R {
        ADMA2_SUPPORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adma1_support(&self) -> ADMA1_SUPPORT_R {
        ADMA1_SUPPORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hi_speed_support(&self) -> HI_SPEED_SUPPORT_R {
        HI_SPEED_SUPPORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdma_support(&self) -> SDMA_SUPPORT_R {
        SDMA_SUPPORT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sus_res_support(&self) -> SUS_RES_SUPPORT_R {
        SUS_RES_SUPPORT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vlg_33_support(&self) -> VLG_33_SUPPORT_R {
        VLG_33_SUPPORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vlg_30_support(&self) -> VLG_30_SUPPORT_R {
        VLG_30_SUPPORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vlg_18_support(&self) -> VLG_18_SUPPORT_R {
        VLG_18_SUPPORT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sys_bus_64_support(&self) -> SYS_BUS_64_SUPPORT_R {
        SYS_BUS_64_SUPPORT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn async_int_support(&self) -> ASYNC_INT_SUPPORT_R {
        ASYNC_INT_SUPPORT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cfg_slot_type(&self) -> CFG_SLOT_TYPE_R {
        CFG_SLOT_TYPE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_slot_type(&mut self) -> CFG_SLOT_TYPE_W<14> {
        CFG_SLOT_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capabilities Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_capabilities_2](index.html) module"]
pub struct SD_CAPABILITIES_2_SPEC;
impl crate::RegisterSpec for SD_CAPABILITIES_2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_capabilities_2::R](R) reader structure"]
impl crate::Readable for SD_CAPABILITIES_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_capabilities_2::W](W) writer structure"]
impl crate::Writable for SD_CAPABILITIES_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_capabilities_2 to value 0x25fc"]
impl crate::Resettable for SD_CAPABILITIES_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x25fc;
}
