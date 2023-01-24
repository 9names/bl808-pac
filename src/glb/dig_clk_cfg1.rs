#[doc = "Register `dig_clk_cfg1` reader"]
pub struct R(crate::R<DIG_CLK_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_CLK_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_CLK_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_CLK_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dig_clk_cfg1` writer"]
pub struct W(crate::W<DIG_CLK_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_CLK_CFG1_SPEC>;
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
impl From<crate::W<DIG_CLK_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_CLK_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mm_muxpll_160m_sel` reader - "]
pub type REG_MM_MUXPLL_160M_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_mm_muxpll_160m_sel` writer - "]
pub type REG_MM_MUXPLL_160M_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_mm_muxpll_240m_sel` reader - "]
pub type REG_MM_MUXPLL_240M_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_mm_muxpll_240m_sel` writer - "]
pub type REG_MM_MUXPLL_240M_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_mm_muxpll_320m_sel` reader - "]
pub type REG_MM_MUXPLL_320M_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_mm_muxpll_320m_sel` writer - "]
pub type REG_MM_MUXPLL_320M_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_3_7` reader - "]
pub type RESERVED_3_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_top_muxpll_80m_sel` reader - "]
pub type REG_TOP_MUXPLL_80M_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_top_muxpll_80m_sel` writer - "]
pub type REG_TOP_MUXPLL_80M_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mm_muxpll_160m_sel(&self) -> REG_MM_MUXPLL_160M_SEL_R {
        REG_MM_MUXPLL_160M_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mm_muxpll_240m_sel(&self) -> REG_MM_MUXPLL_240M_SEL_R {
        REG_MM_MUXPLL_240M_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_mm_muxpll_320m_sel(&self) -> REG_MM_MUXPLL_320M_SEL_R {
        REG_MM_MUXPLL_320M_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn reserved_3_7(&self) -> RESERVED_3_7_R {
        RESERVED_3_7_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_top_muxpll_80m_sel(&self) -> REG_TOP_MUXPLL_80M_SEL_R {
        REG_TOP_MUXPLL_80M_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mm_muxpll_160m_sel(&mut self) -> REG_MM_MUXPLL_160M_SEL_W<0> {
        REG_MM_MUXPLL_160M_SEL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mm_muxpll_240m_sel(&mut self) -> REG_MM_MUXPLL_240M_SEL_W<1> {
        REG_MM_MUXPLL_240M_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mm_muxpll_320m_sel(&mut self) -> REG_MM_MUXPLL_320M_SEL_W<2> {
        REG_MM_MUXPLL_320M_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_top_muxpll_80m_sel(&mut self) -> REG_TOP_MUXPLL_80M_SEL_W<8> {
        REG_TOP_MUXPLL_80M_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dig_clk_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_clk_cfg1](index.html) module"]
pub struct DIG_CLK_CFG1_SPEC;
impl crate::RegisterSpec for DIG_CLK_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_clk_cfg1::R](R) reader structure"]
impl crate::Readable for DIG_CLK_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_clk_cfg1::W](W) writer structure"]
impl crate::Writable for DIG_CLK_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dig_clk_cfg1 to value 0"]
impl crate::Resettable for DIG_CLK_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
