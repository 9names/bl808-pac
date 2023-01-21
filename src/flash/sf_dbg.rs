#[doc = "Register `sf_dbg` reader"]
pub struct R(crate::R<SF_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_dbg` writer"]
pub struct W(crate::W<SF_DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_DBG_SPEC>;
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
impl From<crate::W<SF_DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_DBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_autoload_st` reader - "]
pub type SF_AUTOLOAD_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf_autoload_st_done` reader - "]
pub type SF_AUTOLOAD_ST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_6_31` reader - "]
pub type RESERVED_6_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sf_autoload_st(&self) -> SF_AUTOLOAD_ST_R {
        SF_AUTOLOAD_ST_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_autoload_st_done(&self) -> SF_AUTOLOAD_ST_DONE_R {
        SF_AUTOLOAD_ST_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved_6_31(&self) -> RESERVED_6_31_R {
        RESERVED_6_31_R::new((self.bits >> 6) & 0x03ff_ffff)
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
#[doc = "sf_dbg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_dbg](index.html) module"]
pub struct SF_DBG_SPEC;
impl crate::RegisterSpec for SF_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_dbg::R](R) reader structure"]
impl crate::Readable for SF_DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_dbg::W](W) writer structure"]
impl crate::Writable for SF_DBG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_dbg to value 0x01"]
impl crate::Resettable for SF_DBG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
