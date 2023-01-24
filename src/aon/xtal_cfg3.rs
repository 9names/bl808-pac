#[doc = "Register `xtal_cfg3` reader"]
pub struct R(crate::R<XTAL_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xtal_cfg3` writer"]
pub struct W(crate::W<XTAL_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_CFG3_SPEC>;
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
impl From<crate::W<XTAL_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_11` reader - "]
pub type RESERVED_0_11_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reserved_0_11(&self) -> RESERVED_0_11_R {
        RESERVED_0_11_R::new((self.bits & 0x0fff) as u16)
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
#[doc = "xtal_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_cfg3](index.html) module"]
pub struct XTAL_CFG3_SPEC;
impl crate::RegisterSpec for XTAL_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_cfg3::R](R) reader structure"]
impl crate::Readable for XTAL_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_cfg3::W](W) writer structure"]
impl crate::Writable for XTAL_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal_cfg3 to value 0"]
impl crate::Resettable for XTAL_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
