#[doc = "Register `tzc_cfg0` reader"]
pub struct R(crate::R<TZC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_cfg0` writer"]
pub struct W(crate::W<TZC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_CFG0_SPEC>;
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
impl From<crate::W<TZC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_11` reader - "]
pub type RESERVED_0_11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_glb_pwron_rst_lock` reader - "]
pub type TZC_GLB_PWRON_RST_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_cpu_reset_lock` reader - "]
pub type TZC_GLB_CPU_RESET_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_sys_reset_lock` reader - "]
pub type TZC_GLB_SYS_RESET_LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reserved_0_11(&self) -> RESERVED_0_11_R {
        RESERVED_0_11_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_lock(&self) -> TZC_GLB_PWRON_RST_LOCK_R {
        TZC_GLB_PWRON_RST_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_lock(&self) -> TZC_GLB_CPU_RESET_LOCK_R {
        TZC_GLB_CPU_RESET_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_lock(&self) -> TZC_GLB_SYS_RESET_LOCK_R {
        TZC_GLB_SYS_RESET_LOCK_R::new(((self.bits >> 14) & 1) != 0)
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
#[doc = "tzc_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_cfg0](index.html) module"]
pub struct TZC_CFG0_SPEC;
impl crate::RegisterSpec for TZC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_cfg0::R](R) reader structure"]
impl crate::Readable for TZC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_cfg0::W](W) writer structure"]
impl crate::Writable for TZC_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_cfg0 to value 0"]
impl crate::Resettable for TZC_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
