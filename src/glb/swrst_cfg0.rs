#[doc = "Register `swrst_cfg0` reader"]
pub struct R(crate::R<SWRST_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg0` writer"]
pub struct W(crate::W<SWRST_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG0_SPEC>;
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
impl From<crate::W<SWRST_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `swrst_s00` reader - "]
pub type SWRST_S00_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s00` writer - "]
pub type SWRST_S00_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s01` reader - "]
pub type SWRST_S01_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s01` writer - "]
pub type SWRST_S01_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `swrst_s20` reader - "]
pub type SWRST_S20_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s20` writer - "]
pub type SWRST_S20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `swrst_s30` reader - "]
pub type SWRST_S30_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s30` writer - "]
pub type SWRST_S30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s31` reader - "]
pub type SWRST_S31_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s31` writer - "]
pub type SWRST_S31_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s32` reader - "]
pub type SWRST_S32_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s32` writer - "]
pub type SWRST_S32_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `swrst_s33` reader - "]
pub type SWRST_S33_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s33` writer - "]
pub type SWRST_S33_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `swrst_s1_ext_emi_misc` reader - "]
pub type SWRST_S1_EXT_EMI_MISC_R = crate::BitReader<bool>;
#[doc = "Field `swrst_s1_ext_emi_misc` writer - "]
pub type SWRST_S1_EXT_EMI_MISC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&self) -> SWRST_S00_R {
        SWRST_S00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&self) -> SWRST_S01_R {
        SWRST_S01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&self) -> SWRST_S20_R {
        SWRST_S20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&self) -> SWRST_S30_R {
        SWRST_S30_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s31(&self) -> SWRST_S31_R {
        SWRST_S31_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s32(&self) -> SWRST_S32_R {
        SWRST_S32_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s33(&self) -> SWRST_S33_R {
        SWRST_S33_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1_ext_emi_misc(&self) -> SWRST_S1_EXT_EMI_MISC_R {
        SWRST_S1_EXT_EMI_MISC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s00(&mut self) -> SWRST_S00_W<0> {
        SWRST_S00_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s01(&mut self) -> SWRST_S01_W<1> {
        SWRST_S01_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s20(&mut self) -> SWRST_S20_W<4> {
        SWRST_S20_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s30(&mut self) -> SWRST_S30_W<8> {
        SWRST_S30_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s31(&mut self) -> SWRST_S31_W<9> {
        SWRST_S31_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s32(&mut self) -> SWRST_S32_W<10> {
        SWRST_S32_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s33(&mut self) -> SWRST_S33_W<11> {
        SWRST_S33_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn swrst_s1_ext_emi_misc(&mut self) -> SWRST_S1_EXT_EMI_MISC_W<16> {
        SWRST_S1_EXT_EMI_MISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_s1_ext + swrst_s3 + swrst_s2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg0](index.html) module"]
pub struct SWRST_CFG0_SPEC;
impl crate::RegisterSpec for SWRST_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg0::R](R) reader structure"]
impl crate::Readable for SWRST_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg0::W](W) writer structure"]
impl crate::Writable for SWRST_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg0 to value 0"]
impl crate::Resettable for SWRST_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
