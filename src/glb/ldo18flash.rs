#[doc = "Register `ldo18flash` reader"]
pub struct R(crate::R<LDO18FLASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO18FLASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO18FLASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO18FLASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo18flash` writer"]
pub struct W(crate::W<LDO18FLASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO18FLASH_SPEC>;
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
impl From<crate::W<LDO18FLASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO18FLASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ldo18flash` reader - "]
pub type PU_LDO18FLASH_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo18flash` writer - "]
pub type PU_LDO18FLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_bypass` reader - "]
pub type LDO18FLASH_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_bypass` writer - "]
pub type LDO18FLASH_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_pulldown` reader - "]
pub type LDO18FLASH_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_pulldown` writer - "]
pub type LDO18FLASH_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_pulldown_sel` reader - "]
pub type LDO18FLASH_PULLDOWN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_pulldown_sel` writer - "]
pub type LDO18FLASH_PULLDOWN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_bm` reader - "]
pub type LDO18FLASH_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18flash_bm` writer - "]
pub type LDO18FLASH_BM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18FLASH_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_cc` reader - "]
pub type LDO18FLASH_CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18flash_cc` writer - "]
pub type LDO18FLASH_CC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18FLASH_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo18flash_ocp_out` reader - "]
pub type LDO18FLASH_OCP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_ocp_th` reader - "]
pub type LDO18FLASH_OCP_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18flash_ocp_th` writer - "]
pub type LDO18FLASH_OCP_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18FLASH_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo18flash_ocp_en` reader - "]
pub type LDO18FLASH_OCP_EN_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_ocp_en` writer - "]
pub type LDO18FLASH_OCP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_sstart_delay` reader - "]
pub type LDO18FLASH_SSTART_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18flash_sstart_delay` writer - "]
pub type LDO18FLASH_SSTART_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18FLASH_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo18flash_sstart_en` reader - "]
pub type LDO18FLASH_SSTART_EN_R = crate::BitReader<bool>;
#[doc = "Field `ldo18flash_sstart_en` writer - "]
pub type LDO18FLASH_SSTART_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18FLASH_SPEC, bool, O>;
#[doc = "Field `ldo18flash_vout_sel` reader - "]
pub type LDO18FLASH_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18flash_vout_sel` writer - "]
pub type LDO18FLASH_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18FLASH_SPEC, u8, u8, 4, O>;
#[doc = "Field `ldo18flash_vout_trim` reader - "]
pub type LDO18FLASH_VOUT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18flash_vout_trim` writer - "]
pub type LDO18FLASH_VOUT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18FLASH_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo18flash(&self) -> PU_LDO18FLASH_R {
        PU_LDO18FLASH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18flash_bypass(&self) -> LDO18FLASH_BYPASS_R {
        LDO18FLASH_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18flash_pulldown(&self) -> LDO18FLASH_PULLDOWN_R {
        LDO18FLASH_PULLDOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18flash_pulldown_sel(&self) -> LDO18FLASH_PULLDOWN_SEL_R {
        LDO18FLASH_PULLDOWN_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18flash_bm(&self) -> LDO18FLASH_BM_R {
        LDO18FLASH_BM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18flash_cc(&self) -> LDO18FLASH_CC_R {
        LDO18FLASH_CC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo18flash_ocp_out(&self) -> LDO18FLASH_OCP_OUT_R {
        LDO18FLASH_OCP_OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18flash_ocp_th(&self) -> LDO18FLASH_OCP_TH_R {
        LDO18FLASH_OCP_TH_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18flash_ocp_en(&self) -> LDO18FLASH_OCP_EN_R {
        LDO18FLASH_OCP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo18flash_sstart_delay(&self) -> LDO18FLASH_SSTART_DELAY_R {
        LDO18FLASH_SSTART_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo18flash_sstart_en(&self) -> LDO18FLASH_SSTART_EN_R {
        LDO18FLASH_SSTART_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo18flash_vout_sel(&self) -> LDO18FLASH_VOUT_SEL_R {
        LDO18FLASH_VOUT_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18flash_vout_trim(&self) -> LDO18FLASH_VOUT_TRIM_R {
        LDO18FLASH_VOUT_TRIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo18flash(&mut self) -> PU_LDO18FLASH_W<0> {
        PU_LDO18FLASH_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_bypass(&mut self) -> LDO18FLASH_BYPASS_W<1> {
        LDO18FLASH_BYPASS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_pulldown(&mut self) -> LDO18FLASH_PULLDOWN_W<2> {
        LDO18FLASH_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_pulldown_sel(&mut self) -> LDO18FLASH_PULLDOWN_SEL_W<3> {
        LDO18FLASH_PULLDOWN_SEL_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_bm(&mut self) -> LDO18FLASH_BM_W<4> {
        LDO18FLASH_BM_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_cc(&mut self) -> LDO18FLASH_CC_W<8> {
        LDO18FLASH_CC_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_ocp_th(&mut self) -> LDO18FLASH_OCP_TH_W<12> {
        LDO18FLASH_OCP_TH_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_ocp_en(&mut self) -> LDO18FLASH_OCP_EN_W<15> {
        LDO18FLASH_OCP_EN_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_sstart_delay(&mut self) -> LDO18FLASH_SSTART_DELAY_W<16> {
        LDO18FLASH_SSTART_DELAY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_sstart_en(&mut self) -> LDO18FLASH_SSTART_EN_W<19> {
        LDO18FLASH_SSTART_EN_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_vout_sel(&mut self) -> LDO18FLASH_VOUT_SEL_W<20> {
        LDO18FLASH_VOUT_SEL_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18flash_vout_trim(&mut self) -> LDO18FLASH_VOUT_TRIM_W<24> {
        LDO18FLASH_VOUT_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo18flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo18flash](index.html) module"]
pub struct LDO18FLASH_SPEC;
impl crate::RegisterSpec for LDO18FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo18flash::R](R) reader structure"]
impl crate::Readable for LDO18FLASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo18flash::W](W) writer structure"]
impl crate::Writable for LDO18FLASH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo18flash to value 0x073b_b330"]
impl crate::Resettable for LDO18FLASH_SPEC {
    const RESET_VALUE: Self::Ux = 0x073b_b330;
}
