#[doc = "Register `sd_adma_sys_addr_1` reader"]
pub struct R(crate::R<SD_ADMA_SYS_ADDR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_ADMA_SYS_ADDR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_ADMA_SYS_ADDR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_ADMA_SYS_ADDR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_adma_sys_addr_1` writer"]
pub struct W(crate::W<SD_ADMA_SYS_ADDR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_ADMA_SYS_ADDR_1_SPEC>;
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
impl From<crate::W<SD_ADMA_SYS_ADDR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_ADMA_SYS_ADDR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adma_sys_addr` reader - "]
pub type ADMA_SYS_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adma_sys_addr` writer - "]
pub type ADMA_SYS_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_ADMA_SYS_ADDR_1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adma_sys_addr(&self) -> ADMA_SYS_ADDR_R {
        ADMA_SYS_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn adma_sys_addr(&mut self) -> ADMA_SYS_ADDR_W<0> {
        ADMA_SYS_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADMA System Address Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_adma_sys_addr_1](index.html) module"]
pub struct SD_ADMA_SYS_ADDR_1_SPEC;
impl crate::RegisterSpec for SD_ADMA_SYS_ADDR_1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_adma_sys_addr_1::R](R) reader structure"]
impl crate::Readable for SD_ADMA_SYS_ADDR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_adma_sys_addr_1::W](W) writer structure"]
impl crate::Writable for SD_ADMA_SYS_ADDR_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_adma_sys_addr_1 to value 0"]
impl crate::Resettable for SD_ADMA_SYS_ADDR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
