#[doc = "Register `sd_capabilities_4` reader"]
pub struct R(crate::R<SD_CAPABILITIES_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CAPABILITIES_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CAPABILITIES_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CAPABILITIES_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_capabilities_4` writer"]
pub struct W(crate::W<SD_CAPABILITIES_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CAPABILITIES_4_SPEC>;
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
impl From<crate::W<SD_CAPABILITIES_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CAPABILITIES_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk_multiplier` reader - "]
pub type CLK_MULTIPLIER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_15_8` reader - "]
pub type RESERVED_15_8_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clk_multiplier(&self) -> CLK_MULTIPLIER_R {
        CLK_MULTIPLIER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reserved_15_8(&self) -> RESERVED_15_8_R {
        RESERVED_15_8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capabilities Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_capabilities_4](index.html) module"]
pub struct SD_CAPABILITIES_4_SPEC;
impl crate::RegisterSpec for SD_CAPABILITIES_4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_capabilities_4::R](R) reader structure"]
impl crate::Readable for SD_CAPABILITIES_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_capabilities_4::W](W) writer structure"]
impl crate::Writable for SD_CAPABILITIES_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_capabilities_4 to value 0"]
impl crate::Resettable for SD_CAPABILITIES_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
