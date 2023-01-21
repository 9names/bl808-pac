#[doc = "Register `debug_cfg1` reader"]
pub struct R(crate::R<DEBUG_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `debug_cfg1` writer"]
pub struct W(crate::W<DEBUG_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CFG1_SPEC>;
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
impl From<crate::W<DEBUG_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_19` reader - "]
pub type RESERVED_0_19_R = crate::FieldReader<u32, u32>;
#[doc = "Field `debug_ndreset_gate` reader - "]
pub type DEBUG_NDRESET_GATE_R = crate::BitReader<bool>;
#[doc = "Field `debug_ndreset_gate` writer - "]
pub type DEBUG_NDRESET_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEBUG_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_21_31` reader - "]
pub type RESERVED_21_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn reserved_0_19(&self) -> RESERVED_0_19_R {
        RESERVED_0_19_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DEBUG_NDRESET_GATE_R {
        DEBUG_NDRESET_GATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn reserved_21_31(&self) -> RESERVED_21_31_R {
        RESERVED_21_31_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ndreset_gate(&mut self) -> DEBUG_NDRESET_GATE_W<20> {
        DEBUG_NDRESET_GATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "debug_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_cfg1](index.html) module"]
pub struct DEBUG_CFG1_SPEC;
impl crate::RegisterSpec for DEBUG_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_cfg1::R](R) reader structure"]
impl crate::Readable for DEBUG_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_cfg1::W](W) writer structure"]
impl crate::Writable for DEBUG_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets debug_cfg1 to value 0"]
impl crate::Resettable for DEBUG_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
