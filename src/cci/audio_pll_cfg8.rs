#[doc = "Register `audio_pll_cfg8` reader"]
pub struct R(crate::R<AUDIO_PLL_CFG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_PLL_CFG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_PLL_CFG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_PLL_CFG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `audio_pll_cfg8` writer"]
pub struct W(crate::W<AUDIO_PLL_CFG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_PLL_CFG8_SPEC>;
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
impl From<crate::W<AUDIO_PLL_CFG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_PLL_CFG8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aupll_en_div1` reader - "]
pub type AUPLL_EN_DIV1_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div1` writer - "]
pub type AUPLL_EN_DIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div2` reader - "]
pub type AUPLL_EN_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div2` writer - "]
pub type AUPLL_EN_DIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div2p5` reader - "]
pub type AUPLL_EN_DIV2P5_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div2p5` writer - "]
pub type AUPLL_EN_DIV2P5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div3` reader - "]
pub type AUPLL_EN_DIV3_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div3` writer - "]
pub type AUPLL_EN_DIV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div4` reader - "]
pub type AUPLL_EN_DIV4_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div4` writer - "]
pub type AUPLL_EN_DIV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div5` reader - "]
pub type AUPLL_EN_DIV5_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div5` writer - "]
pub type AUPLL_EN_DIV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div6` reader - "]
pub type AUPLL_EN_DIV6_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div6` writer - "]
pub type AUPLL_EN_DIV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div10` reader - "]
pub type AUPLL_EN_DIV10_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div10` writer - "]
pub type AUPLL_EN_DIV10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_en_div15` reader - "]
pub type AUPLL_EN_DIV15_R = crate::BitReader<bool>;
#[doc = "Field `aupll_en_div15` writer - "]
pub type AUPLL_EN_DIV15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `aupll_sel_div1_div2` reader - "]
pub type AUPLL_SEL_DIV1_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `aupll_sel_div1_div2` writer - "]
pub type AUPLL_SEL_DIV1_DIV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `reserved_10_31` reader - "]
pub type RESERVED_10_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_en_div1(&self) -> AUPLL_EN_DIV1_R {
        AUPLL_EN_DIV1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aupll_en_div2(&self) -> AUPLL_EN_DIV2_R {
        AUPLL_EN_DIV2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aupll_en_div2p5(&self) -> AUPLL_EN_DIV2P5_R {
        AUPLL_EN_DIV2P5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aupll_en_div3(&self) -> AUPLL_EN_DIV3_R {
        AUPLL_EN_DIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn aupll_en_div4(&self) -> AUPLL_EN_DIV4_R {
        AUPLL_EN_DIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn aupll_en_div5(&self) -> AUPLL_EN_DIV5_R {
        AUPLL_EN_DIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aupll_en_div6(&self) -> AUPLL_EN_DIV6_R {
        AUPLL_EN_DIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn aupll_en_div10(&self) -> AUPLL_EN_DIV10_R {
        AUPLL_EN_DIV10_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn aupll_en_div15(&self) -> AUPLL_EN_DIV15_R {
        AUPLL_EN_DIV15_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn aupll_sel_div1_div2(&self) -> AUPLL_SEL_DIV1_DIV2_R {
        AUPLL_SEL_DIV1_DIV2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn reserved_10_31(&self) -> RESERVED_10_31_R {
        RESERVED_10_31_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div1(&mut self) -> AUPLL_EN_DIV1_W<0> {
        AUPLL_EN_DIV1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div2(&mut self) -> AUPLL_EN_DIV2_W<1> {
        AUPLL_EN_DIV2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div2p5(&mut self) -> AUPLL_EN_DIV2P5_W<2> {
        AUPLL_EN_DIV2P5_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div3(&mut self) -> AUPLL_EN_DIV3_W<3> {
        AUPLL_EN_DIV3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div4(&mut self) -> AUPLL_EN_DIV4_W<4> {
        AUPLL_EN_DIV4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div5(&mut self) -> AUPLL_EN_DIV5_W<5> {
        AUPLL_EN_DIV5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div6(&mut self) -> AUPLL_EN_DIV6_W<6> {
        AUPLL_EN_DIV6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div10(&mut self) -> AUPLL_EN_DIV10_W<7> {
        AUPLL_EN_DIV10_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_en_div15(&mut self) -> AUPLL_EN_DIV15_W<8> {
        AUPLL_EN_DIV15_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sel_div1_div2(&mut self) -> AUPLL_SEL_DIV1_DIV2_W<9> {
        AUPLL_SEL_DIV1_DIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "audio_pll_cfg8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_pll_cfg8](index.html) module"]
pub struct AUDIO_PLL_CFG8_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_pll_cfg8::R](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_pll_cfg8::W](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg8 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
