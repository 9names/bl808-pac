#[doc = "Register `cpu_pll_cfg0` reader"]
pub struct R(crate::R<CPU_PLL_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg0` writer"]
pub struct W(crate::W<CPU_PLL_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG0_SPEC>;
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
impl From<crate::W<CPU_PLL_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_sdm_rstb` reader - "]
pub type CPUPLL_SDM_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_sdm_rstb` writer - "]
pub type CPUPLL_SDM_RSTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `cpupll_postdiv_rstb` reader - "]
pub type CPUPLL_POSTDIV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_postdiv_rstb` writer - "]
pub type CPUPLL_POSTDIV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `cpupll_fbdv_rstb` reader - "]
pub type CPUPLL_FBDV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_fbdv_rstb` writer - "]
pub type CPUPLL_FBDV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `cpupll_refdiv_rstb` reader - "]
pub type CPUPLL_REFDIV_RSTB_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_refdiv_rstb` writer - "]
pub type CPUPLL_REFDIV_RSTB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_postdiv` reader - "]
pub type PU_CPUPLL_POSTDIV_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_postdiv` writer - "]
pub type PU_CPUPLL_POSTDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_fbdv` reader - "]
pub type PU_CPUPLL_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_fbdv` writer - "]
pub type PU_CPUPLL_FBDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_clamp_op` reader - "]
pub type PU_CPUPLL_CLAMP_OP_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_clamp_op` writer - "]
pub type PU_CPUPLL_CLAMP_OP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_pfd` reader - "]
pub type PU_CPUPLL_PFD_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_pfd` writer - "]
pub type PU_CPUPLL_PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_cp` reader - "]
pub type PU_CPUPLL_CP_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_cp` writer - "]
pub type PU_CPUPLL_CP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_sfreg` reader - "]
pub type PU_CPUPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_sfreg` writer - "]
pub type PU_CPUPLL_SFREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll` reader - "]
pub type PU_CPUPLL_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll` writer - "]
pub type PU_CPUPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `pu_cpupll_clktree` reader - "]
pub type PU_CPUPLL_CLKTREE_R = crate::BitReader<bool>;
#[doc = "Field `pu_cpupll_clktree` writer - "]
pub type PU_CPUPLL_CLKTREE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PLL_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_12_31` reader - "]
pub type RESERVED_12_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sdm_rstb(&self) -> CPUPLL_SDM_RSTB_R {
        CPUPLL_SDM_RSTB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cpupll_postdiv_rstb(&self) -> CPUPLL_POSTDIV_RSTB_R {
        CPUPLL_POSTDIV_RSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpupll_fbdv_rstb(&self) -> CPUPLL_FBDV_RSTB_R {
        CPUPLL_FBDV_RSTB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cpupll_refdiv_rstb(&self) -> CPUPLL_REFDIV_RSTB_R {
        CPUPLL_REFDIV_RSTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_cpupll_postdiv(&self) -> PU_CPUPLL_POSTDIV_R {
        PU_CPUPLL_POSTDIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_cpupll_fbdv(&self) -> PU_CPUPLL_FBDV_R {
        PU_CPUPLL_FBDV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_cpupll_clamp_op(&self) -> PU_CPUPLL_CLAMP_OP_R {
        PU_CPUPLL_CLAMP_OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_cpupll_pfd(&self) -> PU_CPUPLL_PFD_R {
        PU_CPUPLL_PFD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_cpupll_cp(&self) -> PU_CPUPLL_CP_R {
        PU_CPUPLL_CP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_cpupll_sfreg(&self) -> PU_CPUPLL_SFREG_R {
        PU_CPUPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_cpupll(&self) -> PU_CPUPLL_R {
        PU_CPUPLL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_cpupll_clktree(&self) -> PU_CPUPLL_CLKTREE_R {
        PU_CPUPLL_CLKTREE_R::new(((self.bits >> 11) & 1) != 0)
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
    pub fn cpupll_sdm_rstb(&mut self) -> CPUPLL_SDM_RSTB_W<0> {
        CPUPLL_SDM_RSTB_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_postdiv_rstb(&mut self) -> CPUPLL_POSTDIV_RSTB_W<1> {
        CPUPLL_POSTDIV_RSTB_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_fbdv_rstb(&mut self) -> CPUPLL_FBDV_RSTB_W<2> {
        CPUPLL_FBDV_RSTB_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_refdiv_rstb(&mut self) -> CPUPLL_REFDIV_RSTB_W<3> {
        CPUPLL_REFDIV_RSTB_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_postdiv(&mut self) -> PU_CPUPLL_POSTDIV_W<4> {
        PU_CPUPLL_POSTDIV_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_fbdv(&mut self) -> PU_CPUPLL_FBDV_W<5> {
        PU_CPUPLL_FBDV_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_clamp_op(&mut self) -> PU_CPUPLL_CLAMP_OP_W<6> {
        PU_CPUPLL_CLAMP_OP_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_pfd(&mut self) -> PU_CPUPLL_PFD_W<7> {
        PU_CPUPLL_PFD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_cp(&mut self) -> PU_CPUPLL_CP_W<8> {
        PU_CPUPLL_CP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_sfreg(&mut self) -> PU_CPUPLL_SFREG_W<9> {
        PU_CPUPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll(&mut self) -> PU_CPUPLL_W<10> {
        PU_CPUPLL_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_cpupll_clktree(&mut self) -> PU_CPUPLL_CLKTREE_W<11> {
        PU_CPUPLL_CLKTREE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg0](index.html) module"]
pub struct CPU_PLL_CFG0_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg0::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg0::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg0 to value 0x09ef"]
impl crate::Resettable for CPU_PLL_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x09ef;
}
