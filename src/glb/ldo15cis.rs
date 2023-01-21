#[doc = "Register `ldo15cis` reader"]
pub struct R(crate::R<LDO15CIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO15CIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO15CIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO15CIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo15cis` writer"]
pub struct W(crate::W<LDO15CIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO15CIS_SPEC>;
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
impl From<crate::W<LDO15CIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO15CIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ldo15cis` reader - "]
pub type PU_LDO15CIS_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo15cis` writer - "]
pub type PU_LDO15CIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO15CIS_SPEC, bool, O>;
#[doc = "Field `ldo15cis_bypass` reader - "]
pub type LDO15CIS_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_bypass` writer - "]
pub type LDO15CIS_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO15CIS_SPEC, bool, O>;
#[doc = "Field `ldo15cis_pulldown` reader - "]
pub type LDO15CIS_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_pulldown` writer - "]
pub type LDO15CIS_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO15CIS_SPEC, bool, O>;
#[doc = "Field `ldo15cis_pulldown_sel` reader - "]
pub type LDO15CIS_PULLDOWN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_pulldown_sel` writer - "]
pub type LDO15CIS_PULLDOWN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO15CIS_SPEC, bool, O>;
#[doc = "Field `ldo15cis_bm` reader - "]
pub type LDO15CIS_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15cis_bm` writer - "]
pub type LDO15CIS_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO15CIS_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_cc` reader - "]
pub type LDO15CIS_CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15cis_cc` writer - "]
pub type LDO15CIS_CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO15CIS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo15cis_ocp_out` reader - "]
pub type LDO15CIS_OCP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_ocp_th` reader - "]
pub type LDO15CIS_OCP_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15cis_ocp_th` writer - "]
pub type LDO15CIS_OCP_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO15CIS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo15cis_ocp_en` reader - "]
pub type LDO15CIS_OCP_EN_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_ocp_en` writer - "]
pub type LDO15CIS_OCP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO15CIS_SPEC, bool, O>;
#[doc = "Field `ldo15cis_sstart_delay` reader - "]
pub type LDO15CIS_SSTART_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15cis_sstart_delay` writer - "]
pub type LDO15CIS_SSTART_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO15CIS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo15cis_sstart_en` reader - "]
pub type LDO15CIS_SSTART_EN_R = crate::BitReader<bool>;
#[doc = "Field `ldo15cis_sstart_en` writer - "]
pub type LDO15CIS_SSTART_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO15CIS_SPEC, bool, O>;
#[doc = "Field `ldo15cis_vout_sel` reader - "]
pub type LDO15CIS_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15cis_vout_sel` writer - "]
pub type LDO15CIS_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO15CIS_SPEC, u8, u8, 4, O>;
#[doc = "Field `ldo15cis_vout_trim` reader - "]
pub type LDO15CIS_VOUT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15cis_vout_trim` writer - "]
pub type LDO15CIS_VOUT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO15CIS_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo15cis(&self) -> PU_LDO15CIS_R {
        PU_LDO15CIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo15cis_bypass(&self) -> LDO15CIS_BYPASS_R {
        LDO15CIS_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo15cis_pulldown(&self) -> LDO15CIS_PULLDOWN_R {
        LDO15CIS_PULLDOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo15cis_pulldown_sel(&self) -> LDO15CIS_PULLDOWN_SEL_R {
        LDO15CIS_PULLDOWN_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo15cis_bm(&self) -> LDO15CIS_BM_R {
        LDO15CIS_BM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo15cis_cc(&self) -> LDO15CIS_CC_R {
        LDO15CIS_CC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo15cis_ocp_out(&self) -> LDO15CIS_OCP_OUT_R {
        LDO15CIS_OCP_OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo15cis_ocp_th(&self) -> LDO15CIS_OCP_TH_R {
        LDO15CIS_OCP_TH_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo15cis_ocp_en(&self) -> LDO15CIS_OCP_EN_R {
        LDO15CIS_OCP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15cis_sstart_delay(&self) -> LDO15CIS_SSTART_DELAY_R {
        LDO15CIS_SSTART_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo15cis_sstart_en(&self) -> LDO15CIS_SSTART_EN_R {
        LDO15CIS_SSTART_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo15cis_vout_sel(&self) -> LDO15CIS_VOUT_SEL_R {
        LDO15CIS_VOUT_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo15cis_vout_trim(&self) -> LDO15CIS_VOUT_TRIM_R {
        LDO15CIS_VOUT_TRIM_R::new(((self.bits >> 24) & 0x0f) as u8)
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
    pub fn pu_ldo15cis(&mut self) -> PU_LDO15CIS_W<0> {
        PU_LDO15CIS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_bypass(&mut self) -> LDO15CIS_BYPASS_W<1> {
        LDO15CIS_BYPASS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_pulldown(&mut self) -> LDO15CIS_PULLDOWN_W<2> {
        LDO15CIS_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_pulldown_sel(&mut self) -> LDO15CIS_PULLDOWN_SEL_W<3> {
        LDO15CIS_PULLDOWN_SEL_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_bm(&mut self) -> LDO15CIS_BM_W<4> {
        LDO15CIS_BM_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_cc(&mut self) -> LDO15CIS_CC_W<8> {
        LDO15CIS_CC_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_ocp_th(&mut self) -> LDO15CIS_OCP_TH_W<12> {
        LDO15CIS_OCP_TH_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_ocp_en(&mut self) -> LDO15CIS_OCP_EN_W<15> {
        LDO15CIS_OCP_EN_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_sstart_delay(&mut self) -> LDO15CIS_SSTART_DELAY_W<16> {
        LDO15CIS_SSTART_DELAY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_sstart_en(&mut self) -> LDO15CIS_SSTART_EN_W<19> {
        LDO15CIS_SSTART_EN_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_vout_sel(&mut self) -> LDO15CIS_VOUT_SEL_W<20> {
        LDO15CIS_VOUT_SEL_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15cis_vout_trim(&mut self) -> LDO15CIS_VOUT_TRIM_W<24> {
        LDO15CIS_VOUT_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo15cis\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo15cis](index.html) module"]
pub struct LDO15CIS_SPEC;
impl crate::RegisterSpec for LDO15CIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo15cis::R](R) reader structure"]
impl crate::Readable for LDO15CIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo15cis::W](W) writer structure"]
impl crate::Writable for LDO15CIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo15cis to value 0x088a_b321"]
impl crate::Resettable for LDO15CIS_SPEC {
    const RESET_VALUE: Self::Ux = 0x088a_b321;
}
