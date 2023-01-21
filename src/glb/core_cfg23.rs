#[doc = "Register `core_cfg23` reader"]
pub struct R(crate::R<CORE_CFG23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_CFG23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_CFG23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_CFG23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_cfg23` writer"]
pub struct W(crate::W<CORE_CFG23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_CFG23_SPEC>;
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
impl From<crate::W<CORE_CFG23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_CFG23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_e902_int_en1` reader - "]
pub type REG_E902_INT_EN1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_e902_int_en1` writer - "]
pub type REG_E902_INT_EN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_CFG23_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_e902_int_en1(&self) -> REG_E902_INT_EN1_R {
        REG_E902_INT_EN1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_e902_int_en1(&mut self) -> REG_E902_INT_EN1_W<0> {
        REG_E902_INT_EN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core_cfg23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_cfg23](index.html) module"]
pub struct CORE_CFG23_SPEC;
impl crate::RegisterSpec for CORE_CFG23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_cfg23::R](R) reader structure"]
impl crate::Readable for CORE_CFG23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_cfg23::W](W) writer structure"]
impl crate::Writable for CORE_CFG23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core_cfg23 to value 0xffff_ffff"]
impl crate::Resettable for CORE_CFG23_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
