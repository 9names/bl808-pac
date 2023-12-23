#[doc = "Register `wifi_pll_cfg0` reader"]
pub struct R(crate::R<WIFI_PLL_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg0` writer"]
pub struct W(crate::W<WIFI_PLL_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG0_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_sdm_rstb` reader - "]
pub type WIFIPLL_SDM_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_sdm_rstb` writer - "]
pub type WIFIPLL_SDM_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `wifipll_postdiv_rstb` reader - "]
pub type WIFIPLL_POSTDIV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_postdiv_rstb` writer - "]
pub type WIFIPLL_POSTDIV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `wifipll_fbdv_rstb` reader - "]
pub type WIFIPLL_FBDV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_fbdv_rstb` writer - "]
pub type WIFIPLL_FBDV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `wifipll_refdiv_rstb` reader - "]
pub type WIFIPLL_REFDIV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_refdiv_rstb` writer - "]
pub type WIFIPLL_REFDIV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_postdiv` reader - "]
pub type PU_WIFIPLL_POSTDIV_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_postdiv` writer - "]
pub type PU_WIFIPLL_POSTDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_fbdv` reader - "]
pub type PU_WIFIPLL_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_fbdv` writer - "]
pub type PU_WIFIPLL_FBDV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_clamp_op` reader - "]
pub type PU_WIFIPLL_CLAMP_OP_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_clamp_op` writer - "]
pub type PU_WIFIPLL_CLAMP_OP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_pfd` reader - "]
pub type PU_WIFIPLL_PFD_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_pfd` writer - "]
pub type PU_WIFIPLL_PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_cp` reader - "]
pub type PU_WIFIPLL_CP_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_cp` writer - "]
pub type PU_WIFIPLL_CP_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_sfreg` reader - "]
pub type PU_WIFIPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_sfreg` writer - "]
pub type PU_WIFIPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll` reader - "]
pub type PU_WIFIPLL_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll` writer - "]
pub type PU_WIFIPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_wifipll_clktree` reader - "]
pub type PU_WIFIPLL_CLKTREE_R = crate::BitReader<bool>;
#[doc = "Field `pu_wifipll_clktree` writer - "]
pub type PU_WIFIPLL_CLKTREE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_12_31` reader - "]
pub type RESERVED_12_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sdm_rstb(&self) -> WIFIPLL_SDM_RSTB_R {
        WIFIPLL_SDM_RSTB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_postdiv_rstb(&self) -> WIFIPLL_POSTDIV_RSTB_R {
        WIFIPLL_POSTDIV_RSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_fbdv_rstb(&self) -> WIFIPLL_FBDV_RSTB_R {
        WIFIPLL_FBDV_RSTB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_refdiv_rstb(&self) -> WIFIPLL_REFDIV_RSTB_R {
        WIFIPLL_REFDIV_RSTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_wifipll_postdiv(&self) -> PU_WIFIPLL_POSTDIV_R {
        PU_WIFIPLL_POSTDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_wifipll_fbdv(&self) -> PU_WIFIPLL_FBDV_R {
        PU_WIFIPLL_FBDV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_wifipll_clamp_op(&self) -> PU_WIFIPLL_CLAMP_OP_R {
        PU_WIFIPLL_CLAMP_OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_wifipll_pfd(&self) -> PU_WIFIPLL_PFD_R {
        PU_WIFIPLL_PFD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_wifipll_cp(&self) -> PU_WIFIPLL_CP_R {
        PU_WIFIPLL_CP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_wifipll_sfreg(&self) -> PU_WIFIPLL_SFREG_R {
        PU_WIFIPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_wifipll(&self) -> PU_WIFIPLL_R {
        PU_WIFIPLL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_wifipll_clktree(&self) -> PU_WIFIPLL_CLKTREE_R {
        PU_WIFIPLL_CLKTREE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn reserved_12_31(&self) -> RESERVED_12_31_R {
        RESERVED_12_31_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_rstb(&mut self) -> WIFIPLL_SDM_RSTB_W<0> {
        WIFIPLL_SDM_RSTB_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_postdiv_rstb(&mut self) -> WIFIPLL_POSTDIV_RSTB_W<1> {
        WIFIPLL_POSTDIV_RSTB_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_fbdv_rstb(&mut self) -> WIFIPLL_FBDV_RSTB_W<2> {
        WIFIPLL_FBDV_RSTB_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_refdiv_rstb(&mut self) -> WIFIPLL_REFDIV_RSTB_W<3> {
        WIFIPLL_REFDIV_RSTB_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_postdiv(&mut self) -> PU_WIFIPLL_POSTDIV_W<4> {
        PU_WIFIPLL_POSTDIV_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_fbdv(&mut self) -> PU_WIFIPLL_FBDV_W<5> {
        PU_WIFIPLL_FBDV_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_clamp_op(&mut self) -> PU_WIFIPLL_CLAMP_OP_W<6> {
        PU_WIFIPLL_CLAMP_OP_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_pfd(&mut self) -> PU_WIFIPLL_PFD_W<7> {
        PU_WIFIPLL_PFD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_cp(&mut self) -> PU_WIFIPLL_CP_W<8> {
        PU_WIFIPLL_CP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_sfreg(&mut self) -> PU_WIFIPLL_SFREG_W<9> {
        PU_WIFIPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll(&mut self) -> PU_WIFIPLL_W<10> {
        PU_WIFIPLL_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_wifipll_clktree(&mut self) -> PU_WIFIPLL_CLKTREE_W<11> {
        PU_WIFIPLL_CLKTREE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg0](index.html) module"]
pub struct WIFI_PLL_CFG0_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg0::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg0::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg0 to value 0x09ef"]
impl crate::Resettable for WIFI_PLL_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x09ef;
}
