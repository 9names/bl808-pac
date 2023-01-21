#[doc = "Register `dac_cfg0` reader"]
pub struct R(crate::R<DAC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dac_cfg0` writer"]
pub struct W(crate::W<DAC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CFG0_SPEC>;
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
impl From<crate::W<DAC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdaca_rstn_ana` reader - "]
pub type GPDACA_RSTN_ANA_R = crate::BitReader<bool>;
#[doc = "Field `gpdaca_rstn_ana` writer - "]
pub type GPDACA_RSTN_ANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CFG0_SPEC, bool, O>;
#[doc = "Field `gpdacb_rstn_ana` reader - "]
pub type GPDACB_RSTN_ANA_R = crate::BitReader<bool>;
#[doc = "Field `gpdacb_rstn_ana` writer - "]
pub type GPDACB_RSTN_ANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_2_6` reader - "]
pub type RESERVED_2_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_test_en` reader - "]
pub type GPDAC_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_test_en` writer - "]
pub type GPDAC_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CFG0_SPEC, bool, O>;
#[doc = "Field `gpdac_ref_sel` reader - "]
pub type GPDAC_REF_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_ref_sel` writer - "]
pub type GPDAC_REF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CFG0_SPEC, bool, O>;
#[doc = "Field `gpdac_test_sel` reader - "]
pub type GPDAC_TEST_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_test_sel` writer - "]
pub type GPDAC_TEST_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_12_23` reader - "]
pub type RESERVED_12_23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gpdac_reserved` reader - "]
pub type GPDAC_RESERVED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_reserved` writer - "]
pub type GPDAC_RESERVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_CFG0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&self) -> GPDACA_RSTN_ANA_R {
        GPDACA_RSTN_ANA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&self) -> GPDACB_RSTN_ANA_R {
        GPDACB_RSTN_ANA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn reserved_2_6(&self) -> RESERVED_2_6_R {
        RESERVED_2_6_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&self) -> GPDAC_TEST_EN_R {
        GPDAC_TEST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&self) -> GPDAC_REF_SEL_R {
        GPDAC_REF_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&self) -> GPDAC_TEST_SEL_R {
        GPDAC_TEST_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn reserved_12_23(&self) -> RESERVED_12_23_R {
        RESERVED_12_23_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&self) -> GPDAC_RESERVED_R {
        GPDAC_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdaca_rstn_ana(&mut self) -> GPDACA_RSTN_ANA_W<0> {
        GPDACA_RSTN_ANA_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdacb_rstn_ana(&mut self) -> GPDACB_RSTN_ANA_W<1> {
        GPDACB_RSTN_ANA_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_test_en(&mut self) -> GPDAC_TEST_EN_W<7> {
        GPDAC_TEST_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ref_sel(&mut self) -> GPDAC_REF_SEL_W<8> {
        GPDAC_REF_SEL_W::new(self)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_test_sel(&mut self) -> GPDAC_TEST_SEL_W<9> {
        GPDAC_TEST_SEL_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_reserved(&mut self) -> GPDAC_RESERVED_W<24> {
        GPDAC_RESERVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dac_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cfg0](index.html) module"]
pub struct DAC_CFG0_SPEC;
impl crate::RegisterSpec for DAC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_cfg0::R](R) reader structure"]
impl crate::Readable for DAC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_cfg0::W](W) writer structure"]
impl crate::Writable for DAC_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dac_cfg0 to value 0x0f00_0003"]
impl crate::Resettable for DAC_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_0003;
}
