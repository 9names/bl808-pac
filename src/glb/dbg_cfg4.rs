#[doc = "Register `dbg_cfg4` reader"]
pub struct R(crate::R<DBG_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dbg_cfg4` writer"]
pub struct W(crate::W<DBG_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_CFG4_SPEC>;
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
impl From<crate::W<DBG_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `debug_oe` reader - "]
pub type DEBUG_OE_R = crate::BitReader<bool>;
#[doc = "Field `debug_oe` writer - "]
pub type DEBUG_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_CFG4_SPEC, bool, O>;
#[doc = "Field `debug_i` reader - "]
pub type DEBUG_I_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DEBUG_OE_R {
        DEBUG_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DEBUG_I_R {
        DEBUG_I_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn debug_oe(&mut self) -> DEBUG_OE_W<0> {
        DEBUG_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dbg_cfg4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_cfg4](index.html) module"]
pub struct DBG_CFG4_SPEC;
impl crate::RegisterSpec for DBG_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_cfg4::R](R) reader structure"]
impl crate::Readable for DBG_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_cfg4::W](W) writer structure"]
impl crate::Writable for DBG_CFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbg_cfg4 to value 0"]
impl crate::Resettable for DBG_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
