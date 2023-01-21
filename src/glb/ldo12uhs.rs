#[doc = "Register `ldo12uhs` reader"]
pub struct R(crate::R<LDO12UHS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO12UHS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO12UHS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO12UHS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo12uhs` writer"]
pub struct W(crate::W<LDO12UHS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO12UHS_SPEC>;
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
impl From<crate::W<LDO12UHS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO12UHS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ldo12uhs` reader - "]
pub type PU_LDO12UHS_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo12uhs` writer - "]
pub type PU_LDO12UHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO12UHS_SPEC, bool, O>;
#[doc = "Field `ldo12uhs_bypass` reader - "]
pub type LDO12UHS_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_bypass` writer - "]
pub type LDO12UHS_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO12UHS_SPEC, bool, O>;
#[doc = "Field `ldo12uhs_pulldown` reader - "]
pub type LDO12UHS_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_pulldown` writer - "]
pub type LDO12UHS_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO12UHS_SPEC, bool, O>;
#[doc = "Field `ldo12uhs_pulldown_sel` reader - "]
pub type LDO12UHS_PULLDOWN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_pulldown_sel` writer - "]
pub type LDO12UHS_PULLDOWN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO12UHS_SPEC, bool, O>;
#[doc = "Field `ldo12uhs_bm` reader - "]
pub type LDO12UHS_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo12uhs_bm` writer - "]
pub type LDO12UHS_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO12UHS_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_cc` reader - "]
pub type LDO12UHS_CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo12uhs_cc` writer - "]
pub type LDO12UHS_CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO12UHS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo12uhs_ocp_out` reader - "]
pub type LDO12UHS_OCP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_ocp_th` reader - "]
pub type LDO12UHS_OCP_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo12uhs_ocp_th` writer - "]
pub type LDO12UHS_OCP_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO12UHS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo12uhs_ocp_en` reader - "]
pub type LDO12UHS_OCP_EN_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_ocp_en` writer - "]
pub type LDO12UHS_OCP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO12UHS_SPEC, bool, O>;
#[doc = "Field `ldo12uhs_sstart_delay` reader - "]
pub type LDO12UHS_SSTART_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo12uhs_sstart_delay` writer - "]
pub type LDO12UHS_SSTART_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO12UHS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo12uhs_sstart_en` reader - "]
pub type LDO12UHS_SSTART_EN_R = crate::BitReader<bool>;
#[doc = "Field `ldo12uhs_sstart_en` writer - "]
pub type LDO12UHS_SSTART_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO12UHS_SPEC, bool, O>;
#[doc = "Field `ldo12uhs_vout_sel` reader - "]
pub type LDO12UHS_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo12uhs_vout_sel` writer - "]
pub type LDO12UHS_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO12UHS_SPEC, u8, u8, 4, O>;
#[doc = "Field `ldo12uhs_vout_trim` reader - "]
pub type LDO12UHS_VOUT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo12uhs_vout_trim` writer - "]
pub type LDO12UHS_VOUT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO12UHS_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo12uhs(&self) -> PU_LDO12UHS_R {
        PU_LDO12UHS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo12uhs_bypass(&self) -> LDO12UHS_BYPASS_R {
        LDO12UHS_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo12uhs_pulldown(&self) -> LDO12UHS_PULLDOWN_R {
        LDO12UHS_PULLDOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo12uhs_pulldown_sel(&self) -> LDO12UHS_PULLDOWN_SEL_R {
        LDO12UHS_PULLDOWN_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo12uhs_bm(&self) -> LDO12UHS_BM_R {
        LDO12UHS_BM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo12uhs_cc(&self) -> LDO12UHS_CC_R {
        LDO12UHS_CC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_out(&self) -> LDO12UHS_OCP_OUT_R {
        LDO12UHS_OCP_OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_th(&self) -> LDO12UHS_OCP_TH_R {
        LDO12UHS_OCP_TH_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_en(&self) -> LDO12UHS_OCP_EN_R {
        LDO12UHS_OCP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo12uhs_sstart_delay(&self) -> LDO12UHS_SSTART_DELAY_R {
        LDO12UHS_SSTART_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo12uhs_sstart_en(&self) -> LDO12UHS_SSTART_EN_R {
        LDO12UHS_SSTART_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo12uhs_vout_sel(&self) -> LDO12UHS_VOUT_SEL_R {
        LDO12UHS_VOUT_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo12uhs_vout_trim(&self) -> LDO12UHS_VOUT_TRIM_R {
        LDO12UHS_VOUT_TRIM_R::new(((self.bits >> 24) & 0x0f) as u8)
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
    pub fn pu_ldo12uhs(&mut self) -> PU_LDO12UHS_W<0> {
        PU_LDO12UHS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_bypass(&mut self) -> LDO12UHS_BYPASS_W<1> {
        LDO12UHS_BYPASS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_pulldown(&mut self) -> LDO12UHS_PULLDOWN_W<2> {
        LDO12UHS_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_pulldown_sel(&mut self) -> LDO12UHS_PULLDOWN_SEL_W<3> {
        LDO12UHS_PULLDOWN_SEL_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_bm(&mut self) -> LDO12UHS_BM_W<4> {
        LDO12UHS_BM_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_cc(&mut self) -> LDO12UHS_CC_W<8> {
        LDO12UHS_CC_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_ocp_th(&mut self) -> LDO12UHS_OCP_TH_W<12> {
        LDO12UHS_OCP_TH_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_ocp_en(&mut self) -> LDO12UHS_OCP_EN_W<15> {
        LDO12UHS_OCP_EN_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_sstart_delay(&mut self) -> LDO12UHS_SSTART_DELAY_W<16> {
        LDO12UHS_SSTART_DELAY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_sstart_en(&mut self) -> LDO12UHS_SSTART_EN_W<19> {
        LDO12UHS_SSTART_EN_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_vout_sel(&mut self) -> LDO12UHS_VOUT_SEL_W<20> {
        LDO12UHS_VOUT_SEL_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ldo12uhs_vout_trim(&mut self) -> LDO12UHS_VOUT_TRIM_W<24> {
        LDO12UHS_VOUT_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo12uhs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo12uhs](index.html) module"]
pub struct LDO12UHS_SPEC;
impl crate::RegisterSpec for LDO12UHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo12uhs::R](R) reader structure"]
impl crate::Readable for LDO12UHS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo12uhs::W](W) writer structure"]
impl crate::Writable for LDO12UHS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo12uhs to value 0x083a_b320"]
impl crate::Resettable for LDO12UHS_SPEC {
    const RESET_VALUE: Self::Ux = 0x083a_b320;
}
