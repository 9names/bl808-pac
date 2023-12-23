#[doc = "Register `mipi_pll_cfg9` reader"]
pub struct R(crate::R<MIPI_PLL_CFG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_PLL_CFG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_PLL_CFG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_PLL_CFG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mipi_pll_cfg9` writer"]
pub struct W(crate::W<MIPI_PLL_CFG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_PLL_CFG9_SPEC>;
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
impl From<crate::W<MIPI_PLL_CFG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_PLL_CFG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mipipll_ssc_en` reader - "]
pub type MIPIPLL_SSC_EN_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_ssc_en` writer - "]
pub type MIPIPLL_SSC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_ssc_cnt` reader - "]
pub type MIPIPLL_SSC_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_ssc_cnt` writer - "]
pub type MIPIPLL_SSC_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MIPI_PLL_CFG9_SPEC, u8, u8, 8, O>;
#[doc = "Field `mipipll_ssc_gain` reader - "]
pub type MIPIPLL_SSC_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mipipll_ssc_gain` writer - "]
pub type MIPIPLL_SSC_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MIPI_PLL_CFG9_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_ssc_start_gate_en` reader - "]
pub type MIPIPLL_SSC_START_GATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_ssc_start_gate_en` writer - "]
pub type MIPIPLL_SSC_START_GATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG9_SPEC, bool, O>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_ssc_en(&self) -> MIPIPLL_SSC_EN_R {
        MIPIPLL_SSC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn mipipll_ssc_cnt(&self) -> MIPIPLL_SSC_CNT_R {
        MIPIPLL_SSC_CNT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn mipipll_ssc_gain(&self) -> MIPIPLL_SSC_GAIN_R {
        MIPIPLL_SSC_GAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mipipll_ssc_start_gate_en(&self) -> MIPIPLL_SSC_START_GATE_EN_R {
        MIPIPLL_SSC_START_GATE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserved_17_31(&self) -> RESERVED_17_31_R {
        RESERVED_17_31_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_ssc_en(&mut self) -> MIPIPLL_SSC_EN_W<0> {
        MIPIPLL_SSC_EN_W::new(self)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_ssc_cnt(&mut self) -> MIPIPLL_SSC_CNT_W<4> {
        MIPIPLL_SSC_CNT_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_ssc_gain(&mut self) -> MIPIPLL_SSC_GAIN_W<12> {
        MIPIPLL_SSC_GAIN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_ssc_start_gate_en(&mut self) -> MIPIPLL_SSC_START_GATE_EN_W<16> {
        MIPIPLL_SSC_START_GATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mipi_pll_cfg9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_pll_cfg9](index.html) module"]
pub struct MIPI_PLL_CFG9_SPEC;
impl crate::RegisterSpec for MIPI_PLL_CFG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_pll_cfg9::R](R) reader structure"]
impl crate::Readable for MIPI_PLL_CFG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_pll_cfg9::W](W) writer structure"]
impl crate::Writable for MIPI_PLL_CFG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mipi_pll_cfg9 to value 0x0001_5640"]
impl crate::Resettable for MIPI_PLL_CFG9_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_5640;
}
