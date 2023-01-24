#[doc = "Register `uhs_pll_cfg9` reader"]
pub struct R(crate::R<UHS_PLL_CFG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PLL_CFG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PLL_CFG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PLL_CFG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_pll_cfg9` writer"]
pub struct W(crate::W<UHS_PLL_CFG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PLL_CFG9_SPEC>;
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
impl From<crate::W<UHS_PLL_CFG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PLL_CFG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhspll_ssc_en` reader - "]
pub type UHSPLL_SSC_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_ssc_en` writer - "]
pub type UHSPLL_SSC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_ssc_cnt` reader - "]
pub type UHSPLL_SSC_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_ssc_cnt` writer - "]
pub type UHSPLL_SSC_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG9_SPEC, u8, u8, 8, O>;
#[doc = "Field `uhspll_ssc_gain` reader - "]
pub type UHSPLL_SSC_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_ssc_gain` writer - "]
pub type UHSPLL_SSC_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG9_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_ssc_en(&self) -> UHSPLL_SSC_EN_R {
        UHSPLL_SSC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn uhspll_ssc_cnt(&self) -> UHSPLL_SSC_CNT_R {
        UHSPLL_SSC_CNT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn uhspll_ssc_gain(&self) -> UHSPLL_SSC_GAIN_R {
        UHSPLL_SSC_GAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_ssc_en(&mut self) -> UHSPLL_SSC_EN_W<0> {
        UHSPLL_SSC_EN_W::new(self)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_ssc_cnt(&mut self) -> UHSPLL_SSC_CNT_W<4> {
        UHSPLL_SSC_CNT_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_ssc_gain(&mut self) -> UHSPLL_SSC_GAIN_W<12> {
        UHSPLL_SSC_GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhs_pll_cfg9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_pll_cfg9](index.html) module"]
pub struct UHS_PLL_CFG9_SPEC;
impl crate::RegisterSpec for UHS_PLL_CFG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_pll_cfg9::R](R) reader structure"]
impl crate::Readable for UHS_PLL_CFG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_pll_cfg9::W](W) writer structure"]
impl crate::Writable for UHS_PLL_CFG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_pll_cfg9 to value 0x5640"]
impl crate::Resettable for UHS_PLL_CFG9_SPEC {
    const RESET_VALUE: Self::Ux = 0x5640;
}
