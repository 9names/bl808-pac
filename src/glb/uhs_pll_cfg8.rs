#[doc = "Register `uhs_pll_cfg8` reader"]
pub struct R(crate::R<UHS_PLL_CFG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PLL_CFG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PLL_CFG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PLL_CFG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_pll_cfg8` writer"]
pub struct W(crate::W<UHS_PLL_CFG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PLL_CFG8_SPEC>;
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
impl From<crate::W<UHS_PLL_CFG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PLL_CFG8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhspll_dc_tp_out_en` reader - "]
pub type UHSPLL_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dc_tp_out_en` writer - "]
pub type UHSPLL_DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `uhspll_ten` reader - "]
pub type UHSPLL_TEN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_ten` writer - "]
pub type UHSPLL_TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `uhspll_ten_sfreg` reader - "]
pub type UHSPLL_TEN_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_ten_sfreg` writer - "]
pub type UHSPLL_TEN_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dten_ckin` reader - "]
pub type UHSPLL_DTEN_CKIN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dten_ckin` writer - "]
pub type UHSPLL_DTEN_CKIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `uhspll_dten_fref` reader - "]
pub type UHSPLL_DTEN_FREF_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dten_fref` writer - "]
pub type UHSPLL_DTEN_FREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `uhspll_dten_fsdm` reader - "]
pub type UHSPLL_DTEN_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dten_fsdm` writer - "]
pub type UHSPLL_DTEN_FSDM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `uhspll_dten_pupll` reader - "]
pub type UHSPLL_DTEN_PUPLL_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dten_pupll` writer - "]
pub type UHSPLL_DTEN_PUPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `uhspll_dten_pll_locked` reader - "]
pub type UHSPLL_DTEN_PLL_LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dten_pll_locked` writer - "]
pub type UHSPLL_DTEN_PLL_LOCKED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `reserved_9` reader - "]
pub type RESERVED_9_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dtest_pull_down` reader - "]
pub type UHSPLL_DTEST_PULL_DOWN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_dtest_pull_down` writer - "]
pub type UHSPLL_DTEST_PULL_DOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_dc_tp_out_en(&self) -> UHSPLL_DC_TP_OUT_EN_R {
        UHSPLL_DC_TP_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhspll_ten(&self) -> UHSPLL_TEN_R {
        UHSPLL_TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhspll_ten_sfreg(&self) -> UHSPLL_TEN_SFREG_R {
        UHSPLL_TEN_SFREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhspll_dten_ckin(&self) -> UHSPLL_DTEN_CKIN_R {
        UHSPLL_DTEN_CKIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhspll_dten_fref(&self) -> UHSPLL_DTEN_FREF_R {
        UHSPLL_DTEN_FREF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhspll_dten_fsdm(&self) -> UHSPLL_DTEN_FSDM_R {
        UHSPLL_DTEN_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhspll_dten_pupll(&self) -> UHSPLL_DTEN_PUPLL_R {
        UHSPLL_DTEN_PUPLL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_dten_pll_locked(&self) -> UHSPLL_DTEN_PLL_LOCKED_R {
        UHSPLL_DTEN_PLL_LOCKED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reserved_9(&self) -> RESERVED_9_R {
        RESERVED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_dtest_pull_down(&self) -> UHSPLL_DTEST_PULL_DOWN_R {
        UHSPLL_DTEST_PULL_DOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dc_tp_out_en(&mut self) -> UHSPLL_DC_TP_OUT_EN_W<0> {
        UHSPLL_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_ten(&mut self) -> UHSPLL_TEN_W<1> {
        UHSPLL_TEN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_ten_sfreg(&mut self) -> UHSPLL_TEN_SFREG_W<2> {
        UHSPLL_TEN_SFREG_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dten_ckin(&mut self) -> UHSPLL_DTEN_CKIN_W<4> {
        UHSPLL_DTEN_CKIN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dten_fref(&mut self) -> UHSPLL_DTEN_FREF_W<5> {
        UHSPLL_DTEN_FREF_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dten_fsdm(&mut self) -> UHSPLL_DTEN_FSDM_W<6> {
        UHSPLL_DTEN_FSDM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dten_pupll(&mut self) -> UHSPLL_DTEN_PUPLL_W<7> {
        UHSPLL_DTEN_PUPLL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dten_pll_locked(&mut self) -> UHSPLL_DTEN_PLL_LOCKED_W<8> {
        UHSPLL_DTEN_PLL_LOCKED_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_dtest_pull_down(&mut self) -> UHSPLL_DTEST_PULL_DOWN_W<10> {
        UHSPLL_DTEST_PULL_DOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhs_pll_cfg8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_pll_cfg8](index.html) module"]
pub struct UHS_PLL_CFG8_SPEC;
impl crate::RegisterSpec for UHS_PLL_CFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_pll_cfg8::R](R) reader structure"]
impl crate::Readable for UHS_PLL_CFG8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_pll_cfg8::W](W) writer structure"]
impl crate::Writable for UHS_PLL_CFG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_pll_cfg8 to value 0x0400"]
impl crate::Resettable for UHS_PLL_CFG8_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
