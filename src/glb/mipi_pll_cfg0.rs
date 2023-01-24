#[doc = "Register `mipi_pll_cfg0` reader"]
pub struct R(crate::R<MIPI_PLL_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_PLL_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_PLL_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_PLL_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mipi_pll_cfg0` writer"]
pub struct W(crate::W<MIPI_PLL_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_PLL_CFG0_SPEC>;
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
impl From<crate::W<MIPI_PLL_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_PLL_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mipipll_sdm_rstb` reader - "]
pub type MIPIPLL_SDM_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_sdm_rstb` writer - "]
pub type MIPIPLL_SDM_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_1` reader - "]
pub type RESERVED_1_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_fbdv_rstb` reader - "]
pub type MIPIPLL_FBDV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `mipipll_fbdv_rstb` writer - "]
pub type MIPIPLL_FBDV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_3_4` reader - "]
pub type RESERVED_3_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pu_mipipll_fbdv` reader - "]
pub type PU_MIPIPLL_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `pu_mipipll_fbdv` writer - "]
pub type PU_MIPIPLL_FBDV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pu_mipipll_cp` reader - "]
pub type PU_MIPIPLL_CP_R = crate::BitReader<bool>;
#[doc = "Field `pu_mipipll_cp` writer - "]
pub type PU_MIPIPLL_CP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_mipipll_sfreg` reader - "]
pub type PU_MIPIPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `pu_mipipll_sfreg` writer - "]
pub type PU_MIPIPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_mipipll` reader - "]
pub type PU_MIPIPLL_R = crate::BitReader<bool>;
#[doc = "Field `pu_mipipll` writer - "]
pub type PU_MIPIPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sdm_rstb(&self) -> MIPIPLL_SDM_RSTB_R {
        MIPIPLL_SDM_RSTB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reserved_1(&self) -> RESERVED_1_R {
        RESERVED_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mipipll_fbdv_rstb(&self) -> MIPIPLL_FBDV_RSTB_R {
        MIPIPLL_FBDV_RSTB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn reserved_3_4(&self) -> RESERVED_3_4_R {
        RESERVED_3_4_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_mipipll_fbdv(&self) -> PU_MIPIPLL_FBDV_R {
        PU_MIPIPLL_FBDV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_mipipll_cp(&self) -> PU_MIPIPLL_CP_R {
        PU_MIPIPLL_CP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_mipipll_sfreg(&self) -> PU_MIPIPLL_SFREG_R {
        PU_MIPIPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_mipipll(&self) -> PU_MIPIPLL_R {
        PU_MIPIPLL_R::new(((self.bits >> 10) & 1) != 0)
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
    pub fn mipipll_sdm_rstb(&mut self) -> MIPIPLL_SDM_RSTB_W<0> {
        MIPIPLL_SDM_RSTB_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn mipipll_fbdv_rstb(&mut self) -> MIPIPLL_FBDV_RSTB_W<2> {
        MIPIPLL_FBDV_RSTB_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_mipipll_fbdv(&mut self) -> PU_MIPIPLL_FBDV_W<5> {
        PU_MIPIPLL_FBDV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_mipipll_cp(&mut self) -> PU_MIPIPLL_CP_W<8> {
        PU_MIPIPLL_CP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_mipipll_sfreg(&mut self) -> PU_MIPIPLL_SFREG_W<9> {
        PU_MIPIPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_mipipll(&mut self) -> PU_MIPIPLL_W<10> {
        PU_MIPIPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mipi_pll_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_pll_cfg0](index.html) module"]
pub struct MIPI_PLL_CFG0_SPEC;
impl crate::RegisterSpec for MIPI_PLL_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_pll_cfg0::R](R) reader structure"]
impl crate::Readable for MIPI_PLL_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_pll_cfg0::W](W) writer structure"]
impl crate::Writable for MIPI_PLL_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mipi_pll_cfg0 to value 0x0125"]
impl crate::Resettable for MIPI_PLL_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0125;
}