#[doc = "Register `cpu_pll_cfg10` reader"]
pub struct R(crate::R<CPU_PLL_CFG10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg10` writer"]
pub struct W(crate::W<CPU_PLL_CFG10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG10_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_ssc_en` reader - "]
pub type CPUPLL_SSC_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_ssc_en` writer - "]
pub type CPUPLL_SSC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_ssc_cnt` reader - "]
pub type CPUPLL_SSC_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_ssc_cnt` writer - "]
pub type CPUPLL_SSC_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG10_SPEC, u8, u8, 8, O>;
#[doc = "Field `cpupll_ssc_gain` reader - "]
pub type CPUPLL_SSC_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_ssc_gain` writer - "]
pub type CPUPLL_SSC_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG10_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_ssc_start_gate_en` reader - "]
pub type CPUPLL_SSC_START_GATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_ssc_start_gate_en` writer - "]
pub type CPUPLL_SSC_START_GATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `reserved_17_19` reader - "]
pub type RESERVED_17_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_ssc_start` reader - "]
pub type CPUPLL_SSC_START_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_ssc_start` writer - "]
pub type CPUPLL_SSC_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG10_SPEC, bool, O>;
#[doc = "Field `reserved_21_31` reader - "]
pub type RESERVED_21_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_ssc_en(&self) -> CPUPLL_SSC_EN_R {
        CPUPLL_SSC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn cpupll_ssc_cnt(&self) -> CPUPLL_SSC_CNT_R {
        CPUPLL_SSC_CNT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cpupll_ssc_gain(&self) -> CPUPLL_SSC_GAIN_R {
        CPUPLL_SSC_GAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cpupll_ssc_start_gate_en(&self) -> CPUPLL_SSC_START_GATE_EN_R {
        CPUPLL_SSC_START_GATE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn reserved_17_19(&self) -> RESERVED_17_19_R {
        RESERVED_17_19_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cpupll_ssc_start(&self) -> CPUPLL_SSC_START_R {
        CPUPLL_SSC_START_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn reserved_21_31(&self) -> RESERVED_21_31_R {
        RESERVED_21_31_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_ssc_en(&mut self) -> CPUPLL_SSC_EN_W<0> {
        CPUPLL_SSC_EN_W::new(self)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_ssc_cnt(&mut self) -> CPUPLL_SSC_CNT_W<4> {
        CPUPLL_SSC_CNT_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_ssc_gain(&mut self) -> CPUPLL_SSC_GAIN_W<12> {
        CPUPLL_SSC_GAIN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_ssc_start_gate_en(&mut self) -> CPUPLL_SSC_START_GATE_EN_W<16> {
        CPUPLL_SSC_START_GATE_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_ssc_start(&mut self) -> CPUPLL_SSC_START_W<20> {
        CPUPLL_SSC_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg10](index.html) module"]
pub struct CPU_PLL_CFG10_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg10::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg10::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg10 to value 0x0010_4640"]
impl crate::Resettable for CPU_PLL_CFG10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_4640;
}
