#[doc = "Register `bg_sys_top` reader"]
pub struct R(crate::R<BG_SYS_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_SYS_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_SYS_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_SYS_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bg_sys_top` writer"]
pub struct W(crate::W<BG_SYS_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_SYS_TOP_SPEC>;
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
impl From<crate::W<BG_SYS_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_SYS_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_bg_sys_aon` reader - "]
pub type PU_BG_SYS_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_bg_sys_aon` writer - "]
pub type PU_BG_SYS_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BG_SYS_TOP_SPEC, bool, O>;
#[doc = "Field `istart_ctrl_aon` reader - "]
pub type ISTART_CTRL_AON_R = crate::BitReader<bool>;
#[doc = "Field `istart_ctrl_aon` writer - "]
pub type ISTART_CTRL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BG_SYS_TOP_SPEC, bool, O>;
#[doc = "Field `reserved_2_31` reader - "]
pub type RESERVED_2_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PU_BG_SYS_AON_R {
        PU_BG_SYS_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn istart_ctrl_aon(&self) -> ISTART_CTRL_AON_R {
        ISTART_CTRL_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn reserved_2_31(&self) -> RESERVED_2_31_R {
        RESERVED_2_31_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_bg_sys_aon(&mut self) -> PU_BG_SYS_AON_W<0> {
        PU_BG_SYS_AON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn istart_ctrl_aon(&mut self) -> ISTART_CTRL_AON_W<1> {
        ISTART_CTRL_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bg_sys_top\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_sys_top](index.html) module"]
pub struct BG_SYS_TOP_SPEC;
impl crate::RegisterSpec for BG_SYS_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_sys_top::R](R) reader structure"]
impl crate::Readable for BG_SYS_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_sys_top::W](W) writer structure"]
impl crate::Writable for BG_SYS_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bg_sys_top to value 0x03"]
impl crate::Resettable for BG_SYS_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
