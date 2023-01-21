#[doc = "Register `audio_pll_cfg6` reader"]
pub struct R(crate::R<AUDIO_PLL_CFG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_PLL_CFG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_PLL_CFG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_PLL_CFG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `audio_pll_cfg6` writer"]
pub struct W(crate::W<AUDIO_PLL_CFG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_PLL_CFG6_SPEC>;
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
impl From<crate::W<AUDIO_PLL_CFG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_PLL_CFG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aupll_sdmin` reader - "]
pub type AUPLL_SDMIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `aupll_sdmin` writer - "]
pub type AUPLL_SDMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIO_PLL_CFG6_SPEC, u32, u32, 19, O>;
#[doc = "Field `reserved_19_23` reader - "]
pub type RESERVED_19_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `aupll_sdm_bypass` reader - "]
pub type AUPLL_SDM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `aupll_sdm_bypass` writer - "]
pub type AUPLL_SDM_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG6_SPEC, bool, O>;
#[doc = "Field `reserved_25_31` reader - "]
pub type RESERVED_25_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn aupll_sdmin(&self) -> AUPLL_SDMIN_R {
        AUPLL_SDMIN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn reserved_19_23(&self) -> RESERVED_19_23_R {
        RESERVED_19_23_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn aupll_sdm_bypass(&self) -> AUPLL_SDM_BYPASS_R {
        AUPLL_SDM_BYPASS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn reserved_25_31(&self) -> RESERVED_25_31_R {
        RESERVED_25_31_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdmin(&mut self) -> AUPLL_SDMIN_W<0> {
        AUPLL_SDMIN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdm_bypass(&mut self) -> AUPLL_SDM_BYPASS_W<24> {
        AUPLL_SDM_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "audio_pll_cfg6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_pll_cfg6](index.html) module"]
pub struct AUDIO_PLL_CFG6_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_pll_cfg6::R](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_pll_cfg6::W](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg6 to value 0x0001_61e5"]
impl crate::Resettable for AUDIO_PLL_CFG6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_61e5;
}
