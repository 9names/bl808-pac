#[doc = "Register `sd_capabilities_3` reader"]
pub struct R(crate::R<SD_CAPABILITIES_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CAPABILITIES_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CAPABILITIES_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CAPABILITIES_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_capabilities_3` writer"]
pub struct W(crate::W<SD_CAPABILITIES_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CAPABILITIES_3_SPEC>;
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
impl From<crate::W<SD_CAPABILITIES_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CAPABILITIES_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdr50_support` reader - "]
pub type SDR50_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `sdr104_support` reader - "]
pub type SDR104_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `ddr50_support` reader - "]
pub type DDR50_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `drv_type_a` reader - "]
pub type DRV_TYPE_A_R = crate::BitReader<bool>;
#[doc = "Field `drv_type_c` reader - "]
pub type DRV_TYPE_C_R = crate::BitReader<bool>;
#[doc = "Field `drv_type_d` reader - "]
pub type DRV_TYPE_D_R = crate::BitReader<bool>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `tmr_retune` reader - "]
pub type TMR_RETUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_12` reader - "]
pub type RESERVED_12_R = crate::BitReader<bool>;
#[doc = "Field `sdr50_tune` reader - "]
pub type SDR50_TUNE_R = crate::BitReader<bool>;
#[doc = "Field `retune_modes` reader - "]
pub type RETUNE_MODES_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> SDR50_SUPPORT_R {
        SDR50_SUPPORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> SDR104_SUPPORT_R {
        SDR104_SUPPORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
        DDR50_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn drv_type_a(&self) -> DRV_TYPE_A_R {
        DRV_TYPE_A_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn drv_type_c(&self) -> DRV_TYPE_C_R {
        DRV_TYPE_C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn drv_type_d(&self) -> DRV_TYPE_D_R {
        DRV_TYPE_D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tmr_retune(&self) -> TMR_RETUNE_R {
        TMR_RETUNE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reserved_12(&self) -> RESERVED_12_R {
        RESERVED_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sdr50_tune(&self) -> SDR50_TUNE_R {
        SDR50_TUNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn retune_modes(&self) -> RETUNE_MODES_R {
        RETUNE_MODES_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capabilities Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_capabilities_3](index.html) module"]
pub struct SD_CAPABILITIES_3_SPEC;
impl crate::RegisterSpec for SD_CAPABILITIES_3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_capabilities_3::R](R) reader structure"]
impl crate::Readable for SD_CAPABILITIES_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_capabilities_3::W](W) writer structure"]
impl crate::Writable for SD_CAPABILITIES_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_capabilities_3 to value 0x2f77"]
impl crate::Resettable for SD_CAPABILITIES_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x2f77;
}
