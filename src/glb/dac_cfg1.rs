#[doc = "Register `dac_cfg1` reader"]
pub struct R(crate::R<DAC_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dac_cfg1` writer"]
pub struct W(crate::W<DAC_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CFG1_SPEC>;
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
impl From<crate::W<DAC_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_a_en` reader - "]
pub type GPDAC_A_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_a_en` writer - "]
pub type GPDAC_A_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CFG1_SPEC, bool, O>;
#[doc = "Field `gpdac_ioa_en` reader - "]
pub type GPDAC_IOA_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpdac_ioa_en` writer - "]
pub type GPDAC_IOA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_2_17` reader - "]
pub type RESERVED_2_17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gpdac_a_rng` reader - "]
pub type GPDAC_A_RNG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_a_rng` writer - "]
pub type GPDAC_A_RNG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpdac_a_outmux` reader - "]
pub type GPDAC_A_OUTMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpdac_a_outmux` writer - "]
pub type GPDAC_A_OUTMUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_CFG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_23_31` reader - "]
pub type RESERVED_23_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&self) -> GPDAC_A_EN_R {
        GPDAC_A_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&self) -> GPDAC_IOA_EN_R {
        GPDAC_IOA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:17"]
    #[inline(always)]
    pub fn reserved_2_17(&self) -> RESERVED_2_17_R {
        RESERVED_2_17_R::new(((self.bits >> 2) & 0xffff) as u16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&self) -> GPDAC_A_RNG_R {
        GPDAC_A_RNG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&self) -> GPDAC_A_OUTMUX_R {
        GPDAC_A_OUTMUX_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn reserved_23_31(&self) -> RESERVED_23_31_R {
        RESERVED_23_31_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_en(&mut self) -> GPDAC_A_EN_W<0> {
        GPDAC_A_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ioa_en(&mut self) -> GPDAC_IOA_EN_W<1> {
        GPDAC_IOA_EN_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_rng(&mut self) -> GPDAC_A_RNG_W<18> {
        GPDAC_A_RNG_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_a_outmux(&mut self) -> GPDAC_A_OUTMUX_W<20> {
        GPDAC_A_OUTMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dac_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cfg1](index.html) module"]
pub struct DAC_CFG1_SPEC;
impl crate::RegisterSpec for DAC_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_cfg1::R](R) reader structure"]
impl crate::Readable for DAC_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_cfg1::W](W) writer structure"]
impl crate::Writable for DAC_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dac_cfg1 to value 0x000c_0000"]
impl crate::Resettable for DAC_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_0000;
}
