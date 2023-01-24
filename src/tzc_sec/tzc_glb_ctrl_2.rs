#[doc = "Register `tzc_glb_ctrl_2` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_glb_ctrl_2` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_2_SPEC>;
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
impl From<crate::W<TZC_GLB_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_GLB_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` reader - "]
pub type TZC_GLB_PWRON_RST_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` writer - "]
pub type TZC_GLB_PWRON_RST_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` reader - "]
pub type TZC_GLB_CPU_RESET_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` writer - "]
pub type TZC_GLB_CPU_RESET_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` reader - "]
pub type TZC_GLB_SYS_RESET_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` writer - "]
pub type TZC_GLB_SYS_RESET_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` reader - "]
pub type TZC_GLB_CPU2_RESET_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` writer - "]
pub type TZC_GLB_CPU2_RESET_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_lock(&self) -> TZC_GLB_PWRON_RST_TZSID_LOCK_R {
        TZC_GLB_PWRON_RST_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_lock(&self) -> TZC_GLB_CPU_RESET_TZSID_LOCK_R {
        TZC_GLB_CPU_RESET_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_lock(&self) -> TZC_GLB_SYS_RESET_TZSID_LOCK_R {
        TZC_GLB_SYS_RESET_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(&self) -> TZC_GLB_CPU2_RESET_TZSID_LOCK_R {
        TZC_GLB_CPU2_RESET_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwron_rst_tzsid_lock(&mut self) -> TZC_GLB_PWRON_RST_TZSID_LOCK_W<0> {
        TZC_GLB_PWRON_RST_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu_reset_tzsid_lock(&mut self) -> TZC_GLB_CPU_RESET_TZSID_LOCK_W<1> {
        TZC_GLB_CPU_RESET_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sys_reset_tzsid_lock(&mut self) -> TZC_GLB_SYS_RESET_TZSID_LOCK_W<2> {
        TZC_GLB_SYS_RESET_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(&mut self) -> TZC_GLB_CPU2_RESET_TZSID_LOCK_W<3> {
        TZC_GLB_CPU2_RESET_TZSID_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_glb_ctrl_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_2](index.html) module"]
pub struct TZC_GLB_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_2::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_2::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_2 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
