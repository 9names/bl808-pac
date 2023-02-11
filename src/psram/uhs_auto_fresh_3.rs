#[doc = "Register `uhs_auto_fresh_3` reader"]
pub struct R(crate::R<UHS_AUTO_FRESH_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_AUTO_FRESH_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_AUTO_FRESH_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_AUTO_FRESH_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_auto_fresh_3` writer"]
pub struct W(crate::W<UHS_AUTO_FRESH_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_AUTO_FRESH_3_SPEC>;
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
impl From<crate::W<UHS_AUTO_FRESH_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_AUTO_FRESH_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_auto_ref_thre` reader - "]
pub type REG_AUTO_REF_THRE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_auto_ref_thre` writer - "]
pub type REG_AUTO_REF_THRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_AUTO_FRESH_3_SPEC, u16, u16, 12, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `auto_refresh_level` reader - "]
pub type AUTO_REFRESH_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_auto_ref_thre(&self) -> REG_AUTO_REF_THRE_R {
        REG_AUTO_REF_THRE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn auto_refresh_level(&self) -> AUTO_REFRESH_LEVEL_R {
        AUTO_REFRESH_LEVEL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_auto_ref_thre(&mut self) -> REG_AUTO_REF_THRE_W<0> {
        REG_AUTO_REF_THRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_auto_fresh_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_auto_fresh_3](index.html) module"]
pub struct UHS_AUTO_FRESH_3_SPEC;
impl crate::RegisterSpec for UHS_AUTO_FRESH_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_auto_fresh_3::R](R) reader structure"]
impl crate::Readable for UHS_AUTO_FRESH_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_auto_fresh_3::W](W) writer structure"]
impl crate::Writable for UHS_AUTO_FRESH_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_auto_fresh_3 to value 0"]
impl crate::Resettable for UHS_AUTO_FRESH_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
