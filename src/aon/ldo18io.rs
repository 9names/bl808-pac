#[doc = "Register `ldo18io` reader"]
pub struct R(crate::R<LDO18IO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO18IO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO18IO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO18IO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo18io` writer"]
pub struct W(crate::W<LDO18IO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO18IO_SPEC>;
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
impl From<crate::W<LDO18IO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO18IO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0` reader - "]
pub type RESERVED_0_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_bypass_iso_aon` reader - "]
pub type LDO18IO_BYPASS_ISO_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_bypass_iso_aon` writer - "]
pub type LDO18IO_BYPASS_ISO_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
#[doc = "Field `ldo18io_pulldown_aon` reader - "]
pub type LDO18IO_PULLDOWN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_pulldown_aon` writer - "]
pub type LDO18IO_PULLDOWN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> RESERVED_0_R {
        RESERVED_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18io_bypass_iso_aon(&self) -> LDO18IO_BYPASS_ISO_AON_R {
        LDO18IO_BYPASS_ISO_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18io_pulldown_aon(&self) -> LDO18IO_PULLDOWN_AON_R {
        LDO18IO_PULLDOWN_AON_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_bypass_iso_aon(&mut self) -> LDO18IO_BYPASS_ISO_AON_W<1> {
        LDO18IO_BYPASS_ISO_AON_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_pulldown_aon(&mut self) -> LDO18IO_PULLDOWN_AON_W<2> {
        LDO18IO_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo18io\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo18io](index.html) module"]
pub struct LDO18IO_SPEC;
impl crate::RegisterSpec for LDO18IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo18io::R](R) reader structure"]
impl crate::Readable for LDO18IO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo18io::W](W) writer structure"]
impl crate::Writable for LDO18IO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo18io to value 0"]
impl crate::Resettable for LDO18IO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
