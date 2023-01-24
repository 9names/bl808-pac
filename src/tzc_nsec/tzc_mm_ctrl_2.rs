#[doc = "Register `tzc_mm_ctrl_2` reader"]
pub struct R(crate::R<TZC_MM_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_ctrl_2` writer"]
pub struct W(crate::W<TZC_MM_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_CTRL_2_SPEC>;
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
impl From<crate::W<TZC_MM_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_mm_pwron_rst_tzsid_lock` reader - "]
pub type TZC_MM_PWRON_RST_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_lock` reader - "]
pub type TZC_MM_CPU0_RESET_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_sys_reset_tzsid_lock` reader - "]
pub type TZC_MM_SYS_RESET_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_cpu0_tzsid_lock` reader - "]
pub type TZC_MM_CPU0_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `reserved_4` reader - "]
pub type RESERVED_4_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_sram_tzsid_lock` reader - "]
pub type TZC_MM_SRAM_TZSID_LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_lock(&self) -> TZC_MM_PWRON_RST_TZSID_LOCK_R {
        TZC_MM_PWRON_RST_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_lock(&self) -> TZC_MM_CPU0_RESET_TZSID_LOCK_R {
        TZC_MM_CPU0_RESET_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_lock(&self) -> TZC_MM_SYS_RESET_TZSID_LOCK_R {
        TZC_MM_SYS_RESET_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_lock(&self) -> TZC_MM_CPU0_TZSID_LOCK_R {
        TZC_MM_CPU0_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&self) -> RESERVED_4_R {
        RESERVED_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_lock(&self) -> TZC_MM_SRAM_TZSID_LOCK_R {
        TZC_MM_SRAM_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_mm_ctrl_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_ctrl_2](index.html) module"]
pub struct TZC_MM_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_MM_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_ctrl_2::R](R) reader structure"]
impl crate::Readable for TZC_MM_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_ctrl_2::W](W) writer structure"]
impl crate::Writable for TZC_MM_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_ctrl_2 to value 0"]
impl crate::Resettable for TZC_MM_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
